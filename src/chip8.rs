use std::time::{Duration, Instant};

use rand::Rng;

use crate::{
    consts::{
        BNNN, DISPLAY_WAIT, FONT_START_ADRESS, FRAME_TIME_60HZ, HEIGHT, PROGRAM_START_ADDRESS,
        SHIFT_USE_VY, STORE_LOAD_INCREMENTS_I, VF_RESET, WIDTH,
    },
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
    pub keypad: [bool; 16],
    pub waiting_for_key: bool,
    pub last_key_pressed: Option<u8>,
    last_draw_time: Instant,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0; 0x1000];

        Chip8::store_font(&mut memory);

        Chip8 {
            display: [[false; WIDTH as usize]; HEIGHT as usize],
            program_counter: PROGRAM_START_ADDRESS as u16,
            index_register: 0,
            memory,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            variable_registers: [0; 0x0010],
            last_update: Instant::now(),
            global_timer: 0.0,
            keypad: [false; 16],
            waiting_for_key: false,
            last_key_pressed: None,
            last_draw_time: Instant::now(),
        }
    }

    pub fn release_key(&mut self, key: u8) {
        self.keypad[key as usize] = false;
    }

    pub fn press_key(&mut self, key: u8) {
        self.keypad[key as usize] = true;
    }

    pub fn load_program(&mut self, program: &Vec<u8>) {
        for (i, byte) in program.iter().enumerate() {
            self.memory[PROGRAM_START_ADDRESS + i] = *byte;
        }
    }

    fn store_font(memory: &mut [u8; 0x1000]) {
        let mut i = 0;
        for chr in FONT_CHARACTERS.iter() {
            for byte in chr.bitmap {
                memory[FONT_START_ADRESS + i] = byte;
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

                if y as usize + i >= HEIGHT || x as usize + j >= WIDTH {
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

    pub fn cycle(&mut self) {
        // Fetch
        let instruction = ((self.memory[self.program_counter as usize] as u16) << 8)
            | self.memory[(self.program_counter + 1) as usize] as u16;

        self.program_counter += 2;

        // X: The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
        // Y: The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
        // N: The fourth nibble. A 4-bit number.
        // NN: The second byte (third and fourth nibbles). An 8-bit immediate number.
        // NNN: The second, third and fourth nibbles. A 12-bit immediate memory address.
        let x = ((instruction >> 8) & 0xF) as usize;
        let y = ((instruction >> 4) & 0xF) as usize;
        let n = instruction & 0xF;
        let nn = instruction & 0xFF;
        let nnn = instruction & 0xFFF;

        let vx = self.variable_registers[x];
        let vy = self.variable_registers[y];

        // Decode / Execute
        match (instruction >> 12) & 0xF {
            0x0 => match instruction {
                0x00E0 => self.clear_display(),
                0x00EE => self.program_counter = self.stack.pop().expect("0x00EE: stack is empty"),
                _ => println!(
                    "{:#06X}: execute machine language routine instruction detected!",
                    instruction
                ),
            },
            0x1 => self.program_counter = nnn,
            0x2 => {
                self.stack.push(self.program_counter);
                self.program_counter = nnn;
            }
            0x3 => {
                if vx as u16 == nn {
                    self.program_counter += 2;
                }
            }
            0x4 => {
                if vx as u16 != nn {
                    self.program_counter += 2;
                }
            }
            0x5 => {
                if vx == vy {
                    self.program_counter += 2;
                }
            }
            0x6 => self.variable_registers[x] = nn as u8,
            0x7 => {
                self.variable_registers[x] = vx.wrapping_add(nn as u8);
            }
            0x8 => match instruction & 0xF {
                0x0 => self.variable_registers[x] = vy,
                0x1 => {
                    self.variable_registers[x] |= vy;
                    if VF_RESET {
                        self.variable_registers[0xF] = 0;
                    }
                }
                0x2 => {
                    self.variable_registers[x] &= vy;
                    if VF_RESET {
                        self.variable_registers[0xF] = 0;
                    }
                }
                0x3 => {
                    self.variable_registers[x] ^= vy;
                    if VF_RESET {
                        self.variable_registers[0xF] = 0;
                    }
                }
                0x4 => {
                    let (result, overflow) = vx.overflowing_add(vy);
                    self.variable_registers[x] = result;
                    self.variable_registers[0xF] = overflow as u8;
                }
                0x5 => {
                    let (result, underflow) = vx.overflowing_sub(vy);
                    self.variable_registers[x] = result;
                    self.variable_registers[0xF] = (!underflow) as u8;
                }
                0x6 => {
                    let mut register = vx;
                    if SHIFT_USE_VY {
                        register = vy;
                    }
                    let shifted_bit = register & 0x1;
                    self.variable_registers[x] = register >> 1;
                    self.variable_registers[0xF] = shifted_bit;
                }
                0x7 => {
                    let (result, underflow) = vy.overflowing_sub(vx);
                    self.variable_registers[x] = result;
                    self.variable_registers[0xF] = (!underflow) as u8;
                }
                0xE => {
                    let mut register = vx;
                    if SHIFT_USE_VY {
                        register = vy;
                    }
                    let shifted_bit = (register >> 7) & 0x1;
                    self.variable_registers[x] = register << 1;
                    self.variable_registers[0xF] = shifted_bit;
                }
                _ => panic!("{:#06X}: unknown instruction!", instruction),
            },
            0x9 => {
                if vx != vy {
                    self.program_counter += 2;
                }
            }
            0xA => self.index_register = nnn,
            0xB => match BNNN {
                true => {
                    let v0 = self.variable_registers[0];
                    self.program_counter = nnn + v0 as u16;
                }
                false => {
                    let xnn = ((x as u16) << 8) | nn;
                    self.program_counter = xnn + vx as u16;
                }
            },
            0xC => {
                let random: u8 = rand::rng().random();
                self.variable_registers[x] = random & nn as u8;
            }
            0xD => {
                if DISPLAY_WAIT {
                    let time_since_last_draw = self.last_draw_time.elapsed();
                    let min_draw_interval = Duration::from_secs_f32(FRAME_TIME_60HZ);

                    if time_since_last_draw < min_draw_interval {
                        self.program_counter -= 2;
                        return;
                    }

                    self.last_draw_time = Instant::now();
                }

                self.variable_registers[0xF] = 0;

                let sprite = self.memory
                    [self.index_register as usize..(self.index_register + n) as usize]
                    .to_vec();

                self.draw_sprite(sprite, vx, vy);
            }
            0xE => match instruction & 0xFF {
                0x9E => {
                    if self.keypad[vx as usize] {
                        self.program_counter += 2;
                    }
                }
                0xA1 => {
                    if !self.keypad[vx as usize] {
                        self.program_counter += 2;
                    }
                }
                _ => panic!("{:#06X}: unknown instruction!", instruction),
            },
            0xF => match instruction & 0xFF {
                0x07 => self.variable_registers[x] = self.delay_timer,
                0x15 => self.delay_timer = self.variable_registers[x],
                0x18 => self.sound_timer = self.variable_registers[x],
                0x1E => {
                    self.index_register += vx as u16;
                }
                0x0A => {
                    self.waiting_for_key = true;

                    if let Some(key) = self.last_key_pressed {
                        self.variable_registers[x] = key;
                        self.waiting_for_key = false;
                        self.last_key_pressed = None;
                    } else {
                        self.program_counter -= 2;
                    }
                }
                0x29 => self.index_register = FONT_START_ADRESS as u16 + vx as u16 * 5,
                0x33 => {
                    let digits = [vx / 100, (vx / 10) % 10, vx % 10];

                    for i in 0..3 {
                        self.memory[self.index_register as usize + i] = digits[i];
                    }
                }
                0x55 => {
                    self.variable_registers[0..=x]
                        .iter()
                        .enumerate()
                        .for_each(|(i, r)| {
                            self.memory[self.index_register as usize + i] = *r;
                        });

                    if STORE_LOAD_INCREMENTS_I {
                        self.index_register += (x + 1) as u16;
                    }
                }
                0x65 => {
                    self.memory[self.index_register as usize..=self.index_register as usize + x]
                        .iter()
                        .enumerate()
                        .for_each(|(i, r)| {
                            self.variable_registers[i] = *r;
                        });

                    if STORE_LOAD_INCREMENTS_I {
                        self.index_register += (x + 1) as u16;
                    }
                }
                _ => panic!("{:#06X}: unknown instruction!", instruction),
            },
            _ => panic!("{:#06X}: unknown instruction!", instruction),
        }
    }
}
