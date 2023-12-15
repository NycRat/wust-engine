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

    pub fn update(&mut self, delta_time: f32) {
        if !self.physics_enabled {
            return;
        }
        let gravity = Vec3::new(0.0, -10.0, 0.0);
        let force_of_friction = f32::abs(1.0 * gravity.y);
        self.position += self.velocity * delta_time;
        self.velocity += gravity * delta_time;

        // Temporary "GROUND"
        if self.position.y <= -2.0 {
            self.position.y = -2.0;

            self.velocity.x = self.velocity.x.signum()
                * f32::max(0.0, self.velocity.x.abs() - force_of_friction * delta_time * 2.0);

            self.velocity.z = self.velocity.z.signum()
                * f32::max(0.0, self.velocity.z.abs() - force_of_friction * delta_time * 2.0);

            // BOUNCE
            self.velocity.y *= -0.4;
            // self.velocity.y *= -0.0;

            // SPEED TOO LITTLE JUST STOP IT
            if self.velocity.y.abs() < 0.5 {
                self.velocity.y = 0.0;
                self.velocity.x = self.velocity.x.signum()
                    * f32::max(0.0, self.velocity.x.abs() - force_of_friction * delta_time);

                self.velocity.z = self.velocity.z.signum()
                    * f32::max(0.0, self.velocity.z.abs() - force_of_friction * delta_time);
            }
        } else {
            self.velocity.x = self.velocity.x.signum()
                * f32::max(
                    0.0,
                    self.velocity.x.abs() - force_of_friction * delta_time / 100.0,
                );

            self.velocity.z = self.velocity.z.signum()
                * f32::max(
                    0.0,
                    self.velocity.z.abs() - force_of_friction * delta_time / 100.0,
                );
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
