use display::Display;
use font::FONT_CHARACTERS;
use std::io;
use std::time::{Duration, Instant};

mod display;
mod font;

struct Chip8 {
    program_counter: u16,
    index_register: u16,
    memory: [u8; 0x1000],
}

fn main() -> Result<(), io::Error> {
    let mut display = Display::new();
    let frame_duration = Duration::from_secs_f64(1.0 / 60.0);

    loop {
        let frame_start = Instant::now();

        if !display.event_poll() {
            break;
        }

        display.draw(FONT_CHARACTERS[0x0].bitmap.to_vec(), 0, 0)?;
        display.draw(FONT_CHARACTERS[0x1].bitmap.to_vec(), 5, 0)?;
        display.draw(FONT_CHARACTERS[0x2].bitmap.to_vec(), 10, 0)?;

        // display.debug_display();

        display.render().unwrap();

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }

    Ok(())
}
