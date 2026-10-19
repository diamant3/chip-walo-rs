use crate::constants;

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
    pub display_update: bool,
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
            display_update: false,
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

        chip8.mem[..fonts.len()].copy_from_slice(&fonts);

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

        println!("{:x}", instr & 0xffff);

        // execute
        match code {
            0x0000 => {
                match nn {
                    0xe0 => { self.display_buf.fill(0); }
                    0xee => {
                        self.pc = self.stack[self.sp] as usize;
                        self.sp -= 1;
                    }
                    _ => (),
                }
            }
            0x1000 => { self.pc = nnn as usize }
            0x2000 => {
                self.stack[self.sp] = self.pc as u16;
                self.sp += 1;
                self.pc = nnn as usize;
            }
            0xd000 => {
                self.reg[0xf] = 0;
                let xpos = self.reg[x] as usize % constants::DISPLAY_WIDTH;
                let ypos = self.reg[y] as usize % constants::DISPLAY_HEIGHT;

                for row in 0..n {
                    let bits = self.mem[(self.idx_reg + row) as usize];
                    let cy = (ypos + row as usize) % constants::DISPLAY_HEIGHT;

                    for col in 0..8 as usize {
                        let cx = (xpos + col) % constants::DISPLAY_WIDTH;
                        let current_color = self.display_buf[cy * constants::DISPLAY_WIDTH + cx];
                        let mask = 1 << 7 - col;
                        let color = bits & mask;

                        if color > 0 {
                            if current_color > 0 {
                                self.display_buf[cy * constants::DISPLAY_WIDTH + cx] = 0;
                                self.reg[0x0f] = 1;
                            } else {
                                self.display_buf[cy * constants::DISPLAY_WIDTH + cx] = 1;
                            }
                        } else {
                        }

                        if cx == constants::DISPLAY_WIDTH - 1 {
                            break;
                        }
                    }

                    if cy == constants::DISPLAY_HEIGHT - 1 {
                        break;
                    }
                }

                // self.display_update = true;
            }
            _ => (),
        }
    }
}
