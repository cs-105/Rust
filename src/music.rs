pub mod music {
    use rodio::source::Source;
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::sync::mpsc;

    pub struct SoundCommand {}

    pub enum SoundType {
        Music,
        SoundEffect,
    }

    pub struct Sound {
        filename: String,
        source: BufReader<File>,
        sound_type: SoundType,
    }

    impl Sound {
        pub fn new(filename: &str, sound_type: SoundType) -> Self {
            let file = BufReader::new(File::open(filename).unwrap());
            Sound {
                filename: filename.to_string(),
                source: file,
                sound_type: sound_type,
            }
        }
    }

    pub fn start_sound_thread(tx: mpsc::Receiver<Sound>) {
        // Make Stream Handle
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut sink = Sink::try_new(&stream_handle).unwrap();

        // Open Music File
        loop {
            // Receive new messages in thread
            for val in tx.recv().iter() {
                match val.sound_type {
                    // If music, then stop the music currently playing and start again
                    SoundType::Music => {
                        let file = BufReader::new(File::open(val.filename.clone()).unwrap());
                        let source = Decoder::new(file).unwrap();
                        sink.stop();
                        drop(&sink);
                        sink = Sink::try_new(&stream_handle).unwrap();
                        sink.set_volume(4.0);
                        // Decode the Music File
                        sink.append(source);
                        sink.play();
                    }
                    // If sound effect, then just play it until it ends without stopping the music
                    SoundType::SoundEffect => {
                        let file = BufReader::new(File::open(val.filename.clone()).unwrap());
                        let source = Decoder::new(file).unwrap();
                        stream_handle.play_raw(source.convert_samples());
                    }
                }
            }
        }
    }
}
