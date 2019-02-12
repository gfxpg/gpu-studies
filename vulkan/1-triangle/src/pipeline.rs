use std::ffi::CStr;
use ash::{vk, Entry, Instance, Device};
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};

const SHADER_ENTRY_NAME: &'static CStr = cstr!("main");
const VERT_SHADER_SPIRV: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/triangle.vert.spv"));
const FRAG_SHADER_SPIRV: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/triangle.frag.spv"));

pub fn create(device: &Device, resolution: vk::Extent2D, renderpass: vk::RenderPass) -> vk::Pipeline {
    let vert_shader_module = create_shader_module(device, VERT_SHADER_SPIRV);
    let frag_shader_module = create_shader_module(device, FRAG_SHADER_SPIRV);

    // Shaders need to be assigned to specific pipeline stages:
    let shader_stage_create_infos = [
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::VERTEX,
            module: vert_shader_module,
            p_name: SHADER_ENTRY_NAME.as_ptr(),
            ..Default::default()
        },
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: frag_shader_module,
            p_name: SHADER_ENTRY_NAME.as_ptr(),
            ..Default::default()
        },
    ];

    // We have no vertex data to load yet, bindings and attribute descriptions are set to null
    let vertex_input_state_info: vk::PipelineVertexInputStateCreateInfo = Default::default();

    // TRIANGLE_LIST: interpret every 3 vertices as a triangle
    let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        ..Default::default()
    };

    // A viewport describes the region of the framebuffer the output is rendered to.
    let viewports = [
        vk::Viewport {
            x: 0.0, y: 0.0,
            width: resolution.width as f32, height: resolution.height as f32,
            min_depth: 0.0, max_depth: 1.0
        }
    ];
    // Pixels outside scissor rectangles are discarded by the rasterizer.
    let scissors = [
        vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: resolution.clone()
        }
    ];
    let viewport_state_info = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(&viewports)
        .scissors(&scissors);

    let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
        // lines thicker than 1.0 require enabling a GPU feature
        line_width: 1.0,
        // fill the area of polygons with fragments;
        // other options are to draw only edges (lines) or only vertices (points)
        polygon_mode: vk::PolygonMode::FILL, 
        // cull the back faces
        cull_mode: vk::CullModeFlags::BACK,
        // front faces have the vertices go in CW direction
        front_face: vk::FrontFace::CLOCKWISE,
        ..Default::default()
    };

    // Disable multisampling for now
    let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
        sample_shading_enable: 0,
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        ..Default::default()
    };

    // The color returned from a fragment shader can be blended with the color already
    // in the framebuffer — we disable this for now.
    let color_blend_attachment_states = [
        vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            color_write_mask: vk::ColorComponentFlags::all(),
            ..Default::default()
        }
    ];
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op(vk::LogicOp::CLEAR)
        .attachments(&color_blend_attachment_states);

    // Some state specified above can be changed without recreating the pipeline
    // (e.g. the viewport size). We don't use this feature for now.
    let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default();

    // Pipeline layout specifies `uniform` values to be used in shaders:
    let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::default();
    let pipeline_layout = unsafe {
        device.create_pipeline_layout(&pipeline_layout_create_info, None).unwrap()
    };

    let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(&shader_stage_create_infos)
        .vertex_input_state(&vertex_input_state_info)
        .input_assembly_state(&vertex_input_assembly_state_info)
        .viewport_state(&viewport_state_info)
        .rasterization_state(&rasterization_info)
        .multisample_state(&multisample_state_info)
        .color_blend_state(&color_blend_state)
        .dynamic_state(&dynamic_state_info)
        .layout(pipeline_layout)
        .render_pass(renderpass);

    let graphics_pipeline = unsafe {
        device.create_graphics_pipelines(vk::PipelineCache::null(), &[graphic_pipeline_info.build()], None)
            .unwrap().remove(0) // take the first created pipeline
    };
    
    // A shader module can be destroyed once the pipeline that uses its shaders is created.
    unsafe {
        device.destroy_shader_module(vert_shader_module, None);
        device.destroy_shader_module(frag_shader_module, None);
    }

    graphics_pipeline
}

fn create_shader_module(device: &Device, shader_code: &[u8]) -> vk::ShaderModule {
    let spv = ash::util::read_spv(&mut std::io::Cursor::new(&shader_code)).unwrap();
    let shader_info = vk::ShaderModuleCreateInfo::builder().code(&spv);

    unsafe {
        device.create_shader_module(&shader_info, None).unwrap()
    }
}
