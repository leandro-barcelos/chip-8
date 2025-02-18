use crate::font::FONT_CHARACTERS;

pub struct Chip8 {
    program_counter: u16,
    index_register: u16,
    memory: [u8; 0x1000],
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
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
            sound_timer: 0,
        }
    }

    pub fn update(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
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
