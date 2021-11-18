pub mod music{

    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, source::Source};

        pub fn music(){

            // Make Stream Handle
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

            // Open Music File
            let file = BufReader::new(File::open("assets/Sample.ogg").unwrap());

            let source = Decoder::new(file).unwrap();

            // Decode the Music File
            let music_file = stream_handle.play_raw(source.convert_samples());

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
}
