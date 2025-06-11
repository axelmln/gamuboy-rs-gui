use std::sync::mpsc::Receiver;

use gamuboy::{
    joypad::{self, Joypad},
    joypad_events_handler::EventsHandler,
};
use sdl2::{
    controller::{Axis, Button, GameController},
    event::Event,
    keyboard::Keycode,
};

pub struct SdlEventsHandler {
    _controller: Option<GameController>,
}

impl SdlEventsHandler {
    pub fn new(controller: Option<GameController>) -> Self {
        Self {
            _controller: controller,
        }
    }
}

impl EventsHandler<sdl2::event::Event> for SdlEventsHandler {
    fn handle_events(&mut self, rx: &Receiver<sdl2::event::Event>, joypad: &mut Joypad) {
        let joypad_events: Vec<_> = rx.try_iter().collect();
        for evt in joypad_events {
            match evt {
                // Keyboard
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => match key {
                    Keycode::Up => joypad.update(joypad::Button::Up, true),
                    Keycode::Down => joypad.update(joypad::Button::Down, true),
                    Keycode::Left => joypad.update(joypad::Button::Left, true),
                    Keycode::Right => joypad.update(joypad::Button::Right, true),
                    Keycode::A => joypad.update(joypad::Button::A, true),
                    Keycode::Z => joypad.update(joypad::Button::B, true),
                    Keycode::Return | Keycode::KpEnter | Keycode::Return2 => {
                        joypad.update(joypad::Button::Start, true)
                    }
                    Keycode::Tab => joypad.update(joypad::Button::Select, true),
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => match key {
                    Keycode::Up => joypad.update(joypad::Button::Up, false),
                    Keycode::Down => joypad.update(joypad::Button::Down, false),
                    Keycode::Left => joypad.update(joypad::Button::Left, false),
                    Keycode::Right => joypad.update(joypad::Button::Right, false),
                    Keycode::A => joypad.update(joypad::Button::A, false),
                    Keycode::Z => joypad.update(joypad::Button::B, false),
                    Keycode::Return | Keycode::KpEnter | Keycode::Return2 => {
                        joypad.update(joypad::Button::Start, false)
                    }
                    Keycode::Tab => joypad.update(joypad::Button::Select, false),
                    _ => {}
                },

                // Controller
                Event::ControllerAxisMotion {
                    axis, value: val, ..
                } => {
                    // Axis motion is an absolute value in the range [-32768, 32767]
                    let dead_zone = 5000;
                    let is_trigged = val > dead_zone || val < -dead_zone;
                    match axis {
                        Axis::LeftX => {
                            joypad.update(joypad::Button::Right, is_trigged && val > 0);
                            joypad.update(joypad::Button::Left, is_trigged && val < 0);
                        }
                        Axis::LeftY => {
                            joypad.update(joypad::Button::Down, is_trigged && val > 0);
                            joypad.update(joypad::Button::Up, is_trigged && val < 0);
                        }
                        _ => {}
                    }
                }
                Event::ControllerButtonDown { button, .. } => match button {
                    Button::DPadUp => joypad.update(joypad::Button::Up, true),
                    Button::DPadDown => joypad.update(joypad::Button::Down, true),
                    Button::DPadLeft => joypad.update(joypad::Button::Left, true),
                    Button::DPadRight => joypad.update(joypad::Button::Right, true),
                    Button::A => joypad.update(joypad::Button::A, true),
                    Button::B => joypad.update(joypad::Button::B, true),
                    Button::Start => joypad.update(joypad::Button::Start, true),
                    Button::Back => joypad.update(joypad::Button::Select, true),
                    _ => {}
                },
                Event::ControllerButtonUp { button, .. } => match button {
                    Button::DPadUp => joypad.update(joypad::Button::Up, false),
                    Button::DPadDown => joypad.update(joypad::Button::Down, false),
                    Button::DPadLeft => joypad.update(joypad::Button::Left, false),
                    Button::DPadRight => joypad.update(joypad::Button::Right, false),
                    Button::A => joypad.update(joypad::Button::A, false),
                    Button::B => joypad.update(joypad::Button::B, false),
                    Button::Start => joypad.update(joypad::Button::Start, false),
                    Button::Back => joypad.update(joypad::Button::Select, false),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
