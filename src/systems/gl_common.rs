// Geon : gl_common 
// Author: Jos Feenstra

// common functions while using webgl
// basic abstractions, will try to expand this for 
// ergonomic usage of webgl

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;

pub enum DrawType {
    Static,
    Dynamic,
}

pub enum BufferType {
    Regular,
    Element, 
}


pub fn link_program(gl: &WebGlRenderingContext, vs_source: &str, fs_source: &str
    ) -> Result<WebGlProgram, String>
{
    let program  = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program..."))?;

    let vs = compile_shaders(&gl, GL::VERTEX_SHADER, vs_source).unwrap();
    let fs = compile_shaders(&gl, GL::FRAGMENT_SHADER, fs_source).unwrap();

    gl.attach_shader(&program, &vs);
    gl.attach_shader(&program, &fs);

    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false)
    {
        Ok(program)
    }
    else
    {
        Err(gl.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating gl program object")))
    }
}

fn compile_shaders(gl: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String>
{
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader..."))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    }
    else
    {
        Err(gl.get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unable to get shader info...")))
    }
}

pub fn fill_buffer_f32(gl: &GL, buffer: &WebGlBuffer, data: &Vec<f32>, buffer_type: BufferType, draw_type: DrawType) {

    // convert enums to fake enums
    let gl_buffer_type = match buffer_type
    {
        BufferType::Regular => GL::ARRAY_BUFFER,
        BufferType::Element => GL::ELEMENT_ARRAY_BUFFER,
    };

    let gl_draw_type = match draw_type
    {
        DrawType::Dynamic => GL::DYNAMIC_DRAW,
        DrawType::Static => GL::STATIC_DRAW,
    };

    // lets get a buffer
    let mem_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();  
    let ptr = data.as_ptr() as u32 / 4;      
    let array_js = js_sys::Float32Array::new(&mem_buffer).subarray(
        ptr,
        ptr + data.len() as u32,
    );
    gl.bind_buffer(gl_buffer_type, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        gl_buffer_type, 
        &array_js, 
        gl_draw_type); 
}

pub fn setup_buffer_f32(gl: &GL, data: &Vec<f32>, buffer_type: BufferType, draw_type: DrawType) 
    -> web_sys::WebGlBuffer { 
    // convert enums to fake enums
    let gl_buffer_type = match buffer_type
    {
        BufferType::Regular => GL::ARRAY_BUFFER,
        BufferType::Element => GL::ELEMENT_ARRAY_BUFFER,
    };

    let gl_draw_type = match draw_type
    {
        DrawType::Dynamic => GL::DYNAMIC_DRAW,
        DrawType::Static => GL::STATIC_DRAW,
    };

    // lets get a buffer
    let mem_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();  
    let ptr = data.as_ptr() as u32 / 4;      
    let array_js = js_sys::Float32Array::new(&mem_buffer).subarray(
        ptr,
        ptr + data.len() as u32,
    );
    let buffer = gl.create_buffer()
        .ok_or("failed to create buffer...")
        .unwrap();
    gl.bind_buffer(gl_buffer_type, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        gl_buffer_type, 
        &array_js, 
        gl_draw_type); 

    buffer
}

pub fn setup_buffer_u16(gl: &GL, data: &Vec<u16>, buffer_type: BufferType, draw_type: DrawType) 
    -> web_sys::WebGlBuffer {
    // convert enums to fake enums
    let gl_buffer_type = match buffer_type {
        BufferType::Regular => GL::ARRAY_BUFFER,
        BufferType::Element => GL::ELEMENT_ARRAY_BUFFER,
    };

    let gl_draw_type = match draw_type
    {
        DrawType::Dynamic => GL::DYNAMIC_DRAW,
        DrawType::Static => GL::STATIC_DRAW,
    };

    // lets get a buffer
    let mem_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer(); 
    let ptr = data.as_ptr() as u32 / 2;       
    let array_js = js_sys::Uint16Array::new(&mem_buffer).subarray(
        ptr,
        ptr + data.len() as u32,
    );
    let buffer = gl.create_buffer()
        .ok_or("failed to create buffer...")
        .unwrap();
    gl.bind_buffer(gl_buffer_type, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        gl_buffer_type, 
        &array_js, 
        gl_draw_type); 
        
    buffer
}

pub fn setup_current_buffer(gl: &GL, pointer: u32, length: u32, buffer_type: u32, draw_type: u32) 
    -> web_sys::WebGlBuffer {
    // get memory
    let mem_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    
    // get js array        
    let array_js = js_sys::Float32Array::new(&mem_buffer).subarray(
        pointer,
        pointer + length,
    );

    // get actual buffer 
    let buffer = gl.create_buffer()
        .ok_or("failed to create buffer...")
        .unwrap();

    // assign it
    gl.buffer_data_with_array_buffer_view(
        buffer_type, 
        &array_js, 
        draw_type); 
    
    // return it
    buffer
}

