use crate::{
    chip8::Chip8,
    consts::{FRAME_TIME, HEIGHT, SCALE, TIMER_UPDATE_TIME, WIDTH},
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
        let sdl_context = sdl2::init().expect("could not initialize sdl!");

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Chip-8", (WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)
            .position_centered()
            .build()
            .expect("Failed to initialize video subsystem");
        let canvas = window
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

        let event_pump = sdl_context.event_pump().unwrap();

        let mut chip8 = Chip8::new();
        chip8.load_program(program);

        Self {
            canvas,
            audio_device,
            event_pump,
            chip8,
        }
    }

    pub fn run(&mut self) {
        let mut global_timer = 0.0;
        let mut last_loop = Instant::now();

        'running: loop {
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
                    _ => {}
                }
            }

            // Update
            global_timer += last_loop.elapsed().as_secs_f32();

            //  Everything inside this if is updated at 60Hzt
            if global_timer >= TIMER_UPDATE_TIME {
                self.chip8.decrease_timers();

                global_timer = 0.0;
            }

            if self.chip8.sound_timer > 0 && self.audio_device.status() != AudioStatus::Playing {
                self.audio_device.resume();
            }

            if self.chip8.sound_timer == 0 && self.audio_device.status() == AudioStatus::Playing {
                self.audio_device.pause();
            }

            self.chip8.update();

            // Render
            self.render().unwrap();

            // Time management
            last_loop = Instant::now();
            ::std::thread::sleep(Duration::from_secs_f32(FRAME_TIME));
        }
    }

    fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
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
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let program =
        fs::read(args.get(1).expect("no program file provided!")).expect("file not found!");

    let mut emulator = Emulator::new(&program);
    emulator.run();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ibm_logo() {
        let program = fs::read("tests/IBM Logo.ch8").expect("file not found!");

        let mut emulator = Emulator::new(&program);
        emulator.run();
    }
}
