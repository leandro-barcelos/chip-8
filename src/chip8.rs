use std::time::Instant;

use crate::{
    consts::{HEIGHT, WIDTH},
    font::FONT_CHARACTERS,
};

#[allow(unused)]
pub struct Chip8 {
    pub display: [[bool; WIDTH as usize]; HEIGHT as usize],
    program_counter: u16,
    index_register: u16,
    memory: [u8; 0x1000],
    stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    variable_registers: [u8; 0x0010],
    last_update: Instant,
    global_timer: f32,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0; 0x1000];

        Chip8::store_font(&mut memory);

        Chip8 {
            display: [[false; WIDTH as usize]; HEIGHT as usize],
            program_counter: 0,
            index_register: 0,
            memory,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            variable_registers: [0; 0x0010],
            last_update: Instant::now(),
            global_timer: 0.0,
        }
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        let start: usize = 0x200;

        for (i, byte) in program.iter().enumerate() {
            self.memory[start + i] = *byte;
        }

        self.program_counter = 0x200;
    }

    fn store_font(memory: &mut [u8; 0x1000]) {
        let start: usize = 0x050;

        let mut i = 0;
        for chr in FONT_CHARACTERS.iter() {
            for byte in chr.bitmap {
                memory[start + i] = byte;
                i += 1;
            }
        }
    }

    pub fn draw_sprite(&mut self, sprite: Vec<u8>, mut x: u8, mut y: u8) {
        x %= WIDTH as u8;
        y %= HEIGHT as u8;

        for (i, &row) in sprite.iter().enumerate() {
            for j in 0..8 as usize {
                let pixel = ((row >> (7 - j)) & 1) == 1;

                if y as usize + i > HEIGHT || x as usize + j > WIDTH {
                    continue;
                }

                if pixel && self.display[y as usize + i][x as usize + j] {
                    self.variable_registers[0xF] = 1;
                }

                self.display[y as usize + i][x as usize + j] ^= pixel;
            }
        }
    }

    fn clear_display(&mut self) {
        self.display = [[false; WIDTH]; HEIGHT]
    }

    pub fn decrease_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn update(&mut self) {
        // Fetch
        let instruction = ((self.memory[self.program_counter as usize] as u16) << 8)
            | self.memory[(self.program_counter + 1) as usize] as u16;

        self.program_counter += 2;

        // Decode / Execute
        match (instruction >> 12) & 0xF {
            0x0 => match instruction {
                0x00E0 => self.clear_display(),
                // 0x00EE => self.program_counter = self.stack.pop().expect("0x00EE: stack is empty"),
                _ => panic!(
                    "{:X}: execute machine language routine instruction detected!",
                    instruction
                ),
            },
            0x1 => self.program_counter = instruction & 0xFFF,
            // 0x2 => {
            //     self.stack.push(self.program_counter);
            //     self.program_counter = instruction & 0xFFF;
            // }
            // 0x3 => {
            //     let vx = self.variable_registers[((instruction >> 8) & 0xF) as usize];
            //     let nn = instruction & 0xFF;

            //     if vx as u16 == nn {
            //         self.program_counter += 2;
            //     }
            // }
            // 0x4 => {
            //     let vx = self.variable_registers[((instruction >> 8) & 0xF) as usize];
            //     let nn = instruction & 0xFF;

            //     if vx as u16 != nn {
            //         self.program_counter += 2;
            //     }
            // }
            // 0x5 => {
            //     let vx = self.variable_registers[((instruction >> 8) & 0xF) as usize];
            //     let vy = self.variable_registers[((instruction >> 4) & 0xF) as usize];

            //     if vx == vy {
            //         self.program_counter += 2;
            //     }
            // }
            0x6 => {
                self.variable_registers[((instruction >> 8) & 0xF) as usize] =
                    (instruction & 0xFF) as u8
            }
            0x7 => {
                self.variable_registers[((instruction >> 8) & 0xF) as usize] +=
                    (instruction & 0xFF) as u8;
            }
            // 0x9 => {
            //     let vx = self.variable_registers[((instruction >> 8) & 0xF) as usize];
            //     let vy = self.variable_registers[((instruction >> 4) & 0xF) as usize];

            //     if vx != vy {
            //         self.program_counter += 2;
            //     }
            // }
            0xA => self.index_register = instruction & 0xFFF,
            0xD => {
                let vx = self.variable_registers[((instruction >> 8) & 0xF) as usize];
                let vy = self.variable_registers[((instruction >> 4) & 0xF) as usize];
                let n = instruction & 0xF;

                self.variable_registers[0xF] = 0;

                let sprite = self.memory
                    [self.index_register as usize..(self.index_register + n) as usize]
                    .to_vec();

                self.draw_sprite(sprite, vx, vy);
            }

            _ => panic!("unknown instruction!"),
        }
    }
}
