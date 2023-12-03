use std::collections::HashSet;

use crate::mouse::Mouse;

#[derive(Debug)]
pub struct State {
    pub camera_position: [f32; 3],
    pub keys_pressed: HashSet<String>,
    pub pointer_locked: bool,
    pub mouse: Mouse,
    pub last_tick: f64,
    pub suzanne_rotation: f32,
}

impl State {
    pub fn new() -> Self {
        State {
            camera_position: [0.0, 0.0, 0.0],
            keys_pressed: HashSet::new(),
            pointer_locked: false,
            mouse: Mouse::new(1.0 / 2500.0),
            last_tick: web_sys::js_sys::Date::now(),
            suzanne_rotation: 0.0,
        }
    }
}
