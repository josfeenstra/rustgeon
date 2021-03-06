use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;

use crate::{core_state::AppState, systems::{console, keys::Key}};

use super::super::gl_common;
use super::super::gl_common::{DrawType, BufferType};
use super::super::math::matrix;
use super::super::geometry;
use super::Scene;

pub struct Scene3 {

    // programs
    pub program: WebGlProgram,

    // buffers
    pub index_buffer: WebGlBuffer,
    pub index_count: i32,
    
    pub verts_buffer: WebGlBuffer,
    pub verts_count: i32,

    pub y_buffer: WebGlBuffer,

    // uniforms
    pub u_opacity: WebGlUniformLocation,
    pub u_projection: WebGlUniformLocation,

    // true data 
    pub y_data: Vec<f32>,
    pub size: i32,
    pub key_scale: f32,
}

impl Scene3 {

    pub fn new(gl: &WebGlRenderingContext, size: usize) -> Self {
        
        let program = gl_common::link_program(
            &gl,
            crate::render::shaders::vertex::vs_graph_3d::SHADER,
            crate::render::shaders::fragment::fs_color_3d::SHADER,
        ).unwrap();
        
        // THE ACTUAL DATA
        let mesh = geometry::mesh::create_grid(size);
        let y_data = vec![0.0; mesh.verts.len()];

        let verts_buffer = gl_common::setup_buffer_f32(&gl, &mesh.verts, BufferType::Regular, DrawType::Static);
        let index_buffer = gl_common::setup_buffer_u16(&gl, &mesh.indices, BufferType::Element, DrawType::Static);
        let y_buffer = gl_common::setup_buffer_f32(&gl, &y_data, BufferType::Regular, DrawType::Dynamic);

        Self {
            
            // uniforms
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_projection: gl.get_uniform_location(&program, "uProjection").unwrap(),

            // buffers
            index_buffer: index_buffer,
            index_count: mesh.indices.len() as i32,
            verts_buffer: verts_buffer,
            verts_count: mesh.verts.len() as i32,

            // program
            program: program,
            y_buffer: y_buffer,
            y_data: y_data,

            // general
            size: size as i32,
            key_scale: 0.,
        }
    }
}

impl Scene for Scene3 {

    fn start(&self) {
        todo!()
    }

    fn update(&mut self, s: &AppState) {
        self.y_data = get_updated_3d_y_values(self.size as usize, s.time);

        // key test 
        if s.keypressed(Key::Minus) {
            console::log_str("minus");
            self.key_scale += 30.;
        }
        if s.keypressed(Key::Plus) {
            console::log_str("plus");
            self.key_scale -= 30.;
        }
    }

    fn draw(&self, gl: &WebGlRenderingContext, s: &AppState) 
    {
        gl.use_program(Some(&self.program));
        
        let projection = matrix::get_3d_projection_matrix(
            s.border_bottom, s.border_top, s.border_left, s.border_right, 
            s.canvas_width, s.canvas_height, s.cam_rotation_x, s.cam_rotation_y, (s.mouse_scroll + self.key_scale) * -0.01);

        gl.enable_vertex_attrib_array(0);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_projection), false, &projection);
        gl.uniform1f(Some(&self.u_opacity), 1.);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.verts_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);

        gl_common::fill_buffer_f32(&gl, &self.y_buffer, &self.y_data, BufferType::Regular, DrawType::Dynamic);
        
        gl.enable_vertex_attrib_array(1);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.y_buffer));
        gl.vertex_attrib_pointer_with_i32(1, 1, GL::FLOAT, false, 0, 0);

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        
        let wireframe: bool = true;
        if wireframe {
            gl.draw_elements_with_i32(GL::LINE_STRIP, self.index_count, GL::UNSIGNED_SHORT, 0);
        } 
        gl.uniform1f(Some(&self.u_opacity), 0.5);
        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }
}

pub fn get_updated_3d_y_values(size: usize, curr_time: f32) -> Vec<f32> {
    let point_count_per_row = size + 1;
    let mut y_vals: Vec<f32> = vec![0.; point_count_per_row * point_count_per_row];
    let half_grid: f32 = point_count_per_row as f32 / 2.;
    let frequency_scale: f32 = 3. * std::f32::consts::PI;
    let y_scale = 0.15;
    let sin_offset = curr_time / 1000.; //speed

    for z in 0..point_count_per_row {
        for x in 0..point_count_per_row {
            let use_y_index = z * point_count_per_row + x;
            let scaled_x = frequency_scale * (x as f32 - half_grid) / half_grid;
            let scaled_z = frequency_scale * (z as f32 - half_grid) / half_grid;
            y_vals[use_y_index] = y_scale * ((scaled_x * scaled_x + scaled_z * scaled_z).sqrt() + sin_offset).sin();
        }
    }

    y_vals
}