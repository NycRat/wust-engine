use std::collections::HashSet;

use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{mouse::Mouse, object::Objects, objs};

#[derive(Debug)]
pub struct State {
    pub camera_position: [f32; 3],
    pub keys_pressed: HashSet<String>,
    pub pointer_locked: bool,
    pub mouse: Mouse,
    pub last_tick: f64,
    pub objects_list: Vec<Objects>,
}

impl State {
    pub fn new(gl: &WebGl2RenderingContext, program: &WebGlProgram) -> Self {
        State {
            camera_position: [0.0, 0.0, 0.0],
            keys_pressed: HashSet::new(),
            pointer_locked: false,
            mouse: Mouse::new(1.0 / 2500.0),
            last_tick: web_sys::js_sys::Date::now(),
            objects_list: vec![Objects::new(gl, program, objs::get_suzanne_obj())],
        }
    }
}
