mod gl;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use wavefront_obj::obj;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

const FRAGMENT_SHADER_SRC: &'static str = include_str!("../shaders/fragment.glsl");
const VERTEX_SHADER_SRC: &'static str = include_str!("../shaders/vertex.glsl");

#[wasm_bindgen]
pub fn load_gl(canvas_js: JsValue) -> Result<(), JsValue> {
    let canvas = canvas_js.dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas.get_context("webgl2")?.unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let v_shader = gl::compile_shader(&ctx, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER_SRC)?;
    let f_shader = gl::compile_shader(&ctx, WebGl2RenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)?;

    let program = gl::link_program(&ctx, [v_shader, f_shader].iter())?;

    ctx.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0,
                              0.7, -0.7, 0.0,
                              0.0, 0.7, 0.0];
    let vertices_location = vertices.as_ptr() as u32 / 4;
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()?
        .buffer();
    let vert_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + vertices.len() as u32);

    gl::load_attrib(&ctx, &vert_array, 3, WebGl2RenderingContext::FLOAT, false)?;

    ctx.clear_color(0.0, 0.0, 0.0, 0.0);
    ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (vertices.len() / 3) as i32);

    Ok(())
}

#[wasm_bindgen]
pub fn load_vertices(obj_src: String, object_idx: usize) -> Vec<f32> {
    let obj_set = obj::parse(obj_src).unwrap();
    let obj = &obj_set.objects[object_idx];

    obj.vertices
        .iter()
        .flat_map(|&obj::Vertex { x, y, z }| vec![x as f32, y as f32, z as f32])
        .collect()
}
