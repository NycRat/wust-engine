use std::io::BufReader;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub fn get_window_size() -> (f32, f32) {
    let window = web_sys::window().unwrap();
    (
        window.inner_width().unwrap().as_f64().unwrap() as f32,
        window.inner_height().unwrap().as_f64().unwrap() as f32,
    )
}

pub fn create_shader(gl: &WebGl2RenderingContext, shader_type: u32, source: &str) -> WebGlShader {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    return shader;
}

pub fn create_program(
    gl: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> WebGlProgram {
    let program = gl.create_program().unwrap();
    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);
    return program;
}

pub fn get_sphere_obj() -> obj::Obj {
    let input = BufReader::new(include_str!("./sphere.obj").as_bytes());
    obj::load_obj(input).unwrap()
}

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

pub fn invert_matrix(m: [f32; 16]) -> [f32; 16] {
    let mut inv = [0f32; 16];
    let mut res = [0f32; 16];

    let mut det;

    inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
        + m[9] * m[7] * m[14]
        + m[13] * m[6] * m[11]
        - m[13] * m[7] * m[10];

    inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
        - m[8] * m[7] * m[14]
        - m[12] * m[6] * m[11]
        + m[12] * m[7] * m[10];

    inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
        + m[8] * m[7] * m[13]
        + m[12] * m[5] * m[11]
        - m[12] * m[7] * m[9];

    inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
        - m[8] * m[6] * m[13]
        - m[12] * m[5] * m[10]
        + m[12] * m[6] * m[9];

    inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15]
        - m[9] * m[3] * m[14]
        - m[13] * m[2] * m[11]
        + m[13] * m[3] * m[10];

    inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15]
        + m[8] * m[3] * m[14]
        + m[12] * m[2] * m[11]
        - m[12] * m[3] * m[10];

    inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15]
        - m[8] * m[3] * m[13]
        - m[12] * m[1] * m[11]
        + m[12] * m[3] * m[9];

    inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14]
        + m[8] * m[2] * m[13]
        + m[12] * m[1] * m[10]
        - m[12] * m[2] * m[9];

    inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15]
        + m[5] * m[3] * m[14]
        + m[13] * m[2] * m[7]
        - m[13] * m[3] * m[6];

    inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15]
        - m[4] * m[3] * m[14]
        - m[12] * m[2] * m[7]
        + m[12] * m[3] * m[6];

    inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15]
        + m[4] * m[3] * m[13]
        + m[12] * m[1] * m[7]
        - m[12] * m[3] * m[5];

    inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14]
        - m[4] * m[2] * m[13]
        - m[12] * m[1] * m[6]
        + m[12] * m[2] * m[5];

    inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11]
        - m[5] * m[3] * m[10]
        - m[9] * m[2] * m[7]
        + m[9] * m[3] * m[6];

    inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11]
        + m[4] * m[3] * m[10]
        + m[8] * m[2] * m[7]
        - m[8] * m[3] * m[6];

    inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11]
        - m[4] * m[3] * m[9]
        - m[8] * m[1] * m[7]
        + m[8] * m[3] * m[5];

    inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10]
        + m[4] * m[2] * m[9]
        + m[8] * m[1] * m[6]
        - m[8] * m[2] * m[5];

    det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];

    if det == 0.0 {
        panic!();
    }

    det = 1.0 / det;

    for i in 0..16 {
        res[i] = inv[i] * det;
    }

    return res;
}

pub fn matrix_multiply(matrix_b: [f32; 16], matrix_a: [f32; 16]) -> [f32; 16] {
    let mut matrix_c = [0.0; 16];
    for row_a in 0..4 {
        for column_b in 0..4 {
            matrix_c[row_a * 4 + column_b] = (0..4)
                .map(|x| matrix_a[row_a * 4 + x] * matrix_b[x * 4 + column_b])
                .sum();
        }
    }
    return matrix_c;
}

pub fn cross(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn subtract_vectors(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn normalize(v: &[f32; 3]) -> [f32; 3] {
    let length = f32::sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    // make sure we don't divide by 0.
    if length > 0.00001 {
        [v[0] / length, v[1] / length, v[2] / length]
    } else {
        [0.0, 0.0, 0.0]
    }
}

pub fn transpose(m: [f32; 16]) -> [f32; 16] {
    [
        m[0], m[4], m[8], m[12], m[1], m[5], m[9], m[13], m[2], m[6], m[10], m[14], m[3], m[7],
        m[11], m[15],
    ]
}
