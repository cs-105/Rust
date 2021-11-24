pub mod music{

    use std::fs::File;
    use std::io::BufReader;
    use std::time::Duration;
    use std::thread;
    use rodio::{Decoder, OutputStream, Sink};
    use rodio::source::{SineWave, Source};
    use futures::executor::block_on;

        pub fn main_menu_music(){
            

            thread::spawn(|| {
            // Make Stream Handle
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

            // Open Music File
            let file = BufReader::new(File::open("assets/Sample.ogg").unwrap());

            let source = Decoder::new(file).unwrap();

            // Decode the Music File
            let music_file = stream_handle.play_raw(source.convert_samples());

            // Set length or end point for music stream
            std::thread::sleep(std::time::Duration::from_secs(10));

            thread::sleep(Duration::from_millis(500));
        });
    }

        pub fn in_game_music(){

            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            
            let file = BufReader::new(File::open("assets/Sample.ogg").unwrap());
            let source = Decoder::new(file).unwrap();
            // Add a dummy source of the sake of the example.
            
            sink.append(source);
            
            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            sink.sleep_until_end();
        }
}
