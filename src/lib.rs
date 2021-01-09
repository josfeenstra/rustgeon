// author : Jos Feenstra
// based upon: Doug Milfords' Rust 3D Graphics tutorials

// entry point. 
// ONLY this file talks to javascript.
// ONLY this file recieves calls by javascript
extern crate wasm_bindgen;
use std::sync::Mutex;
use wasm_bindgen::prelude::*; // still dont really know what prelude does
// use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use] 
extern crate lazy_static;

mod app_state;
mod shaders;
mod math;
mod geometry;
mod programs;
mod systems;

use systems::gl_common;
use systems::context;


// how to get javascript to rust 
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use std::collections::HashMap;

lazy_static! {
    static ref logmap: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new()); 
}

pub fn log_once(message: &str, key: &str)
{
    let mut map = logmap.lock().unwrap();
    if !map.contains_key(key) {
        map.insert(key.to_string(), "".to_string());
        log(&message);    
    }
}

// how to get rust to javascript
#[wasm_bindgen]
pub fn welcome_message()
{
    log("goedemorgen");
}

pub struct Screen
{

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
        let gl = context::init_webgl_context().unwrap();    
        
        Self {
            program1: programs::Program1::new(&gl),
            program3: programs::Program3::new(&gl, 50),
            gl: gl,
        }
    }

    pub fn update(&mut self, time:f32, height: f32, width: f32) -> Result<(), JsValue> 
    {
        app_state::update_appstate(width, height, time);
        self.program3.update(time);
        Ok(())   
    }

    pub fn draw(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let state = app_state::get_appstate();

        let oc = (state.time / 1000.).sin();

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
            state.cam_rotation_x,
            state.cam_rotation_y,
            state.mouse_scroll,
        );


    }
}