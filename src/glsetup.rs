use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

pub fn init_webgl_context() -> Result<WebGlRenderingContext, JsValue> 
{
    // get canvas & gl, deal with dynamic types
    // TODO : error handling
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("wasmcanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    // configure
    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.);

    Ok(gl)
}