use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;


use super::super::gl_common;
use super::super::math::matrix;
use super::super::geometry;

pub struct Program3 {
    // programs
    pub program: WebGlProgram,

    // buffers
    pub index_buffer: WebGlBuffer,
    pub index_count: i32,
    
    pub verts_buffer: WebGlBuffer,
    pub verts_count: i32,

    // uniforms
    pub u_opacity: WebGlUniformLocation,
    pub u_projection: WebGlUniformLocation,
}

impl Program3 {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        
        let program = gl_common::link_program(
            &gl,
            super::super::shaders::vertex::vs_graph_3d::SHADER,
            super::super::shaders::fragment::fs_color_3d::SHADER,
        ).unwrap();
        
        let mesh = geometry::mesh::create_grid(10);
        let verts_buffer = gl_common::setup_buffer_f32_standard(&gl, &mesh.verts);
        let index_buffer = gl_common::setup_buffer_u16_standard(&gl, &mesh.indices);

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

        }
    }

    pub fn render(&self, gl: &WebGlRenderingContext, 
        top:f32, bottom:f32, left:f32, right: f32, 
        canvas_width: f32, canvas_height: f32, _total_time: f32,
        rotation_angle_x: f32, rotation_angle_y: f32)
    {
        gl.use_program(Some(&self.program));
        
        let projection = matrix::get_3d_projection_matrix(
            bottom, top, left, right, 
            canvas_width, canvas_height, rotation_angle_x, rotation_angle_y);
        
        matrix::print(projection);

        let positionAttributeLocation = 0;
        gl.enable_vertex_attrib_array(positionAttributeLocation);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_projection), false, &projection);
        gl.uniform1f(Some(&self.u_opacity), 1.);
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.verts_buffer));
        gl.vertex_attrib_pointer_with_i32(positionAttributeLocation, 3, GL::FLOAT, false, 0, 0);

        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }
}