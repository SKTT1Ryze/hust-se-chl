//! Test for mp3-metadata Crate

extern crate mp3_metadata;

fn main() {
    println!("Hello, mp3-metadata!");
    let meta = mp3_metadata::read_from_file("./mp3/Ref-rain.mp3").unwrap();
    println!("Number of frames: {}", meta.frames.len());
    println!("First 5 frames information:");
    for frame in meta.frames[0..5].iter() {
        println!("========== NEW FRAME ==========");
        println!("size:                 {}", frame.size);
        println!("version:              {:?}", frame.version);
        println!("layer:                {:?}", frame.layer);
        println!("bitrate:              {} Kb/s", frame.bitrate);
        println!("sampling frequency:   {} Hz", frame.sampling_freq);
        println!("channel type:         {:?}", frame.chan_type);
        println!("copyright:            {:?}", frame.copyright);
        println!("status:               {:?}", frame.status);
        println!("emphasis:             {:?}", frame.emphasis);
    }

    println!("\n========== TAGS ==========");
    if let Some(tag) = meta.tag {
        println!("title: {}", tag.title);
        println!("artist: {}", tag.artist);
        println!("album: {}", tag.album);
        println!("year: {}", tag.year);
        println!("comment: {}", tag.comment);
        println!("genre: {:?}", tag.genre);
    } else {
        println!("No tag");
    }
}
