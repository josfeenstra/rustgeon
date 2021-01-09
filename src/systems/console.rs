use std::{collections::HashMap};
use std::sync::Mutex;

use crate::log as js_log;

lazy_static! {
    static ref LOGMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new()); 
}

pub fn log(message: &String)
{
    unsafe {
        js_log(message.as_str());
    }
}

pub fn log_str(message: &str)
{
    unsafe {
        js_log(message);
    }
}

pub fn log_once(message: &String, key: &str)
{
    let mut map = LOGMAP.lock().unwrap();
    if !map.contains_key(key) {
        map.insert(key.to_string(), "".to_string());
        log(&message);    
    }
}