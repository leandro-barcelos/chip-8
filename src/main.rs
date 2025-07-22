use crate::{
    consts::{FRAME_TIME, HEIGHT, SCALE, WIDTH},
    font::FONT_CHARACTERS,
};
use sdl2::{
    audio::{AudioCallback, AudioSpecDesired, AudioStatus},
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
};
use std::time::Instant;
use std::{f32::consts::PI, io};

mod consts;
mod font;

struct SineWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SineWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = self.phase.sin() * self.volume;
            self.phase = (self.phase + self.phase_inc) % (2.0 * PI);
        }
    }
}

#[allow(unused)]
pub struct Chip8 {
    display: [[bool; WIDTH]; HEIGHT],
    program_counter: u16,
    index_register: u16,
    memory: [u8; 0x1000],
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    variable_registers: [u8; 0x0010],
    last_update: Instant,
    global_timer: f32,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0; 0x1000];

        Chip8::store_font(&mut memory);

        Chip8 {
            display: [[false; WIDTH]; HEIGHT],
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

    pub fn draw_sprite(&mut self, sprite: Vec<u8>, x: usize, y: usize) -> Result<(), io::Error> {
        if x + 8 > WIDTH || y + sprite.len() > HEIGHT {
            return Err(io::Error::new(io::ErrorKind::Other, "Sprite out of bounds"));
        }

        for (i, &row) in sprite.iter().enumerate() {
            for j in 0..8 {
                let pixel = (row >> (7 - j)) & 1;
                self.display[y + i][x + j] ^= pixel == 1;
            }
        }

        Ok(())
    }

    pub fn clear_display(&mut self) {
        self.display = [[false; WIDTH]; HEIGHT]
    }
}

fn main() -> Result<(), io::Error> {
    let sdl_context = sdl2::init().expect("could not initialize sdl!");

    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8", (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
        .position_centered()
        .build()
        .expect("Failed to initialize video subsystem");
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to make a canvas");

    let audio_subsystem = sdl_context
        .audio()
        .expect("could not initialize audio subsystem!");
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: Some(1024),
    };
    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            let freq = 440.0;
            SineWave {
                phase_inc: 2.0 * PI * freq / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })
        .expect("failed to initialize audio device!");

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();

    let mut global_timer = 0.0;
    let mut last_loop = Instant::now();

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Update
        global_timer += last_loop.elapsed().as_secs_f32();

        //  Everything inside this if is updated at 60Hzt
        if global_timer >= FRAME_TIME {
            if chip8.delay_timer > 0 {
                chip8.delay_timer -= 1;
            }

            if chip8.sound_timer > 0 {
                chip8.sound_timer -= 1;
            }

            global_timer = 0.0;
        }

        if chip8.sound_timer > 0 && device.status() != AudioStatus::Playing {
            device.resume();
        }

        if chip8.sound_timer == 0 && device.status() == AudioStatus::Playing {
            device.pause();
        }

        // Render
        render(&mut canvas, &chip8).unwrap();

        // Time management
        last_loop = Instant::now();
    }

    Ok(())
}

pub fn render(canvas: &mut WindowCanvas, chip8: &Chip8) -> Result<(), String> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    canvas.set_draw_color(Color::GREEN);

    for (i, row) in chip8.display.iter().enumerate() {
        for (j, &pixel) in row.iter().enumerate() {
            if pixel {
                let scaled_pixel = Rect::new(
                    (j * SCALE) as i32,
                    (i * SCALE) as i32,
                    SCALE as u32,
                    SCALE as u32,
                );
                canvas.fill_rect(scaled_pixel)?;
            }
        }
    }

    canvas.present();

    Ok(())
}
