use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;

use crate::core_state::AppState;

use super::super::gl_common;
use super::super::math::matrix;
use super::Scene;

pub struct Scene2
{
    program: WebGlProgram,
    buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,
    pattern_length: usize,

    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Scene2 {

    pub fn new(gl: &GL) -> Self {
        
        // setup the program with the shaders
        let program = gl_common::link_program(
            &gl, 
            super::super::shaders::vertex::vs_color_2d_gradient::SHADER,
            super::super::shaders::fragment::fs_color_2d_gradient::SHADER,
        ).unwrap();

        // feed the shaders some nice data
        let verts: [f32; 8] = [
            0.,1.,
            0.,0.,
            1.,1.,
            1.,0.,
        ];

        // lets get a buffer
        let verts_ptr = verts.as_ptr() as u32 / 4; // divided by 4??
        let mem_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();        
        let verts_js = js_sys::Float32Array::new(&mem_buffer).subarray(
            verts_ptr,
            verts_ptr + verts.len() as u32,
        );
        let buffer = gl.create_buffer().ok_or("failed to create buffer...").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts_js, GL::STATIC_DRAW);

        // add pattern
        let pattern: [u16; 6] = [
            0, 1, 2, 2, 1 ,3
        ];
        
        
        // lets get a pattern buffer
        let pattern_mem_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();        
        let pattern_ptr = pattern.as_ptr() as u32 / 2; 
        let pattern_verts_js = js_sys::Uint16Array::new(&pattern_mem_buffer).subarray(
            pattern_ptr,
            pattern_ptr + pattern.len() as u32,
        );
        let pattern_buffer = gl.create_buffer().ok_or("failed to create buffer...").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&pattern_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER, 
            &pattern_verts_js, 
            GL::STATIC_DRAW);

        // 
        Self 
        {
            color_buffer: gl.create_buffer().ok_or("failed to create buffer").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),

            program: program,
            pattern_length: pattern.len(),
            buffer: buffer,
        }
    }
}

impl Scene for Scene2
{
    fn draw(&self, gl: &WebGlRenderingContext, state: &AppState)
    {
        // 
        gl.use_program(Some(&self.program));

        // procedure for 1 buffer
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform1f(Some(&self.u_opacity), 1.0);

        // procedure for second buffer 
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        let oc = (state.time / 1000.).sin();

        let colors: [f32; 16] = [
            0.0, 0.0+oc, 1.0, 1.0,
            0.0 + oc, 1.0, 0.0, 1.0,
            1.0, 0.0+ oc, 0.0, 1.0,
            1.0, 0.5, 0.5 + oc, 1.0,
        ];

        let buffer = gl_common::setup_current_buffer(
            &gl, 
            colors.as_ptr() as u32 / 4, 
            colors.len() as u32, 
            GL::ARRAY_BUFFER, 
            GL::DYNAMIC_DRAW
        );

        let tm = matrix::create_translation(
            2. * state.border_left / state.canvas_width -1.,
            2. * state.border_bottom / state.canvas_height - 1.,
            0.
        );

        let sm = matrix::create_scale(
            2. * (state.border_right - state.border_left) / state.canvas_width,
            2. * (state.border_top - state.border_bottom) / state.canvas_height,
            0.
        );

        let matrix = matrix::multiply(sm, tm);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &matrix);
        // gl.draw_arrays(GL::TRIANGLES, 0, (self.buffer_length / 2) as i32);

        gl.draw_elements_with_i32(GL::TRIANGLES, self.pattern_length as i32, GL::UNSIGNED_SHORT, 0);
    }

    fn start(&self) {
        todo!()
    }

    fn update(&mut self, state: &AppState) {
        todo!()
    }
}