use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, Window, WindowOptions};

use crate::render::{Framebuffer, HEIGHT, WIDTH};

pub struct Platform {
    window: Window,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InputSnapshot {
    pub keys_pressed: Vec<Key>,
    pub mouse_position: Option<(f32, f32)>,
    pub left_button_down: bool,
}

impl Platform {
    pub fn new(title: &str) -> Result<Self, minifb::Error> {
        let mut window = Window::new(
            title,
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        )?;
        window.set_target_fps(60);
        Ok(Self { window })
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn input(&self) -> InputSnapshot {
        InputSnapshot {
            keys_pressed: self.window.get_keys_pressed(KeyRepeat::No),
            mouse_position: self.window.get_mouse_pos(MouseMode::Clamp),
            left_button_down: self.window.get_mouse_down(MouseButton::Left),
        }
    }

    pub fn present(&mut self, framebuffer: &Framebuffer) -> Result<(), minifb::Error> {
        self.window
            .update_with_buffer(framebuffer.pixels(), WIDTH, HEIGHT)
    }
}
