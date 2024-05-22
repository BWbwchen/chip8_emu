use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::cpu;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;
const BLOCK_SIZE: u32 = 10;
pub type ScreenType = [[Point; WIDTH as usize]; HEIGHT as usize];

/// Point
#[derive(Clone, Copy)]
pub struct Point {
    color: Color,
}

impl Point {
    fn new(color: Color) -> Self {
        Self { color }
    }
    pub fn set(&mut self, color: Color) {
        self.color = color;
    }
    pub fn cmp_color(&self, color: Color) -> bool {
        self.color == color
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(Color::BLACK)
    }
}

pub struct UI {
    pub sdl: Sdl,
    canvas: Canvas<Window>,
}

impl UI {
    pub fn new() -> Result<UI, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let window = video_subsystem
            .window("CHIP 8 emulator", WIDTH * BLOCK_SIZE, HEIGHT * BLOCK_SIZE)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(UI { sdl, canvas })
    }

    fn draw_point(&mut self, p: &Point, (x, y): (i32, i32)) -> Result<(), String> {
        self.canvas.set_draw_color(p.color);
        self.canvas
            .fill_rect(Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE))
            .map_err(|e| e.to_string())
    }
    pub fn refresh(&mut self, cpu: &mut cpu::Cpu) -> Result<(), String> {
        if !cpu.draw_flag {
            return Ok(());
        }

        self.canvas.clear();

        for (y, v) in cpu.get_screen().iter().enumerate() {
            for (x, p) in v.iter().enumerate() {
                let _ = self.draw_point(p, (x as i32, y as i32))?;
            }
        }

        self.canvas.present();
        cpu.draw_flag = false;
        Ok(())
    }
}
