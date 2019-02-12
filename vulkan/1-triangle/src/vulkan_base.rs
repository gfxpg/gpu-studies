use ash::{vk, Entry, Instance, Device};
use ash::version::DeviceV1_0;
use ash::extensions::khr::{Surface, Swapchain};

use crate::{init, pipeline};

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
    pub pipeline: vk::Pipeline,

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

        let (_command_pool, command_buffers) = init::create_command_pool_and_buffers(&device, queue_family_idx,
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

        let present_queue = unsafe {
            device.get_device_queue(queue_family_idx, 0)
        };

        let pipeline = pipeline::create(&device, surface_resolution, renderpass);

        let semaphore_create_info = vk::SemaphoreCreateInfo::default();
        let present_complete_semaphore = unsafe {
            device.create_semaphore(&semaphore_create_info, None).unwrap()
        };
        let rendering_complete_semaphore = unsafe {
            device.create_semaphore(&semaphore_create_info, None).unwrap()
        };

        Self {
            entry, instance, device, physical_device, surface_loader, surface, surface_resolution,
            swapchain_loader, swapchain, present_images, present_image_views, present_queue,
            command_buffers, renderpass, framebuffers, pipeline,
            present_complete_semaphore, rendering_complete_semaphore
        }
    }

    pub fn draw_frame(&self) {
        let (present_idx, _) = unsafe {
            self.swapchain_loader.acquire_next_image(self.swapchain, std::u64::MAX,
                self.present_complete_semaphore, vk::Fence::null()).unwrap()
        };

        self.submit_command_buffer(present_idx as usize, || {
            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0]
                    }
                }
            ];
            let renderpass_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(self.renderpass)
                .framebuffer(self.framebuffers[present_idx as usize])
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: self.surface_resolution.clone(),
                })
                .clear_values(&clear_values);

            let cmd_buf = self.command_buffers[present_idx as usize];

            unsafe {
                self.device.cmd_begin_render_pass(cmd_buf, &renderpass_begin_info, vk::SubpassContents::INLINE);
                self.device.cmd_bind_pipeline(cmd_buf, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
                self.device.cmd_draw(cmd_buf, 3, 1, 0, 0);
                self.device.cmd_end_render_pass(cmd_buf);
            }
        });

        let wait_semaphors = [self.rendering_complete_semaphore];
        let swapchains = [self.swapchain];
        let image_indices = [present_idx];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphors)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            self.swapchain_loader.queue_present(self.present_queue, &present_info).unwrap();
        }
    }

    pub fn submit_command_buffer<F: FnOnce()>(&self, image_idx: usize, f: F) {
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

        f();

        unsafe {
            self.device.end_command_buffer(command_buffer).unwrap();
        }

        let submit_fence = unsafe {
            self.device.create_fence(&vk::FenceCreateInfo::default(), None).unwrap()
        };

        let command_buffers = [command_buffer];
        let wait_semaphores = [self.present_complete_semaphore];
        let signal_semaphores = [self.rendering_complete_semaphore]; // this semaphore will be signaled once command buffer finishes execution
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
