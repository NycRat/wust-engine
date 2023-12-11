use std::io::BufReader;

use web_sys::WebGl2RenderingContext;

pub fn get_cube_obj() -> obj::Obj {
    let input = BufReader::new(include_str!("./cube.obj").as_bytes());
    obj::load_obj(input).unwrap()
}

pub fn get_suzanne_obj() -> obj::Obj {
    let input = BufReader::new(include_str!("./suzanne.obj").as_bytes());
    obj::load_obj(input).unwrap()
}

pub fn get_ground_obj() -> obj::Obj {
    let input = BufReader::new(include_str!("./ground.obj").as_bytes());
    obj::load_obj(input).unwrap()
}

pub fn set_positions(gl: &WebGl2RenderingContext, obj: &obj::Obj) -> i32 {
    unsafe {
        let positions: Vec<f32> = obj
            .indices
            .iter()
            .map(|i| obj.vertices[*i as usize].position)
            .flat_map(|x| x)
            .collect();

        let array_buf_view = js_sys::Float32Array::view(&positions);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        positions.len() as i32 / 3
    }
}

pub fn set_normals(gl: &WebGl2RenderingContext, obj: &obj::Obj) -> i32 {
    unsafe {
        let normals: Vec<f32> = obj
            .indices
            .iter()
            .map(|i| obj.vertices[*i as usize].normal)
            .flat_map(|x| x)
            .collect();

        let array_buf_view = js_sys::Float32Array::view(&normals);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        normals.len() as i32
    }
}
