pub mod music {

    use futures::executor::block_on;
    use rodio::source::SamplesConverter;
    use rodio::source::{SineWave, Source};
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub struct Sound {
        filename: String,
        source: Decoder<BufReader<File>>,
    }

    impl Sound {
        pub fn new(filename: &str) -> Self {
            let file = BufReader::new(File::open(filename).unwrap());
            let source = Decoder::new(file).unwrap();
            Sound {
                filename: filename.to_string(),
                source: source,
            }
        }
    }

    pub fn main_menu_music(tx: mpsc::Receiver<Sound>) {
        // Make Stream Handle
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Open Music File

        loop {
            for val in tx.recv().iter() {
                let file = BufReader::new(File::open(val.filename.clone()).unwrap());
                let source = Decoder::new(file).unwrap();
                // Decode the Music File
                let music_file = stream_handle.play_raw(source.convert_samples());
            }
        }

        // Set length or end point for music stream
        std::thread::sleep(std::time::Duration::from_secs(100));
    }

    pub fn in_game_music() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
    }
}
