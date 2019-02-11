use std::ffi::CStr;
use ash::{vk, vk_make_version, Entry, Instance, Device};
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::extensions::{ext::DebugReport, khr::{Surface, Swapchain, WaylandSurface}};

const VK_API_VERSION: u32 = vk_make_version!(1, 1, 0);
const VK_ENABLED_LAYERS: [&'static CStr; 1] = [
    // Vulkan API doesn't have an extensive built-in error checking to avoid runtime overhead.
    // To verify API parameter values, catch thread-safety, etc. validation layers need to be
    // enabled explicitly. See https://vulkan.lunarg.com/doc/view/1.0.13.0/windows/layers.html
    // for more information.
    cstr!("VK_LAYER_LUNARG_standard_validation")
];
const VK_ENABLED_LAYERS_PTR: [*const i8; 1] = [
    VK_ENABLED_LAYERS[0].as_ptr()
];

/// Per documentation, "there is no global state in Vulkan and
/// all per-application state is stored in a VkInstance object".
pub fn create_instance(app_name: &CStr, entry: &Entry) -> Instance {
    let appinfo = vk::ApplicationInfo::builder()
        .application_name(app_name)
        .application_version(0)
        .engine_name(app_name)
        .engine_version(0)
        .api_version(VK_API_VERSION);

    // Unlike device extensions, global extensions apply to the whole program.
    let instance_extensions = [
        Surface::name().as_ptr(),
        WaylandSurface::name().as_ptr(),
        DebugReport::name().as_ptr()
    ];

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&appinfo)
        .enabled_layer_names(&VK_ENABLED_LAYERS_PTR)
        .enabled_extension_names(&instance_extensions);

    unsafe {
        entry.create_instance(&create_info, None).unwrap()
    }
}

/// Operations in Vulkan are submitted to a queue. To create a queue,
/// we first need to determine a suitable queue _family_. Queue families are
/// device-specific, so we also need to pick the right device.
///
/// Presenting images to a particular window system surface is a queue-specific
/// feature, so we must account for this too.
///
/// This function selects the first physical device with a graphics queue family, but we could
/// take into account physical device features, too, using instance.get_physical_device_features.
/// For instance, we could check for tessellation shaders support.
pub fn find_physical_device_and_graphics_queue_index(instance: &Instance, surface: vk::SurfaceKHR, surface_loader: &Surface) -> Option<(vk::PhysicalDevice, u32)> {
    let physical_devices = unsafe {
        instance.enumerate_physical_devices().unwrap()
    };
    physical_devices.into_iter().find_map(|d| {
        let queue_family_props = unsafe {
            instance.get_physical_device_queue_family_properties(d)
        };
        queue_family_props.into_iter().enumerate().find_map(|(index, info)| {
            let supports_graphics_and_surface = unsafe {
                info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && surface_loader.get_physical_device_surface_support(d, index as u32, surface)
            };
            if supports_graphics_and_surface { Some((d, index as u32)) }
            else { None }
        })
    })
}

/// A logical device is the primary interface for a physical device.
pub fn create_logical_device(instance: &Instance, physical_device: vk::PhysicalDevice, queue_family_idx: u32) -> Device {
    // If there are multiple queues in a single family, the priority of each one influences command
    // buffer execution scheduling. The value is required even if there's only one queue.
    let priorities = [1.0];
    let queue_infos = [
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_idx)
            .queue_priorities(&priorities)
            .build()
    ];

    let enabled_device_extensions = [
        Swapchain::name().as_ptr()
    ];
    let features = vk::PhysicalDeviceFeatures {
        ..Default::default()
    };
    let device_create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_extension_names(&enabled_device_extensions)
        .enabled_features(&features);

    unsafe {
        instance.create_device(physical_device, &device_create_info, None).unwrap()
    }
}

