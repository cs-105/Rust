pub mod music{

    use std::fs::File;
    use std::io::BufReader;
    use std::time::Duration;
    use rodio::{Decoder, OutputStream, Sink};
    use rodio::source::{SineWave, Source};

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

        pub fn music2(){

            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            
            // Add a dummy source of the sake of the example.
            let source = SineWave::new(440).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
            sink.append(source);
            
            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            sink.sleep_until_end();
        }
}
