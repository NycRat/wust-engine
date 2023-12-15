use std::collections::{HashMap, HashSet};

use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{mouse::Mouse, object::Object, object_type::ObjectType, utils};

#[derive(Debug)]
pub struct State {
    pub camera_position: [f32; 3],
    pub keys_pressed: HashSet<String>,
    pub pointer_locked: bool,
    pub mouse: Mouse,
    pub last_tick: f64,
    pub object_types: HashMap<String, ObjectType>,
    pub objects: Vec<Object>,
}

impl State {
    pub fn new(gl: &WebGl2RenderingContext, program: &WebGlProgram) -> Self {
        State {
            camera_position: [0.5, -1.8, 4.2],
            keys_pressed: HashSet::new(),
            pointer_locked: false,
            mouse: Mouse::new(1.0 / 2500.0),
            last_tick: web_sys::js_sys::Date::now(),
            object_types: [
                (
                    "suzanne".into(),
                    ObjectType::new(gl, program, utils::get_suzanne_obj()),
                ),
                (
                    "cube".into(),
                    ObjectType::new(gl, program, utils::get_cube_obj()),
                ),
                (
                    "sphere".into(),
                    ObjectType::new(gl, program, utils::get_sphere_obj()),
                ),
                (
                    "ground".into(),
                    ObjectType::new(gl, program, utils::get_ground_obj()),
                ),
            ]
            .into(),
            objects: serde_json::from_str(include_str!("objects.json")).unwrap(),
        }
    }
    pub fn reset(&mut self) {
        self.objects = serde_json::from_str(include_str!("objects.json")).unwrap();
    }
}
