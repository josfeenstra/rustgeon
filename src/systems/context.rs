use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::WebAssembly;

use super::console;

pub fn init_webgl_context() -> Result<WebGlRenderingContext, JsValue> 
{
    // get canvas & gl, deal with dynamic types
    // TODO : error handling
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("wasmcanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    // add all handlers
    // TODO: add keyboard handlers in the same fashion
    attach_mouse_down_handler(&canvas)?;
    attach_mouse_up_handler(&canvas)?;
    attach_mouse_move_handler(&canvas)?;
    attach_mouse_scroll_handler(&canvas)?;
    attach_key_down_handler(&canvas)?;
    attach_key_up_handler(&canvas)?;

    // configure
    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.);

    Ok(gl)
}

// NOTE : this looks redundant, but it should be like this

fn attach_mouse_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        super::super::core_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, true);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        super::super::core_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, false);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_move_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        super::super::core_state::update_mouse_position(event.client_x() as f32, event.client_y() as f32);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_mouse_scroll_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    
    let handler = move |event: web_sys::WheelEvent| {
        super::super::core_state::update_mouse_scroll(event.delta_y() as f32);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("wheel", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

fn attach_key_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    
    let handler = move |event: web_sys::KeyboardEvent| {
        super::super::core_state::update_key(event.key(), false);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}


fn attach_key_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    
    let handler = move |event: web_sys::KeyboardEvent| {
        super::super::core_state::update_key(event.key(), true);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("keyup", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}

