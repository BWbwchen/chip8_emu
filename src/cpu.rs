use std::{fs::File, io::Read};

use log::info;
use sdl2::pixels::Color;

use crate::{
    alu,
    ui::{Point, HEIGHT},
    ui::{ScreenType, WIDTH},
};

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu {
    // 4K memory
    pub(crate) memory: [u8; 4096],
    pub(crate) reg: [u8; 16],
    // keyboard
    pub(crate) key: [u8; 16],
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,
    pub(crate) graph: ScreenType,

    // program counter
    pub(crate) pc: u16,
    pub(crate) stack: [u16; 16],
    // stack pointer
    pub(crate) sp: u16,
    // index pointer
    pub(crate) i: u16,

    pub(crate) draw_flag: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0_u8; 4096],
            reg: [0_u8; 16],
            key: [0_u8; 16],
            delay_timer: 0,
            sound_timer: 0,
            graph: [[Point::default(); WIDTH as usize]; HEIGHT as usize],

            pc: 0x200,
            stack: [0_u16; 16],
            sp: 0,
            i: 0,

            draw_flag: false,
        }
    }
    pub fn tick(&mut self) -> Result<(), String> {
        let opcode = self.fetch().ok_or("Fetch Error")?;
        info!("Opcode : {:#X}", opcode);

        let _ = alu::execute(opcode, self)?;
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        Ok(())
    }
    pub fn clear_screen(&mut self) {
        for r in self.graph.iter_mut() {
            for p in r.iter_mut() {
                p.set(Color::BLACK);
            }
        }
    }
    pub fn get_screen(&self) -> &ScreenType {
        &self.graph
    }
    fn fetch(&mut self) -> Option<u16> {
        Some(
            ((*self.memory.get(self.pc as usize)? as u16) << 8)
                | (*self.memory.get(self.pc as usize + 1)? as u16),
        )
    }
    pub fn load_rom(&mut self, rom_path: String) -> Result<(), String> {
        for (i, &byte) in FONT_SET.iter().enumerate() {
            self.memory[i] = byte;
        }

        let mut file = File::open(rom_path).map_err(|e| e.to_string())?;
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).map_err(|e| e.to_string())?;

        for (i, &byte) in buf.iter().enumerate() {
            self.memory[i + 512] = byte;
        }
        Ok(())
    }
}
