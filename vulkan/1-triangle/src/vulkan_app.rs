use ash::{vk, Entry, Instance, Device};
use ash::extensions::khr::{Surface, Swapchain};

use crate::init;

pub struct VulkanApp {
    pub entry: Entry,
    pub instance: Instance,
    pub device: Device,
    pub physical_device: vk::PhysicalDevice,

    pub surface_loader: Surface,
    pub surface: vk::SurfaceKHR,

    pub swapchain_loader: Swapchain,
    pub swapchain: vk::SwapchainKHR,

    pub present_images: Vec<vk::Image>,
    pub present_image_views: Vec<vk::ImageView>,
}

impl VulkanApp {
    pub fn new(entry: Entry, instance: Instance, surface_loader: Surface, surface: vk::SurfaceKHR, window: &winit::Window) -> Self {
        let (physical_device, queue_family_idx) =
            init::find_physical_device_and_graphics_queue_index(&instance, surface, &surface_loader)
            .expect("No suitable devices found");
        let device = init::create_logical_device(&instance, physical_device, queue_family_idx);

        let (window_width, window_height) = window.get_inner_size().unwrap().into();
        let swapchain_loader = Swapchain::new(&instance, &device);
        let (swapchain, surface_format) = init::create_swapchain_with_surface_format(
            &swapchain_loader, physical_device, surface, &surface_loader, window_width, window_height);

        // The swap chain implementaion creates a set of images that can be drawn to and presented:
        let present_images: Vec<vk::Image> = unsafe {
            swapchain_loader.get_swapchain_images(swapchain).unwrap()
        };
        // To use the images in the render pipeline, we need to map them to image views:
        let present_image_views: Vec<vk::ImageView> = present_images
            .iter().map(|&image| init::create_image_view(&device, surface_format.format, image)).collect();

        Self {
            entry, instance, device, physical_device, surface_loader, surface, swapchain_loader, swapchain,
            present_images, present_image_views
        }
    }
}
