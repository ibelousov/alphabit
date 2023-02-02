use std::io::{BufReader, Cursor};
use std::thread;
use rand::thread_rng;
use rodio::{Decoder, OutputStream, source::Source};

const CLICK_SOUND: &[u8] = include_bytes!("../../assets/sounds/click.wav");
const HIGH_WIN_SOUND: &[u8] = include_bytes!("../../assets/sounds/high_win.wav");
const LOW_WIN_SOUND: &[u8] = include_bytes!("../../assets/sounds/low_win.wav");
const UNDO_SOUND: &[u8] = include_bytes!("../../assets/sounds/undo.wav");

pub fn play_click() {
    // thread::spawn(|| {
    //     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //     let slice = Cursor::new(CLICK_SOUND.as_ref());
    //     let source = Decoder::new(slice).unwrap();
    //     let _sound_result = stream_handle.play_raw(source.convert_samples());
    //     //std::thread::sleep(std::time::Duration::from_millis(3000));
    // });
}

pub fn play_high_win() {
    thread::spawn(|| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let slice = Cursor::new(HIGH_WIN_SOUND.as_ref());
        let source = Decoder::new(slice).unwrap();
        let _sound_result = stream_handle.play_raw(source.convert_samples());
        std::thread::sleep(std::time::Duration::from_millis(3000));
    });
}

pub fn play_low_win() {
    thread::spawn(|| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let slice = Cursor::new(LOW_WIN_SOUND.as_ref());
        let source = Decoder::new(slice).unwrap();
        let _sound_result = stream_handle.play_raw(source.convert_samples());
        std::thread::sleep(std::time::Duration::from_millis(3000));
    });
}

pub fn play_undo() {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let slice = Cursor::new(UNDO_SOUND.as_ref());
        let source = Decoder::new(slice).unwrap();
        let _sound_result = stream_handle.play_raw(source.convert_samples());
        std::thread::sleep(std::time::Duration::from_millis(3000));
    });
}