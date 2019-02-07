mod gl;
mod interop;
mod model;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram};

const MODEL_PLY: &'static str = include_str!("../cube.ply");
const VERTEX_SHADER_SRC: &'static str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER_SRC: &'static str = include_str!("../shaders/fragment.glsl");

#[wasm_bindgen]
pub struct Renderer {
    ctx: WebGl2RenderingContext,
    program: WebGlProgram,
    model: (Vec<f32>, Vec<u8>)
}

#[wasm_bindgen]
impl Renderer {
    pub fn new(canvas_js: JsValue) -> Result<Renderer, JsValue> {
        let canvas = canvas_js.dyn_into::<HtmlCanvasElement>()?;

        let ctx = canvas.get_context("webgl2")?.unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        let program = ctx.create_program()
            .ok_or("Failed to create WebGlProgram")?;

        let model = model::load(MODEL_PLY)
            .map_err(|e| format!("{}", e))?;

        Ok(Renderer { ctx, program, model })
    }

    pub fn instantiate(&self) -> Result<(), JsValue> {
        let v_shader = gl::compile_shader(&self.ctx, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER_SRC)?;
        let f_shader = gl::compile_shader(&self.ctx, WebGl2RenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)?;

        gl::link_program(&self.ctx, &self.program, [v_shader, f_shader].iter())?;

        self.ctx.use_program(Some(&self.program));

        let (ref vertices, ref colors) = self.model;
        let vertices_array = interop::get_float32array(vertices)?;
        let colors_array = interop::get_uint8array(colors)?;

        gl::load_attrib(&self.ctx, &self.program, "a_position", &vertices_array, 3, WebGl2RenderingContext::FLOAT, false)?;
        gl::load_attrib(&self.ctx, &self.program, "a_color", &colors_array, 3, WebGl2RenderingContext::UNSIGNED_BYTE, true)?;

        Ok(())
    }

    pub fn render(&self) {
        self.ctx.clear_color(0.0, 0.0, 0.0, 0.0);
        self.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        self.ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (self.model.0.len() / 3) as i32);
    }
}
