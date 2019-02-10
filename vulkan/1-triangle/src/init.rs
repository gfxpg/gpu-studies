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
/// hardware.
pub fn create_swapchain<I: InstanceV1_0, D: DeviceV1_0>(
    instance: &I, device: &D, physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR, surface_loader: &Surface,
    width: u32, height: u32
) -> vk::SwapchainKHR {
    // First, we need to determine the colorspace and color channel format.
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
    assert!(width <= surface_capabilities.max_image_extent.width);
    assert!(height <= surface_capabilities.max_image_extent.height);
    let surface_resolution = vk::Extent2D { width, height };
    
    let swapchain_loader = Swapchain::new(instance, device);
    let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface)
        .min_image_count(surface_capabilities.min_image_count)
        .image_color_space(surface_format.color_space)
        .image_format(surface_format.format)
        .image_extent(surface_resolution)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(surface_capabilities.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .image_array_layers(1);

    unsafe {
        swapchain_loader.create_swapchain(&swapchain_create_info, None).unwrap()
    }
}
