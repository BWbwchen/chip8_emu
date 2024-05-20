use chip8_emu::{cpu, devices, ui};
use clap::Parser;
use log::info;
use std::time::Duration;

use env_logger;
/// Chip8-rs is an emulator of CHIP8
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Rom path
    #[arg(short, long)]
    rom_path: String,
}

pub fn main() -> Result<(), String> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let args = Args::parse();
    let mut ui = ui::UI::new()?;
    let mut cpu = cpu::Cpu::new();
    let mut devices = devices::Device::new(&ui)?;
    info!("Rom path: {}", args.rom_path);

    loop {
        // cpu.tick()?;
        ui.refresh(cpu.get_screen())?;
        let shutdown = devices.deal_keyboard(&mut cpu);

        if shutdown {
            break;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
