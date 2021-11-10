use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

pub fn main(){
// Make Stream Handle
let (_stream, stream_handle) = OutputStream::try_default().unwrap();

// Open Music File
let file = BufReader::new(File::open("assets/Sample.mp3").unwrap());

{let source = Decoder::new(file).unwrap();}

fn main2(
// Decode the Music File
return {stream_handle.play_raw(source.convert_samples());});
};