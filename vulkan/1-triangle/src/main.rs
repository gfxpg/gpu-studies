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

use ash::version::{EntryV1_0, InstanceV1_0};
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

    let events_loop = winit::EventsLoop::new_wayland();
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
}
