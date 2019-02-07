use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub fn load_attrib(ctx: &WebGl2RenderingContext, program: &WebGlProgram, attrib: &str,
                   data: &js_sys::Object, size: i32, data_type: u32, normalized: bool) -> Result<(), String> {
    let buffer = ctx.create_buffer().ok_or("Failed to create WebGlBuffer")?;

    let a_location = match WebGl2RenderingContext::get_attrib_location(ctx, program, attrib) {
        -1 => Err(format!("Unable to find the \"{}\" attribute in the current program", attrib)),
        valid_location => Ok(valid_location as u32)
    }?;

    ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    ctx.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &data,
        WebGl2RenderingContext::STATIC_DRAW
    );
    ctx.enable_vertex_attrib_array(a_location);
    ctx.vertex_attrib_pointer_with_i32(a_location, size, data_type, normalized, 0, 0);

    Ok(())
}

pub fn compile_shader(ctx: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = ctx.create_shader(shader_type).ok_or("Failed to create WebGlShader")?;

    ctx.shader_source(&shader, source);
    ctx.compile_shader(&shader);

    let success = ctx.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool().unwrap_or(false);

    if success {
        Ok(shader)
    }
    else {
        Err(ctx.get_shader_info_log(&shader)
            .unwrap_or("Unknown error creating WebGlShader".to_owned()))
    }
}

pub fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(ctx: &WebGl2RenderingContext, program: &WebGlProgram, shaders: T) -> Result<(), String> {
    for shader in shaders {
        ctx.attach_shader(program, shader)
    }

    ctx.link_program(program);

    let success = ctx.get_program_parameter(program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool().unwrap_or(false);

    if success {
        Ok(())
    }
    else {
        Err(ctx.get_program_info_log(program)
            .unwrap_or("Unknown error creating WebGlProgram".to_owned()))
    }
}
