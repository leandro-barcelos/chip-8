use crate::{
    chip8::Chip8,
    consts::{FRAME_TIME_60HZ, FRAME_TIME_700HZ, HEIGHT, SCALE, WIDTH},
};
use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus},
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    EventPump,
};
use std::{
    env, fs,
    time::{Duration, Instant},
};
use std::{f32::consts::PI, io};

mod chip8;
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

struct Emulator {
    canvas: WindowCanvas,
    audio_device: AudioDevice<SineWave>,
    event_pump: EventPump,
    chip8: Chip8,
}

impl Emulator {
    pub fn new(program: &Vec<u8>) -> Self {
        let sdl_context = sdl2::init().expect("failed to initialize sdl!");

        let video_subsystem = sdl_context
            .video()
            .expect("failed to initialize video subsystem!");
        let window = video_subsystem
            .window("Chip-8", (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
            .position_centered()
            .build()
            .expect("failed to make a window!");
        let canvas = window
            .into_canvas()
            .build()
            .expect("failed to make a canvas!");

        let audio_subsystem = sdl_context
            .audio()
            .expect("failed to initialize audio subsystem!");
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: Some(1024),
        };
        let audio_device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                let freq = 440.0;
                SineWave {
                    phase_inc: 2.0 * PI * freq / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .expect("failed to initialize audio device!");

        let event_pump = sdl_context
            .event_pump()
            .expect("failed to obtain event pump!");

        let mut chip8 = Chip8::new();
        chip8.load_program(program);

        Self {
            canvas,
            audio_device,
            event_pump,
            chip8,
        }
    }

    pub fn run(&mut self, n_cycles: u32) {
        let mut global_timer = 0.0;
        let mut last_loop = Instant::now();
        let mut halting_key: Option<u8> = None;

        'running: for _ in 0..n_cycles {
            // Handle events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        if let Some(chip8_key) = Emulator::keycode_to_chip8_key(key) {
                            if self.chip8.waiting_for_key {
                                halting_key = Some(chip8_key);
                            }

                            self.chip8.press_key(chip8_key);
                        }
                    }
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        if let Some(chip8_key) = Emulator::keycode_to_chip8_key(key) {
                            if halting_key.is_some_and(|k| k == chip8_key) {
                                self.chip8.last_key_pressed = Some(chip8_key);
                                halting_key = None;
                            }

                            self.chip8.release_key(chip8_key);
                        }
                    }
                    _ => {}
                }
            }

            // Update
            self.chip8.cycle();

            global_timer += last_loop.elapsed().as_secs_f32();
            //  Everything inside this if is updated at 60Hzt
            if global_timer >= FRAME_TIME_60HZ {
                self.chip8.decrease_timers();
            }

            if self.chip8.sound_timer > 0 && self.audio_device.status() != AudioStatus::Playing {
                self.audio_device.resume();
            }
            if self.chip8.sound_timer == 0 && self.audio_device.status() == AudioStatus::Playing {
                self.audio_device.pause();
            }

            // Render
            self.render().unwrap();

            // Time management
            if global_timer >= FRAME_TIME_60HZ {
                global_timer = 0.0;
            }
            last_loop = Instant::now();
            ::std::thread::sleep(Duration::from_secs_f32(FRAME_TIME_700HZ));
        }
    }

    fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(1, 170, 1));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::GREEN);

        for (i, row) in self.chip8.display.iter().enumerate() {
            for (j, &pixel) in row.iter().enumerate() {
                if pixel {
                    let scaled_pixel = Rect::new(
                        (j * SCALE) as i32,
                        (i * SCALE) as i32,
                        SCALE as u32,
                        SCALE as u32,
                    );
                    self.canvas.fill_rect(scaled_pixel)?;
                }
            }
        }

        self.canvas.present();

        Ok(())
    }

    fn keycode_to_chip8_key(keycode: Keycode) -> Option<u8> {
        match keycode {
            Keycode::NUM_1 => Some(0x1),
            Keycode::NUM_2 => Some(0x2),
            Keycode::NUM_3 => Some(0x3),
            Keycode::NUM_4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),
            _ => None,
        }
    }
}

fn main() -> Result<(), io::Error> {
    let program =
        fs::read(env::args().nth(1).expect("no program file provided!")).expect("file not found!");

    let n_cycles = env::args()
        .nth(2)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(u32::MAX);

    let mut emulator = Emulator::new(&program);
    emulator.run(n_cycles);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8_logo() {
        let program = fs::read("tests/1-chip8-logo.ch8").expect("file not found!");

        let mut emulator = Emulator::new(&program);
        emulator.run(39);
        emulator.run(u32::MAX);
    }

    #[test]
    fn test_ibm_logo() {
        let program = fs::read("tests/2-ibm-logo.ch8").expect("file not found!");

        let mut emulator = Emulator::new(&program);
        emulator.run(20);
        emulator.run(u32::MAX);
    }

    #[test]
    fn test_corax() {
        let program = fs::read("tests/3-corax+.ch8").expect("file not found!");

        let mut emulator = Emulator::new(&program);
        emulator.run(u32::MAX);
    }

    #[test]
    fn test_flags() {
        let program = fs::read("tests/4-flags.ch8").expect("file not found!");

        let mut emulator = Emulator::new(&program);
        emulator.run(u32::MAX);
    }

    #[test]
    fn test_quirks() {
        let program = fs::read("tests/5-quirks.ch8").expect("file not found!");

        let mut emulator = Emulator::new(&program);
        emulator.run(u32::MAX);
    }

    #[test]
    fn test_keypad() {
        let program = fs::read("tests/6-keypad.ch8").expect("file not found!");

        let mut emulator = Emulator::new(&program);
        emulator.run(u32::MAX);
    }

    #[test]
    fn test_beep() {
        let program = fs::read("tests/7-beep.ch8").expect("file not found!");

        let mut emulator = Emulator::new(&program);
        emulator.run(u32::MAX);
    }

    #[test]
    fn test_scrolling() {
        let program = fs::read("tests/8-scrolling.ch8").expect("file not found!");

        let mut emulator = Emulator::new(&program);
        emulator.run(u32::MAX);
    }
}
