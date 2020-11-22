//! Test for ridio crate

use std::{
    fs::File,
    io::BufReader,
    thread,
    time::Duration,
};
use rodio::Source;
fn main() {
    println!("Hello, ridio!");
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let file = File::open("./mp3/CLICK.mp3").unwrap();
    let data_0 = BufReader::new(file.try_clone().unwrap());
    // let data_1 = BufReader::new(file.try_clone().unwrap());
    let source = rodio::Decoder::new(data_0).unwrap().buffered();
    // let source_0 = rodio::Decoder::new(data_0).unwrap();
    // let source_1 = rodio::Decoder::new(data_1).unwrap();
    // sink.append(source_0);
    // sink.append(source_1);
    sink.append(source.clone());
    sink.append(source.clone());
    sink.set_volume(0.2);
    thread::sleep(Duration::from_millis(3000));
    sink.pause();
    thread::sleep(Duration::from_millis(3000));
    sink.play();
    sink.sleep_until_end();
}
