mod audio;
mod song;
mod ui;

use crate::song::Song;

fn main() {
    println!("Welcome to rust music player!");

    let song = Song::new(
        String::from("Reverie"),
        String::from("Polyphia"),
        String::from("C:/Users/Alfredo/Desktop/music-player/assets/music/polyphia_reverie.mp3"),
    ); 

    println!("Playing {} by {} for {} seconds", song.name, song.artist, song.duration);
    println!("Press Ctrl+C to stop");
    // audio::play_audio(&song);

    let audio = audio::Audio::new();
    audio.play(&song);

    // ui::run();
}
