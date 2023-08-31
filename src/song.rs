// use std::fs;
use lofty::{AudioFile, Probe};

pub struct Song {
    pub name: String,
    pub artist: String,
    pub duration: u64,
    pub path: String,
}

impl Song {
    pub fn new(name: String, artist: String, path: String) -> Self {
      let tagged_file = Probe::open(&path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

      let properties = tagged_file.properties();
      let duration = properties.duration().as_secs() as u64;

      Self {
          name,
          artist,
          duration,
          path,
      }
    }
}