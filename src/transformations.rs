use crate::utils;

pub fn translation(x: f32, y: f32, z: f32) -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
    ]
}

pub fn rotation_x(degrees: f32) -> [f32; 16] {
    let c = f32::cos(degrees);
    let s = f32::sin(degrees);
    [
        1.0, 0.0, 0.0, 0.0, 0.0, c, s, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn rotation_y(degrees: f32) -> [f32; 16] {
    let c = f32::cos(degrees);
    let s = f32::sin(degrees);
    [
        c, 0.0, -s, 0.0, 0.0, 1.0, 0.0, 0.0, s, 0.0, c, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn rotation_z(degrees: f32) -> [f32; 16] {
    let c = f32::cos(degrees);
    let s = f32::sin(degrees);
    [
        c, -s, 0.0, 0.0, s, c, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn scaling(x: f32, y: f32, z: f32) -> [f32; 16] {
    [
        x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

pub fn projection(x: f32, y: f32, z: f32) -> [f32; 16] {
    [
        2.0 / x,
        0.0,
        0.0,
        0.0,
        0.0,
        -2.0 / y,
        0.0,
        0.0,
        0.0,
        0.0,
        2.0 / z,
        0.0,
        -1.0,
        1.0,
        0.0,
        1.0,
    ]
}

pub fn perspective(field_of_view_in_radians: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let f = f32::tan(std::f32::consts::PI * 0.5 - 0.5 * field_of_view_in_radians);
    let range_inv = 1.0 / (near - far);
    [
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        (near + far) * range_inv,
        -1.0,
        0.0,
        0.0,
        near * far * range_inv * 2.0,
        0.0,
    ]
}

pub fn look_at(camera_position: &[f32; 3], target: &[f32; 3]) -> [f32; 16] {
    let up = [0.0, 1.0, 0.0];
    let z_axis = utils::normalize(&utils::subtract_vectors(camera_position, target));
    let x_axis = utils::normalize(&utils::cross(&up, &z_axis));
    let y_axis = utils::normalize(&utils::cross(&z_axis, &x_axis));
    [
        x_axis[0],
        x_axis[1],
        x_axis[2],
        0.0,
        y_axis[0],
        y_axis[1],
        y_axis[2],
        0.0,
        z_axis[0],
        z_axis[1],
        z_axis[2],
        0.0,
        camera_position[0],
        camera_position[1],
        camera_position[2],
        1.0,
    ]
}
