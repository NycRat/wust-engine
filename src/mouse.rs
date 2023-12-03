#[derive(Debug)]
pub struct Mouse {
    sensitivity: f32,
    yaw: f32,
    pitch: f32,
    target: [f32; 3],
}

impl Mouse {
    pub fn new(sensitivity: f32) -> Self {
        Mouse {
            sensitivity,
            yaw: 0.0,
            pitch: 0.0,
            target: [0.0, 0.0, -1.0],
        }
    }

    pub fn get_look_at_target(&self, camera_position: &[f32; 3]) -> [f32; 3] {
        [
            camera_position[0] + self.target[0],
            camera_position[1] + self.target[1],
            camera_position[2] + self.target[2],
        ]
    }

    pub fn get_target(&self) -> [f32; 3] {
        self.target
    }

    pub fn process_mouse_movement(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw -= delta_x * self.sensitivity;
        self.pitch -= delta_y * self.sensitivity;
        if self.pitch > std::f32::consts::PI / 2.0 {
            self.pitch = std::f32::consts::PI / 2.0 - 0.001;
        }
        if self.pitch < -std::f32::consts::PI / 2.0 {
            self.pitch = -std::f32::consts::PI / 2.0 + 0.001;
        }
        self.update_target();
    }

    fn update_target(&mut self) {
        self.target[0] = -f32::sin(self.yaw) * f32::cos(self.pitch);
        self.target[1] = f32::sin(self.pitch);
        self.target[2] = -f32::cos(self.yaw) * f32::cos(self.pitch);
    }
}
