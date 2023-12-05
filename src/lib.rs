use std::{cell::RefCell, rc::Rc};

use state::State;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub mod mouse;
pub mod objs;
pub mod state;
pub mod transformations;
pub mod utils;
pub mod object;
pub mod vec3;

const SPEED: f32 = 0.005;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = document.get_element_by_id("canvas").unwrap().dyn_into()?;
    let (size_x, size_y) = utils::get_window_size();

    // let resolution = 2000.0;
    let resolution = f64::max(size_x as f64, size_y as f64);
    let scale = window.device_pixel_ratio();

    canvas.set_width((resolution * scale) as u32);
    canvas.set_height((resolution * scale) as u32);

    let gl: WebGl2RenderingContext = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    gl.enable(WebGl2RenderingContext::CULL_FACE);
    gl.enable(WebGl2RenderingContext::DEPTH_TEST);

    let vertex_shader = utils::create_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("./shader.vert"),
    );
    let fragment_shader = utils::create_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        include_str!("./shader.frag"),
    );

    let program = utils::create_program(&gl, &vertex_shader, &fragment_shader);

    let attrib_position_l = gl.get_attrib_location(&program, "a_pos") as u32;
    let attrib_normal_l = gl.get_attrib_location(&program, "a_normal") as u32;

    let uniform_transformation_l = gl.get_uniform_location(&program, "u_transformation");
    let uniform_world_invserse_transposed_l =
        gl.get_uniform_location(&program, "u_world_inverse_transposed");
    let uniform_reverse_light_direction_l =
        gl.get_uniform_location(&program, "u_reverse_light_direction");

    let vao = gl.create_vertex_array();
    gl.bind_vertex_array(vao.as_ref());

    let vertices_buffer = gl.create_buffer();
    gl.bind_buffer(
        WebGl2RenderingContext::ARRAY_BUFFER,
        vertices_buffer.as_ref(),
    );

    let vertices_len = objs::set_positions(&gl, &cube_obj);

    gl.enable_vertex_attrib_array(attrib_position_l);
    gl.vertex_attrib_pointer_with_i32(
        attrib_position_l,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    let normal_buffer = gl.create_buffer();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, normal_buffer.as_ref());
    objs::set_normals(&gl, &cube_obj);
    gl.enable_vertex_attrib_array(attrib_normal_l);
    gl.vertex_attrib_pointer_with_i32(
        attrib_normal_l,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );

    gl.use_program(Some(&program));

    gl.uniform3f(uniform_reverse_light_direction_l.as_ref(), 0.5, 0.7, 1.0);

    gl.bind_vertex_array(vao.as_ref());

    gl.clear_color(0.2, 0.1, 0.2, 1.0);

    let state = Rc::new(RefCell::new(State::new()));

    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let mut state = state.borrow_mut();
            state.pointer_locked = document.pointer_lock_element().is_some();
        });
        window
            .add_event_listener_with_callback("pointerlockchange", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let mut state = state.borrow_mut();
            if state.pointer_locked {
                let (delta_x, delta_y) = (event.movement_x() as f32, event.movement_y() as f32);
                state.mouse.process_mouse_movement(delta_x, delta_y);
            }
        });
        window
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let mut state = state.borrow_mut();
            let key = event.key();
            state.keys_pressed.insert(key.to_lowercase());
        });
        window
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let state = state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let mut state = state.borrow_mut();
            let key = event.key();
            state.keys_pressed.remove(&key.to_lowercase());
        });
        window
            .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    {
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
            canvas.request_pointer_lock();
        });
        window
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    let update_closure = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let update_closure2 = update_closure.clone();

    {
        let gl = gl.clone();
        let state = state.clone();
        let window = window.clone();

        *update_closure2.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
            let (size_x, size_y) = utils::get_window_size();
            let mut state = state.borrow_mut();

            let now = web_sys::js_sys::Date::now();
            let delta_time = (now - state.last_tick) as f32;
            state.last_tick = now;

            state.suzanne_rotation += std::f32::consts::PI * delta_time / 1000.0 / 2.0;

            if state.pointer_locked {
                let target = state.mouse.get_target();
                if state.keys_pressed.contains("a") {
                    state.camera_position[0] -= -target[2] * SPEED * delta_time;
                    state.camera_position[2] -= target[0] * SPEED * delta_time;
                }
                if state.keys_pressed.contains("d") {
                    state.camera_position[0] += -target[2] * SPEED * delta_time;
                    state.camera_position[2] += target[0] * SPEED * delta_time;
                }
                if state.keys_pressed.contains("w") {
                    for i in 0..3 {
                        state.camera_position[i] += target[i] * SPEED * delta_time;
                    }
                }
                if state.keys_pressed.contains("s") {
                    for i in 0..3 {
                        state.camera_position[i] -= target[i] * SPEED * delta_time;
                    }
                }
            }

            let projection_matrix = transformations::perspective(
                std::f32::consts::PI / (3.0),
                size_x / size_y,
                0.5,
                200.0,
            );

            let camera_matrix = transformations::look_at(
                &state.camera_position,
                &state.mouse.get_look_at_target(&state.camera_position),
            );

            let view_matrix = utils::invert_matrix(camera_matrix);
            let view_projection_matrix = utils::matrix_multiply(projection_matrix, view_matrix);

            let world_matrix = utils::matrix_multiply(
                transformations::translation(0.0, 0.0, -5.0),
                transformations::rotation_y(state.suzanne_rotation),
            );

            let world_view_projection_matrix =
                utils::matrix_multiply(view_projection_matrix, world_matrix);

            // INVERSED FOR NORMALS
            let world_inverse_transposed_matrix =
                utils::transpose(utils::invert_matrix(world_matrix));

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

            gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertices_len);

            window
                .request_animation_frame(
                    update_closure
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap();
        }));
    }

    window
        .request_animation_frame(
            update_closure2
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )
        .unwrap();

    // outer: for v in m { 'inner: for i in v { if i < 0 { println!("Found {}", i); break 'outer; } } }

    Ok(())
}
