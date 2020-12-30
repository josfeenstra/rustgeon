use std::sync::Arc;
use std::sync::Mutex;

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
        }
    }
}