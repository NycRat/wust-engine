use obj::Obj;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject};

use crate::utils;

#[derive(Debug)]
pub struct ObjectType {
    pub vertices_len: i32,
    pub vao: Option<WebGlVertexArrayObject>,
}

impl ObjectType {
    pub fn new(gl: &WebGl2RenderingContext, program: &WebGlProgram, obj_data: Obj) -> Self {
        let position_location = gl.get_attrib_location(program, "a_pos") as u32;
        let normal_location = gl.get_attrib_location(program, "a_normal") as u32;

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(vao.as_ref());

        let vertices_buffer = gl.create_buffer();
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            vertices_buffer.as_ref(),
        );

        let vertices_len = utils::set_positions(gl, &obj_data);

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
        utils::set_normals(gl, &obj_data);

        gl.enable_vertex_attrib_array(normal_location);
        gl.vertex_attrib_pointer_with_i32(
            normal_location,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        Self { vertices_len, vao }
    }
}
