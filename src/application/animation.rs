use std::time::{SystemTime, UNIX_EPOCH};

pub struct ColorGenerator {}

impl ColorGenerator {
    pub fn get_milliseconds() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    pub fn get_color_component(offset: u32, duration: u32, bottom: u8, top: u8) -> u8 {
        let m = (
            ((Self::get_milliseconds() + offset as u128) % duration as u128) as i128 -
                (duration as i128 / 2)
        ).abs() as u32;

        let divider = duration / 256;
        let range = top - bottom;
        let value = ((m / divider) * range as u32) / 256;

        (bottom + value as u8) as u8
    }
}