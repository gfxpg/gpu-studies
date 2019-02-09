mod gl;
mod interop;
mod model;
mod transform;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram};

const MODEL_PLY: &'static str = include_str!("../plane100.ply");
const VERTEX_SHADER_SRC: &'static str = include_str!("../shaders/vertex.glsl");
const FRAGMENT_SHADER_SRC: &'static str = include_str!("../shaders/fragment.glsl");

#[wasm_bindgen]
pub struct Renderer {
    ctx: WebGl2RenderingContext,
    program: WebGlProgram,
    model: model::Model,
    perspective_mat: [f32; 16],

    rot_x_rad: f32,
    rot_y_rad: f32,
    scale: f32
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
            .map_err(|e| format!("ply error: {}", e))?;

        let perspective_mat = transform::matmul4(
            transform::mat4_translation(0.0, 0.0, -2.5),
            transform::mat4_perspective(
                60_f32.to_radians(), canvas.width() as f32 / canvas.height() as f32,
                1.0, 12.0)
        );

        Ok(Renderer {
            ctx, program, model, perspective_mat,
            rot_x_rad: 0.0, rot_y_rad: 0.0, scale: 1.0
        })
    }

    pub fn set_rotation(&mut self, x_rad: f32, y_rad: f32) {
        self.rot_x_rad = x_rad;
        self.rot_y_rad = y_rad;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn instantiate(&self) -> Result<(), JsValue> {
        let v_shader = gl::compile_shader(&self.ctx, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER_SRC)?;
        let f_shader = gl::compile_shader(&self.ctx, WebGl2RenderingContext::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)?;

        gl::link_program(&self.ctx, &self.program, [v_shader, f_shader].iter())?;

        let vertices_array = interop::get_float32array(&self.model.vertices)?;

        gl::load_attrib(&self.ctx, &self.program, "a_position", &vertices_array, 3, WebGl2RenderingContext::FLOAT, false)?;

        /* Don't draw back-facing triangles */
        //self.ctx.enable(WebGl2RenderingContext::CULL_FACE);
        //self.ctx.enable(WebGl2RenderingContext::DEPTH_TEST);

        self.ctx.use_program(Some(&self.program));

        Ok(())
    }

    pub fn render(&self) -> Result<(), JsValue> {
        let mut world_mat =
            transform::matmul4(
                transform::mat4_scale(self.scale),
                transform::matmul4(
                    transform::mat4_rot_y(self.rot_y_rad),
                    transform::matmul4(
                        transform::mat4_rot_x(self.rot_x_rad),
                        self.perspective_mat
                    )
                )
            );

        self.ctx.clear_color(0.0, 0.0, 0.0, 0.0);
        self.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        let _ = gl::load_uniform_mat4(&self.ctx, &self.program, "u_world_transform", &mut world_mat);

        self.ctx.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (self.model.vertices.len() / 3) as i32);

        Ok(())
    }
}
