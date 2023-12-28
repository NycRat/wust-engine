use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram};

use crate::{object::Object, state::State, transformations, utils};

const SPEED: f32 = 8.0;

pub struct Application;

impl Application {
    pub fn init() -> Result<(), JsValue> {
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

        let uniform_reverse_light_direction_l =
            gl.get_uniform_location(&program, "u_reverse_light_direction");

        let state = Rc::new(RefCell::new(State::new(&gl, &program)));

        gl.use_program(Some(&program));
        gl.uniform3f(uniform_reverse_light_direction_l.as_ref(), 0.5, 0.7, 1.0);
        // gl.clear_color(0.2, 0.1, 0.2, 1.0);
        gl.clear_color(0.4, 0.4, 0.7, 1.0);

        {
            let state = state.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                let mut state = state.borrow_mut();
                state.pointer_locked = document.pointer_lock_element().is_some();
            });
            window
                .add_event_listener_with_callback(
                    "pointerlockchange",
                    closure.as_ref().unchecked_ref(),
                )
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

        {
            let state = state.clone();
            let closure = Closure::<dyn FnMut()>::new(move || {
                let mut state = state.borrow_mut();

                let now = web_sys::js_sys::Date::now();
                let delta_time = (now - state.last_tick) as f32 / 1000.0;
                state.last_tick = now;

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
                    if state.keys_pressed.contains(" ") {
                        state.camera_position[1] += SPEED * delta_time;
                    }
                    if state.keys_pressed.contains("shift") {
                        state.camera_position[1] -= SPEED * delta_time;
                    }
                }
                if state.keys_pressed.contains("r") {
                    state.reset();
                }

                if state.camera_position[1] < -2.99 {
                    state.camera_position[1] = -2.99;
                }

                // web_sys::console::log_1(&format!("{state:?}").into());

                Self::update(&mut state, delta_time);
            });
            window
                .set_interval_with_callback(closure.as_ref().unchecked_ref())
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
                let state = state.borrow();

                Self::render(&gl, &program, &state);

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

        Ok(())
    }

    fn update(state: &mut State, delta_time: f32) {
        if delta_time > 0.1 {
            web_sys::console::error_1(
                &format!("DELTA_TIME: {delta_time}s // DELTA_TIME TOO LARGE").into(),
            );
            return;
        }
        for object in &mut state.objects {
            object.update(delta_time);
        }
        for i in 0..state.objects.len() {
            for j in i + 1..state.objects.len() {
                if Object::collides(&state.objects[i], &state.objects[j]) {
                    // web_sys::console::log_1(&"haha".into());
                    state.objects[i].velocity.x *= -1.0;
                    state.objects[j].velocity.x *= -1.0;
                }
            }
        }
    }

    fn render(gl: &WebGl2RenderingContext, program: &WebGlProgram, state: &State) {
        let camera_matrix = transformations::look_at(
            &state.camera_position.into(),
            &state.mouse.get_look_at_target(&state.camera_position.into()),
        );

        let view_matrix = utils::invert_matrix(camera_matrix);
        let (size_x, size_y) = utils::get_window_size();
        let projection_matrix = transformations::perspective(
            std::f32::consts::PI / (3.0),
            size_x / size_y,
            0.01,
            200.0,
        );
        let view_projection_matrix = utils::matrix_multiply(projection_matrix, view_matrix);

        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        for object in &state.objects {
            let object_type = &state.object_types[&object.object_type];
            gl.bind_vertex_array(object_type.vao.as_ref());

            object.update_gl_uniforms(gl, program, view_projection_matrix);
            gl.draw_arrays(
                WebGl2RenderingContext::TRIANGLES,
                0,
                object_type.vertices_len,
            );
        }
    }
}
