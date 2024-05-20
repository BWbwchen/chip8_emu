use crate::{cpu, ui};
use log::info;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Device {
    keyboard: EventPump,
}

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
                }
                _ => {}
            }
        }
        false
    }
}
