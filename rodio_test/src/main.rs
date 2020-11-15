//! Test for ridio crate

use std::{
    fs::File,
    io::BufReader,
};
use rodio::Source;

fn main() {
    println!("Hello, ridio!");
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = File::open("./mp3/CLICK.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples());

    loop {}
}
