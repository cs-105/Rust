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
        let file = BufReader::new(File::open("assets/Sample.mp3").unwrap());

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
    }
}
