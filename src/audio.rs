use std::f32::consts::PI;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};

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

struct Beep {
    device: AudioDevice<SineWave>,
}

impl Beep {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("could not initialize sdl!");
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

        Beep { device }
    }

    pub fn is_playing(&self) -> bool {
        self.device.status() == AudioStatus::Playing
    }

    pub fn play(&self) {
        self.device.resume();
    }

    pub fn stop(&self) {
        self.device.pause();
    }
}

pub struct Audio {
    pub sound_timer: u8,
    beep: Beep,
}

impl Audio {
    pub fn new(sound_timer: u8) -> Self {
        Self {
            sound_timer,
            beep: Beep::new(),
        }
    }

    pub fn update(&mut self) {
        // Audio
        if self.beep.is_playing() && self.sound_timer == 0 {
            self.beep.stop();
        }

        if !self.beep.is_playing() && self.sound_timer > 0 {
            self.beep.play();
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
