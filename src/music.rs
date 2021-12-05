pub mod music {

    use futures::executor::block_on;
    use rodio::source::{SineWave, Source};
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::thread;
    use std::time::Duration;

    pub fn main_menu_music() {
        // Make Stream Handle
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();



        // Open Music File
        let file = BufReader::new(File::open("assets/Asteroids_MAIN_MENU.mp3").unwrap());

        let source = Decoder::new(file).unwrap();

        // Decode the Music File
        let music_file = stream_handle.play_raw(source.convert_samples());

        // Set length or end point for music stream
        std::thread::sleep(std::time::Duration::from_secs(10));

        thread::sleep(Duration::from_millis(500));
    }

    pub fn in_game_music() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let file = BufReader::new(File::open("assets/Asteroids_GAME.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.sleep_until_end();        
        

        // sink.sleep_until_end();
    }

    pub fn laser_sound() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let file2 = BufReader::new(File::open("assets/Laser_Sound.mp3").unwrap());
        let source2 = Decoder::new(file2).unwrap();
        sink.append(source2);
        sink.sleep_until_end();
         
        

            // sink.sleep_until_end();
    }
}
