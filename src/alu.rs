use sdl2::pixels::Color;

use crate::cpu;

pub fn execute(opcode: u16, cpu: &mut cpu::Cpu) -> Result<(), String> {
    match opcode & 0xF000 {
        0x0000 => {
            if opcode == 0x00E0 {
                // CLS
                cpu.clear_screen();
                cpu.draw_flag = true;
                cpu.pc += 2;
            } else if opcode == 0x00EE {
                // RET
                // TODO: don't know why add 2
                // and stack pointer's position
                cpu.pc -= 1;
                cpu.pc = cpu.stack[cpu.sp as usize];
                cpu.pc += 2;
            } else {
                return Err(format!("Unknown code: {:#X}", opcode));
            }
        }
        0x1000 => {
            // JP addr
            cpu.pc = opcode & 0x0FFF;
        }
        0x2000 => {
            // CALL addr
            // TODO: stack pointer
            cpu.stack[cpu.sp as usize] = cpu.pc;
            cpu.sp += 1;
            cpu.pc = opcode & 0x0FFF;
        }
        0x3000 => {
            // 3xkk
            // SE Vx, byte
            let x = ((opcode & 0x0F00) >> 8) as usize;
            let byte = (opcode & 0x00FF) as u8;
            if cpu.reg[x] == byte {
                cpu.pc += 4;
            } else {
                cpu.pc += 2;
            }
        }
        0x4000 => {
            // 4xkk
            // SNE Vx, byte
            let x = ((opcode & 0x0F00) >> 8) as usize;
            let byte = (opcode & 0x00FF) as u8;
            if cpu.reg[x] != byte {
                cpu.pc += 4;
            } else {
                cpu.pc += 2;
            }
        }
        0x5000 => {
            // 5xy0
            // SE Vx, Vy
            let x = ((opcode & 0x0F00) >> 8) as usize;
            let y = ((opcode & 0x00F0) >> 4) as usize;
            if cpu.reg[x] == cpu.reg[y] {
                cpu.pc += 4;
            } else {
                cpu.pc += 2;
            }
        }
        0x6000 => {
            // 6xkk
            // LD Vx, byte
            let x = ((opcode & 0xF00) >> 8) as usize;
            let byte = (opcode & 0x00FF) as u8;
            cpu.reg[x] = byte;
            cpu.pc += 2;
        }
        0x7000 => {
            // 7xkk
            // ADD Vx, byte
            let x = ((opcode & 0xF00) >> 8) as usize;
            let byte = (opcode & 0x00FF) as u8;
            cpu.reg[x] += byte;
            cpu.pc += 2;
        }
        0x8000 => {
            let x = ((opcode & 0x0F00) >> 8) as usize;
            let y = ((opcode & 0x00F0) >> 4) as usize;
            match opcode & 0x000F {
                0x0 => {
                    // 8xy0 - LD Vx, Vy
                    cpu.reg[x] = cpu.reg[y];
                    cpu.pc += 2;
                }
                0x1 => {
                    // 8xy1 - OR Vx, Vy
                    cpu.reg[x] |= cpu.reg[y];
                    cpu.pc += 2;
                }
                0x2 => {
                    // 8xy2 - AND Vx, Vy
                    cpu.reg[x] &= cpu.reg[y];
                    cpu.pc += 2;
                }
                0x3 => {
                    // 8xy3 - XOR Vx, Vy
                    cpu.reg[x] ^= cpu.reg[y];
                    cpu.pc += 2;
                }
                0x4 => {
                    // 8xy4 - ADD Vx, Vy
                    if cpu.reg[x] > (0xFF - cpu.reg[y]) {
                        cpu.reg[0xF] = 1;
                    } else {
                        cpu.reg[0xF] = 0;
                    }
                    cpu.reg[x] += cpu.reg[y];
                    cpu.pc += 2;
                }
                0x5 => {
                    // 8xy5 - SUB Vx, Vy
                    if cpu.reg[x] > cpu.reg[y] {
                        cpu.reg[0xF] = 1;
                    } else {
                        cpu.reg[0xF] = 0;
                    }
                    cpu.reg[x] -= cpu.reg[y];
                    cpu.pc += 2;
                }
                0x6 => {
                    // 8xy6 - SHR Vx {, Vy}
                    if (cpu.reg[x] & 0x01) > 0 {
                        cpu.reg[0xF] = 1;
                    } else {
                        cpu.reg[0xF] = 0;
                    }
                    cpu.reg[x] >>= 1;
                    cpu.pc += 2;
                }
                0x7 => {
                    // 8xy7 - SUBN Vx, Vy
                    if cpu.reg[x] < cpu.reg[y] {
                        cpu.reg[0xF] = 1;
                    } else {
                        cpu.reg[0xF] = 0;
                    }
                    cpu.reg[x] = cpu.reg[y] - cpu.reg[x];
                    cpu.pc += 2;
                }
                0xE => {
                    // 8xyE - SHL Vx {, Vy}
                    if (cpu.reg[x] & 0x80) > 0 {
                        cpu.reg[0xF] = 1;
                    } else {
                        cpu.reg[0xF] = 0;
                    }
                    cpu.reg[x] <<= 1;
                    cpu.pc += 2;
                }
                _ => {
                    return Err(format!("Unknown code: {:#X}", opcode));
                }
            }
        }
        0x9000 => {
            // 9xy0 - SNE Vx, Vy
            let x = ((opcode & 0x0F00) >> 8) as usize;
            let y = ((opcode & 0x00F0) >> 4) as usize;
            if cpu.reg[x] != cpu.reg[y] {
                cpu.pc += 4;
            } else {
                cpu.pc += 2;
            }
        }
        0xA000 => {
            // Annn - LD I, addr
            cpu.i = opcode & 0x0FFF;
            cpu.pc += 2;
        }
        0xB000 => {
            // Bnnn - JP V0, addr
            cpu.pc = cpu.reg[0] as u16 + (opcode & 0x0FFF);
        }
        0xC000 => {
            // Cxkk - RND Vx, byte
            let rnum = rand::random::<u8>(); // 0 ~ 255
            let x = ((opcode & 0x0F00) >> 8) as usize;
            cpu.reg[x] = rnum & (opcode & 0x00FF) as u8;
            cpu.pc += 2;
        }
        0xD000 => {
            // Dxyn - DRW Vx, Vy, nibble
            let x = ((opcode & 0x0F00) >> 8) as usize;
            let y = ((opcode & 0x00F0) >> 4) as usize;
            let n = (opcode & 0x000F) as u8;

            cpu.reg[0xF] = 0;
            for i in 0..n {
                // row or each byte
                let pixel = cpu.memory[cpu.i.saturating_add(i as u16) as usize];
                for b in 0..8 {
                    if (pixel & (0x80 >> b)) > 0 {
                        // need change
                        let p = &mut cpu.graph[cpu.reg[y].saturating_add(i) as usize]
                            [cpu.reg[x].saturating_add(b) as usize];
                        if p.cmp_color(Color::WHITE) {
                            cpu.reg[0xF] = 1;
                            p.set(Color::BLACK);
                        } else {
                            p.set(Color::WHITE);
                        }
                    }
                }
            }
            cpu.draw_flag = true;
            cpu.pc += 2;
        }
        0xE000 => {
            let x = ((opcode & 0x0F00) >> 8) as usize;
            if (opcode & 0x00FF) == 0x9E {
                // Ex9E - SKP Vx
                if cpu.key[cpu.reg[x] as usize] == 1 {
                    cpu.pc += 4;
                } else {
                    cpu.pc += 2;
                }
            } else if (opcode & 0x00FF) == 0xA1 {
                // ExA1 - SKNP Vx
                if cpu.key[cpu.reg[x] as usize] != 1 {
                    cpu.pc += 4;
                } else {
                    cpu.pc += 2;
                }
            } else {
                return Err(format!("Unknown code: {:#X}", opcode));
            }
        }
        0xF000 => {
            let x = ((opcode & 0x0F00) >> 8) as usize;
            match opcode & 0x00FF {
                0x07 => {
                    // Fx07 - LD Vx, DT
                    cpu.reg[x] = cpu.delay_timer;
                    cpu.pc += 2;
                }
                0x0A => {
                    // Fx0A - LD Vx, K
                    let mut flag = false;
                    for i in 0_u8..16_u8 {
                        if cpu.key[i as usize] != 0 {
                            flag = true;
                            cpu.reg[x] = i;
                        }
                    }
                    if flag {
                        cpu.pc += 2;
                    }
                }
                0x15 => {
                    // Fx15 - LD DT, Vx
                    cpu.delay_timer = cpu.reg[x];
                    cpu.pc += 2;
                }
                0x18 => {
                    // Fx18 - LD ST, Vx
                    cpu.sound_timer = cpu.reg[x];
                    cpu.pc += 2;
                }
                0x1E => {
                    // Fx1E - ADD I, Vx
                    if (cpu.i + cpu.reg[x] as u16) > 0xFFF {
                        cpu.reg[0xF] = 1;
                    } else {
                        cpu.reg[0xF] = 0;
                    }
                    cpu.i += cpu.reg[x] as u16;
                    cpu.pc += 2;
                }
                0x29 => {
                    // Fx29 - LD F, Vx
                    cpu.i = (cpu.reg[x] * 0x5) as u16;
                    cpu.pc += 2;
                }
                0x33 => {
                    // Fx33 - LD B, Vx
                    cpu.memory[cpu.i as usize] = cpu.reg[x] / 100;
                    cpu.memory[cpu.i.saturating_add(1) as usize] = (cpu.reg[x] / 10) % 10;
                    cpu.memory[cpu.i.saturating_add(2) as usize] = cpu.reg[x] % 10;
                    cpu.pc += 2;
                }
                0x55 => {
                    // Fx55 - LD [I], Vx
                    for i in 0..=x {
                        cpu.memory[cpu.i.saturating_add(i as u16) as usize] = cpu.reg[i];
                    }
                    cpu.i += x as u16 + 1;
                    cpu.pc += 2;
                }
                0x65 => {
                    // Fx65 - LD Vx, [I]
                    for i in 0..=x {
                        cpu.reg[i] = cpu.memory[cpu.i.saturating_add(i as u16) as usize];
                    }
                    cpu.i += x as u16 + 1;
                    cpu.pc += 2;
                }
                _ => {
                    return Err(format!("Unknown code: {:#X}", opcode));
                }
            }
        }
        _ => {
            return Err(format!("Unknown code: {:#X}", opcode));
        }
    }
    Ok(())
}
