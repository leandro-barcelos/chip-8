use std::io;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

pub struct Display {
    pixels: [[bool; WIDTH]; HEIGHT],
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl Display {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
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

        let event_pump = sdl_context.event_pump().unwrap();

        Display {
            pixels: [[false; WIDTH]; HEIGHT],
            canvas,
            event_pump,
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [[false; WIDTH]; HEIGHT]
    }

    pub fn draw(&mut self, sprite: Vec<u8>, x: usize, y: usize) -> Result<(), io::Error> {
        if x + 8 > WIDTH || y + sprite.len() > HEIGHT {
            return Err(io::Error::new(io::ErrorKind::Other, "Sprite out of bounds"));
        }

        for (i, &row) in sprite.iter().enumerate() {
            for j in 0..8 {
                let pixel = (row >> (7 - j)) & 1;
                self.pixels[y + i][x + j] ^= pixel == 1;
            }
        }

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::GREEN);

        for (i, row) in self.pixels.iter().enumerate() {
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

    pub fn event_poll(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                _ => {}
            }
        }

        return true;
    }

    pub fn debug_display(&self) {
        for row in self.pixels.iter() {
            for &pixel in row.iter() {
                if pixel {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
