use std::sync::Arc;

use emulator_core::{Button, Joypad};
use minifb::InputCallback;

pub struct JoypadManager {
    joypad: Arc<Joypad>,
}

impl JoypadManager {
    pub fn new(joypad: Arc<Joypad>) -> Self {
        Self { joypad }
    }
}
impl InputCallback for JoypadManager {
    fn add_char(&mut self, _uni_char: u32) {}

    fn set_key_state(&mut self, key: minifb::Key, state: bool) {
        match (key, state) {
            (minifb::Key::Enter, true) => self.joypad.button_down(Button::Start),
            (minifb::Key::Enter, false) => self.joypad.button_release(Button::Start),
            (minifb::Key::Space, true) => self.joypad.button_down(Button::Select),
            (minifb::Key::Space, false) => self.joypad.button_release(Button::Select),
            (minifb::Key::W, true) => self.joypad.button_down(Button::Up),
            (minifb::Key::W, false) => self.joypad.button_release(Button::Up),
            (minifb::Key::A, true) => self.joypad.button_down(Button::Left),
            (minifb::Key::A, false) => self.joypad.button_release(Button::Left),
            (minifb::Key::S, true) => self.joypad.button_down(Button::Down),
            (minifb::Key::S, false) => self.joypad.button_release(Button::Down),
            (minifb::Key::D, true) => self.joypad.button_down(Button::Right),
            (minifb::Key::D, false) => self.joypad.button_release(Button::Right),
            (minifb::Key::Z, true) => self.joypad.button_down(Button::A),
            (minifb::Key::Z, false) => self.joypad.button_release(Button::A),
            (minifb::Key::X, true) => self.joypad.button_down(Button::B),
            (minifb::Key::X, false) => self.joypad.button_release(Button::B),
            (_, _) => {}
        }
    }
}
