use std::time::{Duration, Instant};

use sdl2::audio;

use crate::{audio::Audio, consts::FRAME_TIME, font::FONT_CHARACTERS};

pub struct Chip8 {
    program_counter: u16,
    index_register: u16,
    memory: [u8; 0x1000],
    stack: Vec<u16>,
    delay_timer: u8,
    variable_registers: [u8; 0x0010],
    audio: Audio,
    last_update: Instant,
    global_timer: f32,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0; 0x1000];

        Chip8::store_font(&mut memory);

        Chip8 {
            program_counter: 0,
            index_register: 0,
            memory,
            stack: Vec::new(),
            delay_timer: 0,
            variable_registers: [0; 0x0010],
            audio: Audio::new(0),
            last_update: Instant::now(),
            global_timer: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.global_timer += self.last_update.elapsed().as_secs_f32();

        if self.global_timer >= FRAME_TIME {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            self.audio.update();

            self.global_timer = 0.0;
        }

        self.last_update = Instant::now();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beeping() {
        let timer = 255;

        let mut chip8 = Chip8 {
            program_counter: 0,
            index_register: 0,
            memory: [0; 0x1000],
            stack: Vec::new(),
            delay_timer: 0,
            variable_registers: [0; 0x0010],
            audio: Audio::new(timer),
            last_update: Instant::now(),
            global_timer: 0.0,
        };

        let start = Instant::now();

        while chip8.audio.sound_timer > 0 {
            chip8.update();
            std::thread::sleep(Duration::from_secs_f32(FRAME_TIME));
        }

        let duration = start.elapsed().as_secs_f32();
        println!("{}", duration);

        assert!(duration >= timer as f32 * FRAME_TIME)
    }
}
