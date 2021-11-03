use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

// Make Stream Handle
let (_stream, stream_handle) = OutputStream::try_default().unwrap();

// Open Music File
let file = BufReader::new(File::open("music_files/sample.ogg").unwrap());

// Decode the Music File
let source = Decoder::new(file).unwrap();

// Directly Place the sound from Music File.
stream_handle.play_raw(source.convert_samples());

// Keep main thread going while loop is active for playback.
std::thread::sleep(std::time::Duration::from_secs(5));