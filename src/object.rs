use obj::Obj;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject};

use crate::{objs, transformations, utils, vec3::Vec3};

#[derive(Debug)]
pub struct Object {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Object {
    pub fn update(&mut self, delta_time: f32) {
        let gravity = Vec3::new(0.0, -1.0, 0.0);
        self.position += self.velocity * delta_time;
        self.velocity += gravity * delta_time;
        if self.position.y < -2.0 {
            self.position.y = -2.0;
        }
    }
}

#[derive(Debug)]
pub struct Objects {
    // pub obj_data: Obj,
    pub vertices_len: i32,
    pub vao: Option<WebGlVertexArrayObject>,
    pub objects: Vec<Object>,
}

impl Objects {
    pub fn new(
        gl: &WebGl2RenderingContext,
        program: &WebGlProgram,
        obj_data: Obj,
    ) -> Self {
        let position_location = gl.get_attrib_location(program, "a_pos") as u32;
        let normal_location = gl.get_attrib_location(program, "a_normal") as u32;

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(vao.as_ref());

        let vertices_buffer = gl.create_buffer();
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            vertices_buffer.as_ref(),
        );

        let vertices_len = objs::set_positions(gl, &obj_data);

        gl.enable_vertex_attrib_array(position_location);
        gl.vertex_attrib_pointer_with_i32(
            position_location,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        let normal_buffer = gl.create_buffer();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, normal_buffer.as_ref());
        objs::set_normals(gl, &obj_data);

        gl.enable_vertex_attrib_array(normal_location);
        gl.vertex_attrib_pointer_with_i32(
            normal_location,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        Self {
            // obj_data,
            vertices_len,
            vao,
            objects: vec![Object {
                position: [0.0, 0.0, -3.0].into(),
                velocity: [0.0, 0.0, 0.0].into(),
            }],
            // position: position.into(),
            // velocity: velocity.into(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for object in &mut self.objects {
            object.update(delta_time);
        }
        // let gravity = Vec3::new(0.0, -10.0, 0.0);
        // self.position += self.velocity * delta_time;
        // self.velocity += gravity * delta_time;
    }

    pub fn render(
        &self,
        gl: &WebGl2RenderingContext,
        program: &WebGlProgram,
        view_matrix: [f32; 16],
    ) {
        let (size_x, size_y) = utils::get_window_size();
        let projection_matrix =
            transformations::perspective(std::f32::consts::PI / (3.0), size_x / size_y, 0.5, 200.0);

        let view_projection_matrix = utils::matrix_multiply(projection_matrix, view_matrix);

        for object in &self.objects {
            let world_matrix = utils::matrix_multiply(
                transformations::translation(
                    object.position.x,
                    object.position.y,
                    object.position.z,
                ),
                transformations::rotation_y(1.0),
            );

            let world_view_projection_matrix =
                utils::matrix_multiply(view_projection_matrix, world_matrix);

            // INVERSED FOR NORMALS
            let world_inverse_transposed_matrix =
                utils::transpose(utils::invert_matrix(world_matrix));
            let uniform_transformation_l = gl.get_uniform_location(&program, "u_transformation");
            let uniform_world_invserse_transposed_l =
                gl.get_uniform_location(&program, "u_world_inverse_transposed");

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

            gl.bind_vertex_array(self.vao.as_ref());
            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, self.vertices_len);
        }
    }
}
