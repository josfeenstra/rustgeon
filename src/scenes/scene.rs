// Author : Jos Feenstra

use web_sys::WebGlRenderingContext;
use crate::core_state::AppState;


pub trait Scene {

    fn start(&self);
    fn update(&mut self, state: &AppState);
    fn draw(&self, gl: &WebGlRenderingContext, state: &AppState);
}

