//! Test for ridio crate

use std::{
    fs::File,
    io::BufReader,
    thread,
    time::Duration,
};

fn main() {
    println!("Hello, ridio!");
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let file_0 = File::open("./mp3/CLICK.mp3").unwrap();
    let file_1 = File::open("./mp3/CLICK.mp3").unwrap();
    let source_0 = rodio::Decoder::new(BufReader::new(file_0)).unwrap();
    let source_1 = rodio::Decoder::new(BufReader::new(file_1)).unwrap();
    sink.append(source_0);
    sink.append(source_1);
    sink.set_volume(0.2);
    thread::sleep(Duration::from_millis(3000));
    sink.pause();
    thread::sleep(Duration::from_millis(3000));
    sink.play();
    sink.sleep_until_end();
}
