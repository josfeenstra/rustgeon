// point-renderer.rs
// author: Jos Feenstra
// purpose: wrap webgl calls to render a bunch of points to the screen

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

pub struct PointRenderer {

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

}

impl PointRenderer {
    pub fn new(gl: &WebGlRenderingContext) {
        
        let program = gl_common::link_program(
            &gl,
            super::super::shaders::vertex::vs_graph_3d::SHADER,
            super::super::shaders::fragment::fs_color_3d::SHADER,
        ).unwrap();
        
        let verts_buffer = gl_common::setup_buffer_f32(&gl, &mesh.verts, BufferType::Regular, DrawType::Static);

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
        }
    }
}