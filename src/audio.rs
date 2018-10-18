use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::audio::AudioDevice;
use sdl2::AudioSubsystem;

struct Beep {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for Beep {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = self.volume * if self.phase < 0.5 { 1.0 } else { -1.0 };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Audio {
    device: AudioDevice<Beep>,
}

impl Audio {
    pub fn new(audio_subsystem: &AudioSubsystem) -> Self {
        let spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None
        };

        let device = audio_subsystem.open_playback(None, &spec, |spec| {
            Beep {
                phase_inc: 221.0 / spec.freq as f32,
                phase: 440.0 / spec.freq as f32,
                volume: 0.05
            }
        }).unwrap();

        Audio {
            device: device
        }
    }

    pub fn start_beep(&mut self) {
        self.device.resume();
    }

    pub fn stop_beep(&mut self) {
        self.device.pause();
    }
}
