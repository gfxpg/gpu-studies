use ash::{vk, Entry, Instance, Device};
use ash::version::DeviceV1_0;
use ash::extensions::khr::{Surface, Swapchain};

use crate::init;

pub struct VulkanBase {
    pub entry: Entry,
    pub instance: Instance,
    pub device: Device,
    pub physical_device: vk::PhysicalDevice,

    pub surface_loader: Surface,
    pub surface: vk::SurfaceKHR,
    pub surface_resolution: vk::Extent2D,

    pub swapchain_loader: Swapchain,
    pub swapchain: vk::SwapchainKHR,

    pub present_images: Vec<vk::Image>,
    pub present_image_views: Vec<vk::ImageView>,

    pub present_queue: vk::Queue,
    pub command_buffers: Vec<vk::CommandBuffer>,

    pub renderpass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,

    pub present_complete_semaphore: vk::Semaphore,
    pub rendering_complete_semaphore: vk::Semaphore,
}

impl VulkanBase {
    pub fn new(entry: Entry, instance: Instance, surface_loader: Surface, surface: vk::SurfaceKHR, window: &winit::Window) -> Self {
        let (physical_device, queue_family_idx) =
            init::find_physical_device_and_graphics_queue_index(&instance, surface, &surface_loader)
            .expect("No suitable devices found");
        let device = init::create_logical_device(&instance, physical_device, queue_family_idx);

        let (window_width, window_height) = window.get_inner_size().unwrap().into();
        let surface_resolution = vk::Extent2D { width: window_width, height: window_height };

        let swapchain_loader = Swapchain::new(&instance, &device);
        let (swapchain, surface_format) = init::create_swapchain_with_surface(
            &swapchain_loader, physical_device, surface, &surface_loader, surface_resolution);

        // The swap chain implementaion creates a set of images that can be drawn to and presented:
        let present_images: Vec<vk::Image> = unsafe {
            swapchain_loader.get_swapchain_images(swapchain).unwrap()
        };
        // To use the images in the render pipeline, we need to map them to image views:
        let present_image_views: Vec<vk::ImageView> = present_images
            .iter().map(|&image| init::create_image_view(&device, surface_format.format, image)).collect();

        let (command_pool, command_buffers) = init::create_command_pool_and_buffers(&device, queue_family_idx,
            // During rendering, buffers are bound to a particular image, so we need to create as many as there are images:
            present_images.len() as u32);

        let renderpass = init::create_renderpass(&device, surface_format.format);

        // A framebuffer wraps image views for presentation. Since there are several image views in
        // the swap chain, we need to create framebuffers for each one. During rendering, we'll
        // choose the framebuffer that corresponds to the image view retrieved from the swap chain.
        let framebuffers: Vec<vk::Framebuffer> = present_image_views
            .iter().map(|&image_view| {
                let attachments = [image_view];

                let framebuffer_create_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(renderpass)
                    .attachments(&attachments)
                    .width(window_width)
                    .height(window_height)
                    .layers(1);

                unsafe {
                    device.create_framebuffer(&framebuffer_create_info, None).unwrap()
                }
            })
            .collect();

        let semaphore_create_info = vk::SemaphoreCreateInfo::default();
        let present_complete_semaphore = unsafe {
            device.create_semaphore(&semaphore_create_info, None).unwrap()
        };
        let rendering_complete_semaphore = unsafe {
            device.create_semaphore(&semaphore_create_info, None).unwrap()
        };

        let present_queue = unsafe {
            device.get_device_queue(queue_family_idx, 0)
        };

        Self {
            entry, instance, device, physical_device, surface_loader, surface, surface_resolution,
            swapchain_loader, swapchain, present_images, present_image_views, present_queue,
            command_buffers, renderpass, framebuffers,
            present_complete_semaphore, rendering_complete_semaphore
        }
    }

    pub fn submit_command_buffer<F: FnOnce(&Device, vk::CommandBuffer)>(&self, image_idx: usize, f: F) {
        let command_buffer = self.command_buffers[image_idx];

        unsafe {
            self.device.reset_command_buffer(
                command_buffer, vk::CommandBufferResetFlags::RELEASE_RESOURCES).unwrap();
        }

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            // ONE_TIME_SUBMIT: each recording of the command buffer will only be submitted once,
            // and the command buffer will be reset and recorded again after the submission.
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.device.begin_command_buffer(command_buffer, &command_buffer_begin_info).unwrap();
        }

        f(&self.device, command_buffer);

        unsafe {
            self.device.end_command_buffer(command_buffer).unwrap();
        }

        let submit_fence = unsafe {
            self.device.create_fence(&vk::FenceCreateInfo::default(), None).unwrap()
        };

        let command_buffers = [command_buffer];
        let wait_semaphores = [self.present_complete_semaphore];
        let signal_semaphores = [self.rendering_complete_semaphore];
        let wait_mask = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_mask)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);

        unsafe {
            self.device.queue_submit(self.present_queue, &[submit_info.build()], submit_fence).unwrap();
            self.device.wait_for_fences(&[submit_fence], true, std::u64::MAX).unwrap();
            self.device.destroy_fence(submit_fence, None);
        }
    }
}
