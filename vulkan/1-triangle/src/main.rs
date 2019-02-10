#![feature(const_cstr_unchecked)]
#![feature(const_str_as_bytes)]

#[macro_use]
mod cffi;
mod init;

use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use ash::vk;

const APP_NAME: &'static CStr = cstr!("gpu-studies");
const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;

use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::extensions::khr::{Surface, Swapchain, WaylandSurface};

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

    let entry = ash::Entry::new().unwrap();
    let instance = init::create_instance(APP_NAME, &entry);

    let events_loop = winit::EventsLoop::new_wayland();
    let window = winit::WindowBuilder::new()
        .with_title("gpu-studies")
        .with_dimensions(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&events_loop)
        .unwrap();

    // Vulkan API does not interface with the window system directly; instead,
    // a SurfaceKHR object is used, which represents an abstract surface
    // rendered images are presented to.
    let surface_loader = Surface::new(&entry, &instance);
    let surface: vk::SurfaceKHR = create_surface(&entry, &instance, &window);

    let (p_device, queue_family_idx) =
        init::find_physical_device_and_graphics_queue_index(&instance, surface, &surface_loader)
        .expect("No suitable devices found");

    let device = init::create_logical_device(&instance, p_device, queue_family_idx);

    let swapchain = init::create_swapchain(&instance, &device, p_device,
        surface, &surface_loader, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
}
