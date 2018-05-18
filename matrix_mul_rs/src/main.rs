extern crate ocl;
extern crate ocl_core;

#[macro_use]
mod gen_error;

use std::{env, process, fs::File, io::BufReader, io::prelude::*, cmp};
use ocl::{flags, Platform, Device, Context, Queue, Program, Buffer, Kernel, Event};
use gen_error::{GenResult, GenError};

const MAX_PRINT_ERRORS: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 7 {
        println!("Usage: ./matrix_mul_rs platform tile_size m n p device_gflops, where:");
        println!("    platform is the OpenCL platform used, e.g. \"Intel Gen OCL Driver\"");
        println!("    tile_size is the size of the tiles input matrices are split into during computation (matches the number of work items)");
        println!("    m-by-n specifies the dimensions of matrix A");
        println!("    n-by-p specifies the dimensions of matrix B");
        println!("    device_gflops is the max GFLOPS of the device, used for profiling");
        return;
    }

    let platform_name: String = args[1].to_owned();
    let (tile_size, m, n, p): (u32, u32, u32, u32) =
        (unwrap!(args[2].parse()), unwrap!(args[3].parse()), unwrap!(args[4].parse()), unwrap!(args[5].parse()));
    let device_max_gflops = unwrap!(args[6].parse::<f64>());

    let (device, context, queue) = unwrap!(init_ocl(platform_name));
    let (buffer_a, buffer_b, buffer_c, matrix_c_expected) = unwrap!(load_matrices(&queue, m, n, p));
    let max_work_group_size = unwrap!(device.max_wg_size()) as u32;

    /* wideloads.cl setup */
    let n_wide = ceil_divisible_by(n, tile_size);
    let p_wide = ceil_divisible_by(p, tile_size);
    let wide_buffer_a;
    let ref_wide_buffer_a = if n_wide != n {
        wide_buffer_a = unwrap!(run_pad_cols_kernel(&device, &context, &queue, &buffer_a, m, n, tile_size));
        &wide_buffer_a
    }
    else { &buffer_a };
    let wide_buffer_b;
    let ref_wide_buffer_b = if p_wide != p {
        wide_buffer_b = unwrap!(run_pad_cols_kernel(&device, &context, &queue, &buffer_b, n, p, tile_size));
        &wide_buffer_b
    }
    else { &buffer_b };

    for &src_filename in ["tiled.cl", "wideloads.cl"].iter() {
        let (kernel_name, _ext) = src_filename.split_at(src_filename.len() - 3);
        if kernel_name == "wideloads" && tile_size % 4 != 0 {
            println!("===\ntile_size is not divisible by 4; skipping wideloads");
            continue;
        }
        println!("===\nRunning {}", kernel_name);

        let mut global_size = [ceil_divisible_by(m, tile_size), ceil_divisible_by(n, tile_size)];
        let mut local_size = [tile_size, tile_size];
        if kernel_name == "wideloads" { global_size[1] /= 4; local_size[1] /= 4; }

        if local_size[0] * local_size[1] > max_work_group_size {
            println!("Local work size exceeds device limits; skipping this kernel.");
            println!("You may want to choose a smaller value for tile_size");
            continue;
        }

        println!("Global work size: {} x {}, local work size: {} x {}", global_size[0], global_size[1], local_size[0], local_size[1]);
        let program = unwrap!(build_ocl_program(&device, &context, format!("#define TILE_SIZE {}", tile_size), src_filename));

        let kernel = unwrap!(Kernel::builder()
            .queue(queue.clone())
            .program(&program).name(kernel_name)
            .arg(if kernel_name == "wideloads" { ref_wide_buffer_a } else { &buffer_a })
            .arg(if kernel_name == "wideloads" { ref_wide_buffer_b } else { &buffer_b })
            .arg(&buffer_c).arg(m).arg(n).arg(p)
            .build());

        let mut exec_event = Event::empty();

        unsafe {
            unwrap!(kernel.cmd()
                .queue(&queue)
                .global_work_size(global_size)
                .local_work_size(local_size)
                .enew(&mut exec_event)
                .enq());
        }

        unwrap!(exec_event.wait_for());

        let mut matrix_c_actual = vec![0.0f32; (m * p) as usize];
        unwrap!(buffer_c.cmd().queue(&queue).offset(0).read(&mut matrix_c_actual).enq());

        verify_results(&matrix_c_expected, &matrix_c_actual, p);
        let (kernel_exec_time_ns, total_time_ns) = unwrap!(get_execution_time_ns(&exec_event));
        println!("Execution time is {} [ms]", total_time_ns as f64 / 1_000_000.0);
        let total_flops_theory = ((2 * n - 1) * m * p) as u64;
        let exec_gflops = (total_flops_theory as f64 / kernel_exec_time_ns as f64) / /* nano */ 1_000_000_000.0 * /* giga */ 1_000_000_000.0;
        println!("GFLOPS: {:.3}, efficiency: {:.1}%", exec_gflops, exec_gflops / device_max_gflops * 100.0);
    }
}

