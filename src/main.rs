use chip8::Chip8;
use display::Display;
use std::io;
use std::time::{Duration, Instant};

mod chip8;
mod display;
mod font;

fn main() -> Result<(), io::Error> {
    let mut chip8 = Chip8::new();

    let mut display = Display::new();
    let frame_duration = Duration::from_secs_f64(1.0 / 60.0);

    loop {
        let frame_start = Instant::now();

        if !display.event_poll() {
            break;
        }

        chip8.update();

        display.render().unwrap();

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    Ok(())
}
