use obj::{Obj, Vertex};

use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Object {
    obj_index: usize,
    position: Vec3,
    velocity: Vec3,
}

impl Object {
    pub fn new(obj_index: usize, position: Vec3, velocity: Vec3) -> Self {
        Self {
            obj_index,
            position,
            velocity,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let gravity = Vec3::new(0.0, -10.0, 0.0);
        self.position += self.velocity * delta_time;
        self.velocity += gravity * delta_time;
    }

    pub fn obj(&self) -> &Obj<Vertex, u16> {
        self.obj
    }

    pub fn render(&self) {
    }
}
