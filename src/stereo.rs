use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use gamuboy::{
    apu::{self},
    stereo::StereoPlayer,
};
use sdl2::audio::AudioSpecDesired;

struct AudioBuffer {
    samples: VecDeque<f32>,
}

impl AudioBuffer {
    pub fn new() -> Self {
        Self {
            samples: VecDeque::with_capacity(apu::SAMPLES_BUFFER_SIZE),
        }
    }

    pub fn push_samples(&mut self, data: &[f32]) {
        self.samples.extend(data);
    }

    pub fn pop_samples(&mut self, count: usize) -> Vec<f32> {
        self.samples
            .drain(..count.min(self.samples.len()))
            .collect()
    }
}

struct StereoCallback {
    buffer: AudioBuffer,
}

impl sdl2::audio::AudioCallback for StereoCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let samples = self.buffer.pop_samples(out.len());
        for (i, sample) in samples.iter().enumerate() {
            out[i] = *sample;
        }
    }
}

pub struct SdlStereoPlayer {
    device: Option<Rc<RefCell<sdl2::audio::AudioDevice<StereoCallback>>>>,
}

impl SdlStereoPlayer {
    pub fn new(audio_subsystem: &sdl2::AudioSubsystem) -> Self {
        let desired_spec = AudioSpecDesired {
            freq: Some(apu::SAMPLE_RATE as i32),
            channels: Some(2),
            samples: Some(apu::SAMPLES_BUFFER_SIZE as u16),
        };

        let device =
            audio_subsystem.open_playback(None, &desired_spec, move |_spec| StereoCallback {
                buffer: AudioBuffer::new(),
            });

        let device = match device {
            Ok(device) => {
                device.resume();
                Some(Rc::new(RefCell::new(device)))
            }
            Err(err) => {
                println!("could not open audio device: {}", err);
                None
            }
        };

        Self { device }
    }
}

impl StereoPlayer for SdlStereoPlayer {
    fn play(&self, buffer: &[f32]) {
        match &self.device {
            Some(device) => {
                let mut device = device.borrow_mut();

                let mut lock = device.lock();
                lock.buffer.push_samples(buffer);
            }

            None => {}
        }
    }
}
