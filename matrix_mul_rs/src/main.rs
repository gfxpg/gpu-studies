extern crate ocl;
extern crate ocl_core;

#[macro_use]
mod gen_error;

use std::{env, process, fs::File, io::BufReader, io::prelude::*};
use ocl::{flags, Platform, Device, Context, Queue, Program, Buffer, Kernel, Event};
use gen_error::{GenResult, GenError};

const MAX_PRINT_ERRORS: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 6 {
        println!("Usage: ./matrix_mul_rs platform workitems m n p, where:");
        println!("    platform is the OpenCL platform used, e.g. \"Intel Gen OCL Driver\"");
        println!("    workitems is the number of work items (in each dimension) used for computation");
        println!("    m-by-n specifies the dimensions of matrix A");
        println!("    n-by-p specifies the dimensions of matrix B");
        return;
    }

    let platform_name: String = args[1].to_owned();
    let requested_work_items: u32 = unwrap!(args[2].parse());
    let (m, n, p): (u32, u32, u32) = (unwrap!(args[3].parse()), unwrap!(args[4].parse()), unwrap!(args[5].parse()));
    let (device, context, queue) = unwrap!(init_ocl(platform_name));

    let matrix_a = unwrap!(read_matrix("matrix_a", m * n));
    let matrix_b = unwrap!(read_matrix("matrix_b", n * p));
    let matrix_c = unwrap!(read_matrix("matrix_c", m * p));

    let buffer_a = unwrap!(Buffer::<f32>::builder().queue(queue.clone()).flags(flags::MEM_READ_ONLY).len(m * n).build());
    let buffer_b = unwrap!(Buffer::<f32>::builder().queue(queue.clone()).flags(flags::MEM_READ_ONLY).len(n * p).build());
    let buffer_c = unwrap!(Buffer::<f32>::builder().queue(queue.clone()).flags(flags::MEM_WRITE_ONLY).len(m * p).build());
    let mut matrix_c_actual = vec![0.0f32; (m * p) as usize];

    unwrap!(buffer_a.cmd().queue(&queue).offset(0).write(&matrix_a).enq());
    unwrap!(buffer_b.cmd().queue(&queue).offset(0).write(&matrix_b).enq());

    let src_filename = "tiled.cl";
    let program = unwrap!(build_ocl_program(device, context, format!("#define TILE_SIZE {}", requested_work_items), src_filename));
    let (kernel_name, _ext) = src_filename.split_at(src_filename.len() - 3);

    let kernel = unwrap!(Kernel::builder()
        .program(&program)
        .name(kernel_name)
        .queue(queue.clone())
        .arg(&buffer_a).arg(&buffer_b).arg(&buffer_c)
        .arg(m).arg(n).arg(p)
        .build());

    let mut exec_event = Event::empty();

    unsafe {
        unwrap!(kernel.cmd()
            .queue(&queue)
            .global_work_size([m, p])
            .local_work_size([requested_work_items, requested_work_items])
            .enew(&mut exec_event)
            .enq());
    }

    unwrap!(exec_event.wait_for());

    unwrap!(buffer_c.cmd().queue(&queue).offset(0).read(&mut matrix_c_actual).enq());

    verify_results(&matrix_c, &matrix_c_actual, p);
    println!("Execution time is {} [ms]", unwrap!(get_execution_time_ns(&exec_event)) as f64 / 1000000.0);
}

fn get_execution_time_ns(event: &Event) -> GenResult<u64> {
    use ocl::enums::{ProfilingInfo, ProfilingInfoResult::{Queued, End}};

    if let (Queued(time_queued), End(time_end)) =
        (event.profiling_info(ProfilingInfo::Queued)?, event.profiling_info(ProfilingInfo::End)?) {
        Ok(time_end - time_queued)
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

fn open_file(filename: &str) -> GenResult<File> {
    File::open(filename).or(gen_error_format!("Unable to open {} for reading", filename))
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

fn build_ocl_program(dev: Device, ctx: Context, kernel_defs: String, src_filename: &str) -> GenResult<Program> {
    let mut src_file_contents = String::new();
    open_file(src_filename)?.read_to_string(&mut src_file_contents)?;
    let src = kernel_defs + "\n" + &src_file_contents;

    with_gen_error!(Program::builder().devices(dev).src(src).build(&ctx))
}