fn run_pad_cols_kernel(dev: &Device, ctx: &Context, queue: &Queue, buffer_a: &Buffer<f32>, m: u32, n: u32, tile_size: u32) -> GenResult<Buffer<f32>> {
    println!("===\nRunning pad_cols.cl");
    let (m_wide, n_wide) = (ceil_divisible_by(m, tile_size), ceil_divisible_by(n, tile_size));
    let buffer_a_wide = Buffer::<f32>::builder().queue(queue.clone()).flags(flags::MEM_READ_WRITE).len(m * n_wide).build()?;
    let program = build_ocl_program(dev, ctx, format!("#define TILE_SIZE {}", tile_size), "pad_cols.cl")?;

    let max_local_size = (dev.max_wg_size()? as f32).sqrt() as u32;

    let kernel = Kernel::builder()
        .queue(queue.clone())
        .program(&program).name("pad_cols")
        .arg(buffer_a).arg(&buffer_a_wide).arg(m).arg(n)
        .build()?;

    let mut exec_event = Event::empty();

    unsafe {
        kernel.cmd()
            .queue(&queue)
            .global_work_size([m_wide, n_wide])
            .local_work_size([cmp::min(max_local_size, gcd(m_wide, tile_size)),
                              cmp::min(max_local_size, gcd(n_wide, tile_size))])
            .enew(&mut exec_event)
            .enq()?;
    }

    exec_event.wait_for()?;
    let (_, total_exec_time) = get_execution_time_ns(&exec_event)?;
    println!("Execution time is {} [ms]",total_exec_time as f64 / 1000000.0);

    Ok(buffer_a_wide)
}

fn ceil_divisible_by(n: u32, by: u32) -> u32 {
    ((n as f32 / by as f32).ceil() as u32) * by
}

fn gcd(a: u32, b: u32) -> u32 {
    let (mut a, mut b, mut rem) = (a, b, 0);
    while b > 0 {
        rem = a % b;
        a = b;
        b = rem;
    }
    a
}

fn get_execution_time_ns(event: &Event) -> GenResult<(u64, u64)> {
    use ocl::enums::{ProfilingInfo, ProfilingInfoResult::{Queued, Start, End}};

    if let (Queued(time_queued), Start(time_start), End(time_end)) =
        (event.profiling_info(ProfilingInfo::Queued)?, event.profiling_info(ProfilingInfo::Start)?, event.profiling_info(ProfilingInfo::End)?) {
        Ok((time_end - time_start, time_end - time_queued))
    }
    else {
        gen_error_format!("Unable to obtain kernel profiling info")
    }
}

fn verify_results(matrix_c_expected: &Vec<f32>, matrix_c_actual: &Vec<f32>, cols: u32) {
    let mut errors_encountered = 0;
    let matrix_iter = matrix_c_expected.iter().zip(matrix_c_actual.iter());

    for (i, (expected, actual)) in matrix_iter.enumerate() {
        /* TODO: implement a proper comparison (see https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition) */
        if (expected - actual).abs() > 0.02 {
            errors_encountered += 1;
            if errors_encountered < MAX_PRINT_ERRORS {
                println!("Row {}, col {}: expected {:.8}, got {:.8}", i as u32 / cols, i as u32 % cols, expected, actual);
            }
        }
    }

    if errors_encountered > MAX_PRINT_ERRORS {
        println!("...\n({} errors omitted)", errors_encountered - MAX_PRINT_ERRORS);
    }
    else if errors_encountered == 0 {
        println!("Result verified, no errors found")
    }
}

fn load_matrices(queue: &Queue, m: u32, n: u32, p: u32) -> GenResult<(Buffer<f32>, Buffer<f32>, Buffer<f32>, Vec<f32>)> {
    let matrix_a = read_matrix("matrix_a", m * n)?;
    let matrix_b = read_matrix("matrix_b", n * p)?;
    let matrix_c = read_matrix("matrix_c", m * p)?;

    let buffer_a = Buffer::<f32>::builder().queue(queue.clone()).flags(flags::MEM_READ_ONLY).len(m * n).build()?;
    let buffer_b = Buffer::<f32>::builder().queue(queue.clone()).flags(flags::MEM_READ_ONLY).len(n * p).build()?;
    let buffer_c = Buffer::<f32>::builder().queue(queue.clone()).flags(flags::MEM_WRITE_ONLY).len(m * p).build()?;

    buffer_a.cmd().queue(&queue).offset(0).write(&matrix_a).enq()?;
    buffer_b.cmd().queue(&queue).offset(0).write(&matrix_b).enq()?;

    Ok((buffer_a, buffer_b, buffer_c, matrix_c))
}

fn read_matrix(filename: &str, size: u32) -> GenResult<Vec<f32>> {
    BufReader::new(open_file(filename)?)
        .lines().into_iter()
        .map(|line| { with_gen_error!(line).and_then(|s| with_gen_error!(s.parse())) })
        .collect::<GenResult<Vec<f32>>>()
        .and_then(|vec| {
            if vec.len() != size as usize {
                gen_error_format!("Matrix read from {} has {} elements; {} expected.", filename, vec.len(), size)
            }
            else { Ok(vec) }
        })
}

fn open_file(filename: &str) -> GenResult<File> {
    File::open(filename).or(gen_error_format!("Unable to open {} for reading", filename))
}

fn init_ocl(platform_name: String) -> GenResult<(Device, Context, Queue)> {
    use ocl::flags::CommandQueueProperties as QueueProp;

    let platforms = Platform::list();
    let platform = platforms.iter()
        .find(|&&p| p.name().map(|s| s == platform_name).unwrap_or(false))
        .ok_or("The requested platform could not be found")?;
        
    let device = Device::first(platform)?;
    let context = Context::builder().platform(*platform).devices(device.clone()).build()?;
    let queue = Queue::new(&context, device, Some(QueueProp::new().profiling()))?;

    Ok((device, context, queue))
}

fn build_ocl_program(dev: &Device, ctx: &Context, kernel_defs: String, src_filename: &str) -> GenResult<Program> {
    let mut src_file_contents = String::new();
    open_file(src_filename)?.read_to_string(&mut src_file_contents)?;
    let src = kernel_defs + "\n" + &src_file_contents;

    with_gen_error!(Program::builder().devices(dev.clone()).src(src.clone()).build(&ctx))
}
