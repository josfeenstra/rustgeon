////////////////////////////////////////////////////////////////////////////////
// Author :         Jos Feenstra
// Based upon:      Doug Milfords' Rust 3D Graphics tutorials
// 
// File purpose :   Static Global Values.
////////////////////////////////////////////////////////////////////////////////
use std::sync::Arc;
use std::sync::Mutex;

use crate::systems::console;



lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = 
        Mutex::new(Arc::new(AppState::new()));
}

pub fn update_appstate(canvas_width: f32, canvas_height: f32, time: f32)
{
    let min_height_width = canvas_height.min(canvas_width);
    let size = 0.9 * min_height_width;
    let hds = size / 2.;
    let hwidth = canvas_width / 2.;
    let hheight = canvas_height / 2.;

    let previous_total_time = get_appstate().total_time;

    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new(
        AppState {
            canvas_height: canvas_height, 
            canvas_width: canvas_width, 

            border_bottom: hheight - hds,
            border_top: hheight + hds,
            border_left: hwidth - hds,
            border_right: hwidth + hds,

            time: time,
            total_time: previous_total_time + time,
            ..*data.clone()
        }
    );
}

pub fn get_appstate() -> Arc<AppState>
{
    APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,

    pub border_bottom: f32,
    pub border_top: f32,
    pub border_left: f32,
    pub border_right: f32,

    pub time: f32,
    pub total_time: f32,

    pub mouse_down: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_scroll: f32,

    pub cam_rotation_x: f32,
    pub cam_rotation_y: f32,
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas_height: 0.,
            canvas_width: 0.,
            time: 0.,
            total_time: 0.,
            
            border_bottom: 0.,
            border_top: 0.,
            border_left: 0.,
            border_right: 0.,

            mouse_down: false,
            mouse_x: 0.,
            mouse_y: 0.,
            mouse_scroll: 0.,
        
            cam_rotation_x: 0.5,
            cam_rotation_y: 0.5,
        }
    }
}

// update mouse data
pub fn update_mouse_down(x: f32, y: f32, is_down: bool)
{
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_down: is_down,
        mouse_x: x,
        mouse_y: data.canvas_height - y,
        ..*data.clone()
    });
}

// update camera rotation
pub fn update_mouse_position(x: f32, y: f32) {
    let mut data = APP_STATE.lock().unwrap();

    let inv_y = data.canvas_height - y;

    let dx = x - data.mouse_x;
    let dy = inv_y - data.mouse_y;

    let drotx = if data.mouse_down {
        std::f32::consts::PI * dy / data.canvas_height
    } else {
        0.
    };

    let droty = if data.mouse_down {
        std::f32::consts::PI * dx / data.canvas_width
    } else {
        0.
    };

    *data = Arc::new(AppState {
        mouse_x: x,
        mouse_y: data.canvas_height - y,

        cam_rotation_x: data.cam_rotation_x + drotx,
        cam_rotation_y: data.cam_rotation_y - droty,

        ..*data.clone()
    });
}

pub fn update_mouse_scroll(delta: f32) {
    let mut data = APP_STATE.lock().unwrap();

    // console::log(&data.mouse_scroll.to_string());

    *data = Arc::new(AppState {
        mouse_scroll: data.mouse_scroll + delta,
        ..*data.clone()
    });
}

pub fn update_key(keyname: String, down: bool) {

    let mut data = APP_STATE.lock().unwrap();

    console::log(&keyname);

    *data = Arc::new(AppState {
        ..*data.clone()
    });
}