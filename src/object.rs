use serde::Deserialize;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::{transformations, utils, vec3::Vec3};

#[derive(Debug, Deserialize)]
pub struct Object {
    pub object_type: String,

    pub position: Vec3,
    pub velocity: Vec3,
    pub radius: f32,
    pub mass: f32,

    pub color: Vec3,
    pub physics_enabled: bool,
}

impl Object {
    pub fn update_gl_uniforms(
        &self,
        gl: &WebGl2RenderingContext,
        program: &WebGlProgram,
        view_projection_matrix: [f32; 16],
    ) {
        let world_matrix = utils::matrix_multiply(
            transformations::translation(self.position.x, self.position.y, self.position.z),
            transformations::rotation_y(0.0),
        );

        let world_view_projection_matrix =
            utils::matrix_multiply(view_projection_matrix, world_matrix);

        // INVERSED FOR NORMALS
        let world_inverse_transposed_matrix = utils::transpose(utils::invert_matrix(world_matrix));

        let uniform_transformation_l = gl.get_uniform_location(&program, "u_transformation");
        let uniform_world_invserse_transposed_l =
            gl.get_uniform_location(&program, "u_world_inverse_transposed");
        let uniform_color_l = gl.get_uniform_location(&program, "u_color");

        gl.uniform4f(
            uniform_color_l.as_ref(),
            self.color.x,
            self.color.y,
            self.color.z,
            1.0,
        );

        gl.uniform_matrix4fv_with_f32_array(
            uniform_transformation_l.as_ref(),
            true,
            &world_view_projection_matrix,
        );

        gl.uniform_matrix4fv_with_f32_array(
            uniform_world_invserse_transposed_l.as_ref(),
            true,
            &world_inverse_transposed_matrix,
        );
    }

    pub fn weird(x: f32, add: f32) -> f32 {
        return x.signum() * f32::max(0.0, x.abs() + add);
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.physics_enabled {
            return;
        }

        let gravity = Vec3::new(0.0, -10.0, 0.0);
        let ground_pos = -2.0; // Temporary "GROUND"
        let friction = f32::abs(0.5 * gravity.y); // MAGITUDE OF ACCELERATION DUE TO FRICTION

        let old_position = self.position;
        let old_velocity = self.velocity;

        // friction
        if self.position.y == ground_pos && self.velocity.y == 0.0 {
            self.velocity.x = Self::weird(self.velocity.x, -friction * delta_time);
        }

        self.velocity += gravity * delta_time;

        // ADD AVERAGE VELOCITY * TIME
        self.position += (old_velocity + self.velocity) * 0.5 * delta_time;

        if self.position.y < ground_pos {
            if old_velocity.y == 0.0 {
                self.position.y = ground_pos;
                self.velocity.y = 0.0;
            } else {
                // BOUNCE
                web_sys::console::log_1(&"bounce".into());
                let a = gravity.y;
                let s = ground_pos - old_position.y;
                let u = old_velocity.y;
                let t_ground = (-u - f32::sqrt(u * u + 2.0 * a * s)) / a;
                let t_after = delta_time - t_ground;

                // LOSE ENERGY WHEN BOUNCE
                let u = u + a * t_ground;

                let e_k = self.mass * u * u * 0.5;
                let new_e_k = f32::max(e_k * 0.3 - 10.0, 0.0); // just lose some energy (x/2 - 2)
                let u = f32::sqrt(2.0 * new_e_k / self.mass);

                {
                    let a_x = -friction * 0.2 + if u == 0.0 { t_after } else { 0.0 };

                    self.position.x = old_position.x + old_velocity.x * t_ground;
                    self.velocity.x = Self::weird(old_velocity.x, a_x);

                    self.position.x += self.velocity.x * t_after;

                    self.position.z = old_position.z + old_velocity.z * t_ground;
                    self.velocity.z = Self::weird(old_velocity.z, a_x);

                    self.position.z += self.velocity.z * t_after;
                }

                {
                    if u == 0.0 {
                        self.velocity.y = 0.0;
                    } else {
                        self.velocity.y = u + a * t_after; // DONE
                    }
                    self.position.y = ground_pos + (u + self.velocity.y) * 0.5 * t_after;
                }

                if self.position.y < ground_pos {
                    // DOUBLE BOUNCE IN ONE FRAME
                    web_sys::console::error_1(&"YOOOO FIX THIS DOUBLE BOUNCE".into());
                }
            }
        }
    }

    pub fn collides(&self, obj2: &Self) -> bool {
        if !self.physics_enabled || !obj2.physics_enabled {
            return false;
        }
        let x = f32::abs(self.position.x - obj2.position.x);
        let y = f32::abs(self.position.y - obj2.position.y);
        let z = f32::abs(self.position.z - obj2.position.z);
        let distance = f32::sqrt(x * x + y * y + z * z);
        // web_sys::console::log_1(&(self.position.x - obj2.position.x).into());
        // web_sys::console::log_1(&distance.into());
        if distance < self.radius + obj2.radius {
            return true;
        }
        false
    }
}
