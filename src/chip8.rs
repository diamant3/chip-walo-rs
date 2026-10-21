use crate::constants;

use rand::RngExt;

#[derive(Debug)]
pub struct Chip8 {
    pub mem: [u8; constants::MEMORY_SIZE],
    pub reg: [u8; constants::REGISTER_COUNT],
    pub idx_reg: u16,
    pub stack: [u16; constants::STACK_SIZE],
    pub sp: usize,
    pub pc: usize,
    pub dt: u8,
    pub st: u8,
    pub display_buf: [u8; constants::DISPLAY_SIZE],
    pub key: [u8; constants::KEY_COUNT],
}

impl Chip8 {
    pub fn init() -> Self {
        let mut chip8 = Self {
            mem: [0; constants::MEMORY_SIZE],
            reg: [0; constants::REGISTER_COUNT],
            idx_reg: 0,
            stack: [0; constants::STACK_SIZE],
            sp: 0,
            pc: 0,
            dt: 0,
            st: 0,
            display_buf: [0; constants::DISPLAY_SIZE],
            key: [0; constants::KEY_COUNT],
        };

        let fonts = [
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

        chip8.mem[0..fonts.len()].copy_from_slice(&fonts);
        chip8.pc = constants::PROGRAM_START_ADDRESS;

        chip8
    }

    pub fn load_rom(&mut self, rom: &str) {
        let rom = std::fs::read(rom.trim()).unwrap();
        let rom_size = rom.len();
        let rom_addr = constants::PROGRAM_START_ADDRESS + rom_size;
        self.mem[constants::PROGRAM_START_ADDRESS..rom_addr].copy_from_slice(&rom[0..rom_size]);
    }

    pub fn cycle(&mut self) {
        // fetch
        let instr: u16 = ((self.mem[self.pc] as u16) << 8) | self.mem[self.pc + 1] as u16;
        self.pc += 2;

        // decode
        let code = instr & 0xf000;
        let x = ((instr & 0x0f00) >> 8) as usize;
        let y = ((instr & 0x00f0) >> 4) as usize;
        let n = instr & 0x000f;
        let nn = instr & 0x00ff;
        let nnn = instr & 0x0fff;

        // execute
        match code {
            0x0000 => match nn {
                0xe0 => {
                    self.display_buf.fill(0);
                }
                0xee => {
                    self.pc = self.stack[self.sp] as usize;
                    self.sp -= 1;
                }
                _ => ()
            },
            0x1000 => self.pc = nnn as usize,
            0x2000 => {
                self.sp += 1;
                self.stack[self.sp] = self.pc as u16;
                self.pc = nnn as usize;
            }
            0x3000 => {
                if self.reg[x] == nn as u8 {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.reg[x] != nn as u8 {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.reg[x] == self.reg[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                self.reg[x] = nn as u8;
            }
            0x7000 => {
                self.reg[x] = self.reg[x].wrapping_add(nn as u8);
            }
            0x8000 => match n {
                0x0 => {
                    self.reg[x] = self.reg[y];
                }
                0x1 => {
                    self.reg[x] |= self.reg[y];
                }
                0x2 => {
                    self.reg[x] &= self.reg[y];
                }
                0x3 => {
                    self.reg[x] ^= self.reg[y];
                }
                0x4 => {
                    self.reg[x] = self.reg[x].wrapping_add(self.reg[y]);
                }
                0x5 => {
                    self.reg[x] = self.reg[x].wrapping_sub(self.reg[y]);
                }
                0x6 => {
                    let result = self.reg[x] % 2;
                    self.reg[x] >>= 1;
                    self.reg[0xf] = result;
                }
                0x7 => {
                    self.reg[x] = self.reg[y].wrapping_sub(self.reg[x]);
                }
                0xe => {
                    let result = (self.reg[x] & 128) >> 0x7;
                    self.reg[x] <<= 1;
                    self.reg[0xf] = result;
                }
                _ => ()
            }
            0x9000 => {
                if self.reg[x] != self.reg[y] {
                    self.pc += 2;
                }
            }
            0xa000 => {
                self.idx_reg = nnn;
            }
            0xb000 => {
                let x = (nnn & 0x0f00) as usize;
                self.pc = (self.reg[x] + nnn as u8) as usize;
            }
            0xc000 => {
                let mut rng = rand::rng();
                let num = rng.random_range(0..=255);
                self.reg[x] = num;
            }
            0xd000 => {
                self.reg[0xf] = 0;
                let xpos = self.reg[x] as usize % constants::DISPLAY_WIDTH;
                let ypos = self.reg[y] as usize % constants::DISPLAY_HEIGHT;

                for yline in 0..n {
                    let px = self.mem[(self.idx_reg + yline) as usize];

                    for xline in 0..8 {
                        if (px & (0x80 >> xline)) > 0 {
                            let target =
                                constants::DISPLAY_WIDTH * (ypos + yline as usize) + (xpos + xline);
                            if self.display_buf[target] > 0 {
                                self.reg[0xf] = 1;
                            }

                            self.display_buf[target] ^= 1;
                        }
                    }
                }
            }
            0xe000 => match nn {
                0x9e => {
                    if self.key[self.reg[x] as usize] == 1 {
                        self.pc += 2;
                    }
                }
                0xa1 => {
                    if self.key[self.reg[x] as usize] == 0 {
                        self.pc += 2;
                    }
                }
                _ => ()
            }
            0xf000 => match nn {
                0x07 => {
                    self.reg[x] = self.dt;
                }
                0x0a => {
                    for key in 0..constants::KEY_COUNT {
                        if self.key[key] == 1 {
                            self.reg[x] = key as u8;
                            break;
                        }
                    }
                }
                0x15 => {
                    self.dt = self.reg[x];
                }
                0x18 => {
                    self.st = self.reg[x];
                }
                0x1e => {
                    self.idx_reg += self.reg[x] as u16;
                }
                0x29 => {
                    self.idx_reg = (self.reg[x] * 5) as u16;
                }
                0x33 => {
                    let vx = self.reg[x];
                    self.mem[self.idx_reg as usize] = vx / 100;
                    self.mem[self.idx_reg as usize + 1] = (vx / 10) % 10;
                    self.mem[self.idx_reg as usize + 2] = (vx / 100) % 10;
                }
                0x55 => {
                    for reg in 0..=x {
                        self.mem[self.idx_reg as usize + reg] = self.reg[reg];
                        self.idx_reg += reg as u16;
                    }
                }
                0x65 => {
                    for reg in 0..=x {
                        self.reg[reg as usize] = self.mem[self.idx_reg as usize + reg];
                        self.idx_reg += reg as u16;
                    }
                }
                _ => ()
            }
            _ => (),
        }
    }
}
