use wasm_bindgen::JsCast;
// use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;

use super::super::gl_common;
use super::super::gl_common::{DrawType, BufferType};
use super::super::math::matrix;

pub struct Program1 
{
    program: WebGlProgram,
    buffer: WebGlBuffer,
    buffer_length: usize,

    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Program1
{
    pub fn new(gl: &GL) -> Self {
        
        // setup the program with the shaders
        let program = gl_common::link_program(
            &gl, 
            super::super::shaders::vertex::vs_color_2d::SHADER,
            super::super::shaders::fragment::fs_color_2d::SHADER,
        ).unwrap();

        // feed the shaders some nice data
        let verts: Vec<f32> = vec![
            0.,1.,
            0.,0.,
            1.,1.,
            1.,1.,
            0.,0.,
            1.,0.,
        ];
        let verts_ptr = verts.as_ptr() as u32 / 4; // divided by 4??


        // lets get a buffer

        let buffer = gl_common::setup_buffer_f32( 
            &gl,
            &verts,
            BufferType::Regular, 
            DrawType::Static,
        );

        // let mem_buffer = wasm_bindgen::memory()
        //     .dyn_into::<WebAssembly::Memory>()
        //     .unwrap()
        //     .buffer();        
        // let verts_js = js_sys::Float32Array::new(&mem_buffer).subarray(
        //     verts_ptr,
        //     verts_ptr + verts.len() as u32,
        // );
        // let buffer = gl.create_buffer().ok_or("failed to create buffer...").unwrap();
        // gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        // gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts_js, GL::STATIC_DRAW);

        // 
        Self 
        {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),

            program: program,
            buffer_length: verts.len(),
            buffer: buffer,
        }
    }

    pub fn render(&self, gl: &WebGlRenderingContext, 
        bottom:f32, top:f32, left:f32, right: f32, canvas_width: f32, canvas_height: f32, total_time: f32)
    {
        gl.use_program(Some(&self.program));

        // procedure for 1 buffer
        gl.enable_vertex_attrib_array(0);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        

        // 
        gl.uniform4f(Some(&self.u_color), 0., 1.0, 0.5, 1.0);
        gl.uniform1f(Some(&self.u_opacity), 1.0);
        

        let tm = matrix::create_translation(
            2. * left / canvas_width -1.,
            2. * bottom / canvas_height - 1.,
            0.
        );

        let sm = matrix::create_scale(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.
        );

        let matrix = matrix::multiply(sm, tm);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &matrix);
        gl.draw_arrays(GL::TRIANGLES, 0, (self.buffer_length / 2) as i32);
    }
}