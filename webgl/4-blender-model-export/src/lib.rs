mod gl;
mod interop;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use ply_rs as ply;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

const MODEL_PLY: &'static str = include_str!("../cube.ply");
const VERTEX_SHADER_SRC: &'static str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER_SRC: &'static str = include_str!("../shaders/fragment.glsl");

#[wasm_bindgen]
pub fn load_gl(canvas_js: JsValue) -> Result<(), JsValue> {
    let canvas = canvas_js.dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas.get_context("webgl2")?.unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let v_shader = gl::compile_shader(&ctx, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER_SRC)?;
    let f_shader = gl::compile_shader(&ctx, WebGl2RenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)?;

    let program = gl::link_program(&ctx, [v_shader, f_shader].iter())?;

    ctx.use_program(Some(&program));

    let (vertices, colors) = load_model().map_err(|e| format!("{}", e))?;
    let vertices_array = interop::get_float32array(&vertices)?;
    let colors_array = interop::get_uint8array(&colors)?;

    gl::load_attrib(&ctx, &program, "a_position", &vertices_array, 3, WebGl2RenderingContext::FLOAT, false)?;
    gl::load_attrib(&ctx, &program, "a_color", &colors_array, 3, WebGl2RenderingContext::UNSIGNED_BYTE, true)?;

    ctx.clear_color(0.0, 0.0, 0.0, 0.0);
    ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (vertices.len() / 3) as i32);

    Ok(())
}

pub fn load_model() -> std::io::Result<(Vec<f32>, Vec<u8>)> {
    use ply_rs::ply::PropertyAccess;

    let parser = ply::parser::Parser::<ply::ply::DefaultElement>::new();
    let mut model_source = std::io::Cursor::new(MODEL_PLY);
    let model = parser.read_ply(&mut model_source)?.payload;

    let x_key = "x".to_owned();
    let y_key = "y".to_owned();
    let z_key = "z".to_owned();

    let r_key = "red".to_owned();
    let g_key = "green".to_owned();
    let b_key = "blue".to_owned();

    let vertices: Vec<f32> = model["vertex"].iter().flat_map(|m| {
        vec![
            m.get_float(&x_key).unwrap(),
            m.get_float(&y_key).unwrap(),
            m.get_float(&z_key).unwrap()
        ]
    }).collect();

    let colors: Vec<u8> = model["vertex"].iter().flat_map(|m| {
        vec![
            m.get_uchar(&r_key).unwrap(),
            m.get_uchar(&g_key).unwrap(),
            m.get_uchar(&b_key).unwrap()
        ]
    }).collect();

    Ok((vertices, colors))
}
