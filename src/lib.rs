////////////////////////////////////////////////////////////////////////////////
// Author :         Jos Feenstra
// Based upon:      Doug Milfords' Rust 3D Graphics tutorials
// 
// File purpose :   Messy Entry Point.
//                  Core
//                  - ONLY this file talks to javascript.
//                  - ONLY this file recieves calls by javascript
////////////////////////////////////////////////////////////////////////////////

// ignore dead stuff when developing
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_unsafe)]

extern crate wasm_bindgen;
use std::sync::Mutex;
use scenes::Scene;
use wasm_bindgen::prelude::*; // still dont really know what prelude does
// use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use] 
extern crate lazy_static;

mod core_state;
mod shaders;
mod math;
mod geometry;
mod scenes;
mod systems;

use systems::{console, gl_common};
use systems::context;


// some messy barebones logging

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn welcome_message()
{
    console::log(&String::from("goedemorgen"));
}

#[wasm_bindgen]
pub struct Core {
    gl: GL,
    scenes: Vec<Box<dyn Scene>>,
}

// Core. Central client to manage all scenes.
// TODO: make core also deal with all global state 
#[wasm_bindgen]
impl Core {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self 
    {
        console_error_panic_hook::set_once();
        let gl = context::init_webgl_context().unwrap();    
        
        Self {
            scenes: vec![
                Box::new(scenes::Scene1::new(&gl)),
                Box::new(scenes::Scene3::new(&gl, 50))
                ],
            gl: gl,
        }
    }

    pub fn update(&mut self, time:f32, height: f32, width: f32) -> 
    Result<(), JsValue> 
    {
        core_state::update_appstate(width, height, time);
        let state = core_state::get_appstate();
        for scene in self.scenes.iter_mut() {
            scene.update(&state);
        }
        Ok(())   
    }

    pub fn draw(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let state = core_state::get_appstate();

        let oc = (state.time / 1000.).sin();
        for scene in self.scenes.iter() {
            scene.draw(&self.gl, &state);
        }
    }
}