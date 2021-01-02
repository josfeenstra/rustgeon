// entry point. 
// ONLY this file talks to javascript.
// ONLY this file recieves calls by javascript
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*; // still dont really know what prelude does
// use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use] 
extern crate lazy_static;

#[allow(dead_code)]

mod app_state;
mod shaders;
mod programs;
mod renderer;
mod math;
mod geometry;

use renderer::gl_common;


// how to get javascript to rust 
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// how to get rust to javascript
#[wasm_bindgen]
pub fn welcome_message()
{
    log("goedemorgen");
}


// how to get a class to javascript
#[wasm_bindgen]
pub struct Core {
    gl: GL,
    program1: programs::Program1,
    program3: programs::Program3,
}

#[wasm_bindgen]
impl Core {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self 
    {
        console_error_panic_hook::set_once();
        let gl = gl_common::init_webgl_context().unwrap();    
        
        Self {
            program1: programs::Program1::new(&gl),
            program3: programs::Program3::new(&gl),
            gl: gl,
        }
    }

    pub fn update(&mut self, time:f32, height: f32, width: f32) -> Result<(), JsValue> 
    {
        app_state::update_appstate(width, height, time);
        Ok(())   
    }

    pub fn draw(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let state = app_state::get_appstate();

        let oc = (state.total_time / 1000.).sin();

        self.program1.render(
            &self.gl, 
            state.border_top, 
            state.border_bottom, 
            state.border_left, 
            state.border_right, 
            state.canvas_width,  
            state.canvas_height,
            state.time,
        );

        self.program3.render(
            &self.gl, 
            state.border_top, 
            state.border_bottom, 
            state.border_left, 
            state.border_right, 
            state.canvas_width,  
            state.canvas_height,
            state.time,
            0.5 + oc,
            0.5 + oc,
        );
    }
}