use crate::{cpu, ui};
use log::info;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Device {
    keyboard: EventPump,
}

const KEYMAP: [Keycode; 16] = [
    Keycode::X,
    Keycode::Num1,
    Keycode::Num2,
    Keycode::Num3,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::Z,
    Keycode::C,
    Keycode::Num4,
    Keycode::R,
    Keycode::F,
    Keycode::V,
];

impl Device {
    pub fn new(ui: &ui::UI) -> Result<Self, String> {
        Ok(Device {
            keyboard: ui.sdl.event_pump()?,
        })
    }
    pub fn deal_keyboard(&mut self, cpu: &mut cpu::Cpu) -> bool {
        for event in self.keyboard.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    info!("Key Down: {}", keycode.to_string());
                    for (i, k) in KEYMAP.iter().enumerate() {
                        if keycode == *k {
                            cpu.key[i] = 1;
                        }
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    for (i, k) in KEYMAP.iter().enumerate() {
                        if keycode == *k {
                            cpu.key[i] = 0;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }
}
