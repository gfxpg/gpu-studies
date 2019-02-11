#![feature(const_cstr_unchecked)]
#![feature(const_str_as_bytes)]

#[macro_use]
mod cffi;
mod init;
mod vulkan_base;

use std::ffi::CStr;
use ash::vk;

use vulkan_base::VulkanBase;

const APP_NAME: &'static CStr = cstr!("gpu-studies");
const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::extensions::khr::{Surface, WaylandSurface};

fn create_surface<E: EntryV1_0, I: InstanceV1_0>(entry: &E, instance: &I, window: &winit::Window) -> vk::SurfaceKHR {
    use winit::os::unix::WindowExt;

    let w_surface = window.get_wayland_surface().unwrap();
    let w_display = window.get_wayland_display().unwrap();
    let create_info = vk::WaylandSurfaceCreateInfoKHR::builder()
        .display(w_display)
        .surface(w_surface);

    unsafe {
        WaylandSurface::new(entry, instance).create_wayland_surface(&create_info, None).unwrap()
    }
}

fn main() {
    use winit::os::unix::EventsLoopExt;

    let mut events_loop = winit::EventsLoop::new_wayland();
    let window = winit::WindowBuilder::new()
        .with_title("gpu-studies")
        .with_dimensions(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&events_loop)
        .unwrap();

    let entry = ash::Entry::new().unwrap();
    let instance = init::create_instance(APP_NAME, &entry);

    // Vulkan API does not interface with the window system directly; instead,
    // a SurfaceKHR object is used, which represents an abstract surface
    // rendered images are presented to.
    let surface_loader = Surface::new(&entry, &instance);
    let surface: vk::SurfaceKHR = create_surface(&entry, &instance, &window);

    let base = VulkanBase::new(entry, instance, surface_loader, surface, &window); 

    use ash::extensions::ext::DebugReport;
    let debug_info = vk::DebugReportCallbackCreateInfoEXT::builder()
        .flags(
            vk::DebugReportFlagsEXT::ERROR
                | vk::DebugReportFlagsEXT::INFORMATION
                | vk::DebugReportFlagsEXT::WARNING
                | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING
        )
        .pfn_callback(Some(vulkan_debug_callback));
    let debug_report_loader = DebugReport::new(&base.entry, &base.instance);
    let debug_callback = unsafe {
        debug_report_loader.create_debug_report_callback(&debug_info, None).unwrap()
    };

    events_loop.run_forever(|event| {
        use winit::{Event, WindowEvent, VirtualKeyCode, ControlFlow};

        let (present_idx, _) = unsafe {
            base.swapchain_loader.acquire_next_image(base.swapchain, std::u64::MAX,
                base.present_complete_semaphore, vk::Fence::null()).unwrap()
        };

        base.submit_command_buffer(present_idx as usize, |device, cmd_buf| {
            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [1.0, 0.8, 0.4, 1.0]
                    }
                }
            ];
            let renderpass_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(base.renderpass)
                .framebuffer(base.framebuffers[present_idx as usize])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: base.surface_resolution.clone(),
                })
                .clear_values(&clear_values);
            
            unsafe {
                device.cmd_begin_render_pass(cmd_buf, &renderpass_begin_info, vk::SubpassContents::INLINE);

                device.cmd_end_render_pass(cmd_buf);
            }
        });

        let wait_semaphors = [base.rendering_complete_semaphore];
        let swapchains = [base.swapchain];
        let image_indices = [present_idx];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphors)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            base.swapchain_loader.queue_present(base.present_queue, &present_info).unwrap();
        }

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                        ControlFlow::Break
                    } else {
                        ControlFlow::Continue
                    }
                }
                WindowEvent::CloseRequested => ControlFlow::Break,
                _ => ControlFlow::Continue,
            },
            _ => ControlFlow::Continue,
        }
    });
}

use std::os::raw::{c_char, c_void};

unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const c_char,
    p_message: *const c_char,
    _: *mut c_void,
) -> u32 {
    println!("vk: {:?}", CStr::from_ptr(p_message));
    vk::FALSE
}
