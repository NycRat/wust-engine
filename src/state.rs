use std::collections::HashSet;

use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{
    mouse::Mouse,
    object::{Object, Objects},
    objs,
};

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
            camera_position: [0.5, -1.8, 4.2],
            keys_pressed: HashSet::new(),
            pointer_locked: false,
            mouse: Mouse::new(1.0 / 2500.0),
            last_tick: web_sys::js_sys::Date::now(),
            objects_list: vec![
                Objects::new(
                    gl,
                    program,
                    objs::get_suzanne_obj(),
                    vec![
                        Object {
                            position: [-5.0, 0.0, -3.0].into(),
                            velocity: [3.0, 1.0, 2.0].into(),
                        },
                        Object {
                            position: [2.0, 1.0, -3.0].into(),
                            velocity: [0.0, 0.0, 0.0].into(),
                        },
                    ],
                ),
                Objects::new(
                    gl,
                    program,
                    objs::get_cube_obj(),
                    vec![
                        Object {
                            position: [-8.0, 3.0, -7.0].into(),
                            velocity: [3.0, 0.0, 2.0].into(),
                        },
                        Object {
                            position: [-2.0, 0.0, -5.0].into(),
                            velocity: [0.0, 0.0, 0.0].into(),
                        },
                    ],
                ),
            ],
        }
    }
}