/// Swap chain is a set of image buffers the GPU draws into and that are presented to the display
/// hardware. They are set up with a particular colorspace and color channel format.
pub fn create_swapchain_with_surface(swapchain_loader: &Swapchain, physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR, surface_loader: &Surface, resolution: vk::Extent2D) -> (vk::SwapchainKHR, vk::SurfaceFormatKHR) {
    // We'll use sRGB for colorspace (https://stackoverflow.com/a/12894053/1726690) and
    // B8G8R8A8_UNORM for the format. In case they are not avilable, we just fail.
    // Real code will probably pick the most suitable out of all available.
    let surface_format =
        unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface) }
        .unwrap().into_iter().find_map(|f|
            if (f.format == vk::Format::B8G8R8A8_UNORM || f.format == vk::Format::UNDEFINED) && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {
                Some(f)
            }
            else {
                None
            }
        )
        .expect("No suitable surface format found");

    // Next, we pick the presentation mode. FIFO is guaranteed to be available and prevents tearing
    // (also see https://github.com/LunarG/VulkanSamples/issues/98)
    let present_mode = vk::PresentModeKHR::FIFO; 
    
    // Finally, we select the resolution images will be rendered with
    // (in our case, equal to window dimensions)
    let surface_capabilities = unsafe {
        surface_loader.get_physical_device_surface_capabilities(physical_device, surface).unwrap()
    };
    assert!(resolution.width <= surface_capabilities.max_image_extent.width);
    assert!(resolution.height <= surface_capabilities.max_image_extent.height);
    
    let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface)
        .min_image_count(surface_capabilities.min_image_count)
        .image_color_space(surface_format.color_space)
        .image_format(surface_format.format)
        .image_extent(resolution)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(surface_capabilities.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .image_array_layers(1);

    let swapchain = unsafe {
        swapchain_loader.create_swapchain(&swapchain_create_info, None).unwrap()
    };

    (swapchain, surface_format)
}

/// An image view describes how the image is accessed (color component mapping, mipmaps, etc.)
pub fn create_image_view(device: &Device, format: vk::Format, image: vk::Image) -> vk::ImageView {
    let create_view_info = vk::ImageViewCreateInfo::builder()
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .components(vk::ComponentMapping {
            // Channels can be swizzled & assigned constant values of 0 and 1, but we don't need that now
            r: vk::ComponentSwizzle::R, g: vk::ComponentSwizzle::G, b: vk::ComponentSwizzle::B, a: vk::ComponentSwizzle::A
        })
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1, // no mipmap levels
            base_array_layer: 0,
            layer_count: 1, // multiple layers can be used e.g. for left/right views in stereographic 3D
        })
        .image(image);

    unsafe {
        device.create_image_view(&create_view_info, None).unwrap()
    }
}


/// To execute commands in Vulkan, we need to record them first in command buffer objects.
/// Buffers are allocated from command pools, which manage the memory used to store them.
pub fn create_command_pool_and_buffers(device: &Device, queue_family_idx: u32, buffer_count: u32) -> (vk::CommandPool, Vec<vk::CommandBuffer>) {
    let pool_create_info = vk::CommandPoolCreateInfo::builder()
        // RESET_COMMAND_BUFFER: any command buffer allocated from a pool can be individually reset
        // without this flag, vkResetCommandBuffer must not be called for any buffer allocated from the pool
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        // command pools are tied to a specific queue
        .queue_family_index(queue_family_idx); 
    let pool = unsafe {
        device.create_command_pool(&pool_create_info, None).unwrap()
    };

    let buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_buffer_count(buffer_count)
        .command_pool(pool)
        .level(vk::CommandBufferLevel::PRIMARY);
    let buffers = unsafe {
        device.allocate_command_buffers(&buffer_allocate_info).unwrap()
    };

    (pool, buffers)
}

/// While the commands for draw operations are recorded in command buffers, we also need to
/// describe dependencies (image inputs, outputs, temporaries) so that image layout transitions
/// and barriers are resolved automatically — this is what a render pass is for.
/// A render pass consists of multiple subpasses — operations that depend on the results
/// of the previous passes.
/// Actual commands are recorded for each individual subpass. The order of execution of subpasses
/// may be changed by the driver as long as all subpass dependencies are satisfied; subpasses
/// may even be executed in parallel if they do not depend on each other.
/// See https://gpuopen.com/vulkan-renderpasses/ for more information.
pub fn create_renderpass(device: &Device, present_image_format: vk::Format) -> vk::RenderPass {
    // Attachments describe the inputs and temporaries:
    let renderpass_attachments = [
        vk::AttachmentDescription {
            format: present_image_format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR, // the image will be cleared to black before drawing a new frame
            store_op: vk::AttachmentStoreOp::STORE, // rendered contents should be stored in memory (to be presented later)
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR, // the image will be presented to the screen
            ..Default::default()
        }
    ];
    // Every subpass references one or more attachments:
    let color_attachment_refs = [
        vk::AttachmentReference {
            attachment: 0, // renderpass_attachments[0]
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL // the attachment is used as a color buffer
        }
    ];
    let subpasses = [
        vk::SubpassDescription::builder()
            // the index of the attachment corresponds to layout(location = ...) in the fragment shader
            .color_attachments(&color_attachment_refs)
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS) // could be VK_PIPELINE_BIND_POINT_COMPUTE
            .build()
    ];
    
    let renderpass_create_info = vk::RenderPassCreateInfo::builder()
        .attachments(&renderpass_attachments)
        .subpasses(&subpasses);

    unsafe {
        device.create_render_pass(&renderpass_create_info, None).unwrap()
    }
}
