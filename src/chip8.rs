use rand::prelude::*;

pub const CHIP_8_FONT_SET: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    opcode: u16,
    pub memory: [u8; 4096],
    pub pixel_buffer: [u8; 64 * 32],
    registers: [u8; 16],
    index_register: u16,
    program_counter: usize,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    stack_pointer: usize,
    keys: [u8; 16],
    rng: ThreadRng
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory: [u8; 4096] = [0u8; 4096];
        
        for (index, value) in CHIP_8_FONT_SET.iter().enumerate() {
            memory[index] = *value;
        }

        Self {
            opcode: 0x0000,
            memory: [0u8; 4096],
            pixel_buffer: [0u8; 64 * 32],
            registers: [0u8; 16],
            index_register: 0x0000,
            program_counter: 0x0200,
            delay_timer: 0x00,
            sound_timer: 0x00,
            stack: [0u16; 16],
            stack_pointer: 0x0000,
            keys: [0u8; 16],
            rng: rand::thread_rng()
        }
    }

    fn increment_program_counter(&mut self) {
        self.program_counter += 2;
    }

    pub fn cycle(&mut self) {
        self.opcode = ((self.memory[self.program_counter] as u16) << 8) | self.memory[self.program_counter + 1] as u16;
        println!("Instruction: {:#06x}", self.opcode.clone());
        let first_nyble: u16 = self.opcode >> 12;
        let second_nyble: usize = ((self.opcode & 0x0F00) >> 8).into();
        let third_nyble: usize = ((self.opcode & 0x00F0) >> 4).into();
        let forth_nyble: usize = (self.opcode & 0x000F).into();
        let byte: u8 = (self.opcode & 0x00FF) as u8;
        let argument: usize = (self.opcode & 0x0FFF).into();

        match first_nyble {
            0x0 => {
                if self.opcode == 0x00E0 {
                    for pixel in self.pixel_buffer.iter_mut() {
                        *pixel = 0x00;
                    }
                } else if self.opcode == 0x00EE {
                    self.stack_pointer -= 1;
                    self.program_counter = self.stack[self.stack_pointer].into();
                }
                self.increment_program_counter();
            },
            0x1 => {
                self.program_counter = argument;
            },
            0x2 => {
                self.stack[self.stack_pointer] = self.program_counter as u16;
                self.stack_pointer += 1;
                self.program_counter = argument;
            },
            0x3 => {
                if self.registers[second_nyble] == (self.opcode & 0x00FF) as u8 {
                    self.increment_program_counter();
                }
                self.increment_program_counter();
            },
            0x4 => {
                if self.registers[second_nyble] != (self.opcode & 0x00FF) as u8 {                    
                    self.increment_program_counter();
                }
                self.increment_program_counter();
            },
            0x5 => {
                if self.registers[second_nyble] == self.registers[third_nyble] {
                    self.increment_program_counter();
                }
                self.increment_program_counter();
            },
            0x6 => {
                self.registers[second_nyble] = byte;
                self.increment_program_counter();
            },
            0x7 => {
                self.registers[second_nyble] = self.registers[second_nyble].overflowing_add(byte).0;
                self.increment_program_counter();
            },
            0x8 => {
                match forth_nyble {
                    0x0 => {
                        self.registers[second_nyble] = self.registers[third_nyble]; 
                    },
                    0x1 => {
                        self.registers[second_nyble] |= self.registers[third_nyble]; 
                    },
                    0x2 => {
                        self.registers[second_nyble] &= self.registers[third_nyble];
                    },
                    0x3 => {
                        self.registers[second_nyble] ^= self.registers[third_nyble];
                    },
                    0x4 => {
                        let sum: u16 = self.registers[second_nyble] as u16 + self.registers[third_nyble] as u16;
                        self.registers[0xF] = (sum > 0xFF).into(); 
                        self.registers[second_nyble] = (sum & 0xFF) as u8;
                    },
                    0x5 => {
                        self.registers[0xF] = (self.registers[second_nyble] > self.registers[third_nyble]) as u8;
                        self.registers[second_nyble] = self.registers[second_nyble].overflowing_sub(self.registers[third_nyble]).0;
                    },
                    0x6 => {
                        self.registers[0xF] = self.registers[second_nyble] & 0x1;
                        self.registers[second_nyble] = self.registers[second_nyble] >> 1; 
                    },
                    0x7 => {
                        self.registers[0xF] = (self.registers[third_nyble] > self.registers[second_nyble]).into();
                        self.registers[second_nyble] = self.registers[third_nyble].overflowing_sub(self.registers[second_nyble]).0; 
                    }
                    0xE => {
                        self.registers[0xF] = (self.registers[second_nyble] & 0x80 != 0).into();
                        self.registers[second_nyble] = self.registers[second_nyble] << 1; 
                    }
                    _ => {}
                }
                self.increment_program_counter();
            }
            0x9 => {
                if self.registers[second_nyble] != self.registers[third_nyble] {
                    self.increment_program_counter();
                }
                self.increment_program_counter();
            },
            0xA => {
                self.index_register = argument as u16;
                self.increment_program_counter();
            },
            0xB => {
                self.program_counter = argument + self.registers[0] as usize; 
            },
            0xC => {
                self.registers[second_nyble] = self.rng.gen::<u8>() & byte;
                self.increment_program_counter();
            },
            0xD => {
                self.registers[0xF] = 0;
                let mut y: usize = 0;
                while y < forth_nyble {
                    let mut x: usize = 0;
                    let pixel: u8 = self.memory[self.index_register as usize + y];
                    while x < 8 {
                        let significant: u8 = 0x80;
                        if pixel & (significant >> x) != 0 {
                            let tmp_x: usize = (self.registers[second_nyble] as usize + x) % 64;
                            let tmp_y: usize = (self.registers[third_nyble] as usize+ y) % 32;
                            let index: usize = tmp_x + tmp_y * 64;
                            
                            self.pixel_buffer[index] ^= 1;
                            self.registers[0xF] = (self.pixel_buffer[index] == 0).into();
                        }
                        x += 1;
                    }
                    y += 1;
                }
                self.increment_program_counter();
            },
            0xE => {
                if (byte == 0x9E && self.keys[self.registers[second_nyble] as usize] == 1)  || (byte == 0xA1 && self.keys[self.registers[second_nyble] as usize] != 1) {
                    self.increment_program_counter();
                }
                self.increment_program_counter();
            },
            0xF => {
                match byte {
                    0x07 => {
                        self.registers[second_nyble] = self.delay_timer;
                    },
                    0x0A => {
                        let mut pressed: bool = false;
                        for (key, index) in self.keys.iter().enumerate() {
                            if key != 0 {
                                self.registers[second_nyble] = *index;
                                pressed = true;
                                break;
                            }
                        }
                        if !pressed{
                            return
                        }
                    },
                    0x15 => {
                        self.delay_timer = self.registers[second_nyble];
                    },
                    0x18 => {
                        self.sound_timer = self.registers[second_nyble];
                    },
                    0x1E => {
                       self.index_register = self.index_register.overflowing_add(self.registers[second_nyble].into()).0;
                    },
                    0x29 => {
                        if self.registers[second_nyble] < 16 {
                            self.index_register = (self.registers[second_nyble] * 0x05) as u16;
                        }
                    },
                    0x33 => {
                        self.memory[self.index_register as usize] = self.registers[second_nyble].overflowing_div(100).0;
                        self.memory[self.index_register as usize + 1] = (self.registers[second_nyble].overflowing_div(10)).0.overflowing_rem(10).0;
                        self.memory[self.index_register as usize + 2] = self.registers[second_nyble].overflowing_rem(10).0;
                    },
                    0x55 => {
                        let mut index: u16 = 0;
                        while index <= second_nyble as u16 {
                            self.memory[self.index_register.overflowing_add(index).0 as usize] = self.registers[index as usize];
                            index += 1;
                        }
                    },
                    0x65 => {
                        let mut index: u16 = 0;
                        while index <= second_nyble as u16{
                            self.registers[index as usize] = self.memory[self.index_register.overflowing_add(index).0 as usize];
                            index += 1;
                        }
                    }
                    _ => {}
                }
                self.increment_program_counter();
            }
            _ => {}
        }
    }
}
