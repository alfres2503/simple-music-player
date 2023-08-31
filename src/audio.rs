use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
// use rodio::source::{SineWave, Source};

use crate::song::Song;

pub struct Audio {
    pub stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
    pub sink: Sink,
}

impl Audio {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            stream,
            stream_handle,
            sink,
        }
    }

    pub fn play(&self, song: &Song) {
        let file = 
            BufReader::new(File::open(&song.path)
            .unwrap());

        let source = Decoder::new(file).unwrap();

        self.sink.set_volume(0.1);

        self.sink.append(source);

        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("Pausing song");
        self.pause();

        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("Resuming song");
        self.resume();

        self.sink.sleep_until_end();
    }

    pub fn pause(&self) {
        self.sink.pause();

        
        self.sink.is_paused();
    }

    pub fn resume(&self) {
        if self.sink.empty() {
            println!("Song has ended");
        } else if self.sink.is_paused() {
            self.sink.play();
        } else {
            println!("Song is not paused");
        }
    }
}
