use std::thread;
use rand::thread_rng;
use rodio::{Decoder, OutputStream, source::Source};

pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate: sample_rate,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32
            / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        return None;
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}

pub fn play_sound() {
    thread::spawn(|| {
        let mut wave_table: Vec<f32> =  Vec::with_capacity(64);
        for n in 0..64 {
            wave_table.push((2.0 * std::f32::consts::PI * n as f32 / 64 as f32).sin());
        }
        let mut oscillator = WavetableOscillator::new(44100, wave_table);
        oscillator.set_frequency(220.0);
        let (_stream,stream_handle) = OutputStream::try_default().unwrap();
        let _result = stream_handle.play_raw(oscillator.convert_samples());

        std::thread::sleep(std::time::Duration::from_millis(200));
    });
}