public fn music(){

// Make Stream Handle
let (_stream, stream_handle) = OutputStream::try_default().unwrap();

// Open Music File
let file = BufReader::new(File::open("assets/Sample.ogg").unwrap());

let source = Decoder::new(file).unwrap();

// Decode the Music File
stream_handle.play_raw(source.convert_samples());

std::thread::sleep(std::time::Duration::from_secs(5));

}