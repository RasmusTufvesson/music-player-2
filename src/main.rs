use std::{fs, sync::mpsc::channel, thread};
use music::Player;
use rodio::{OutputStream, Sink};
use serde::Deserialize;

mod music;
mod app;

#[derive(Deserialize)]
struct Config {
    song_folder: String,
}

fn main() {
    let config: Config = serde_json::from_str(&fs::read_to_string("config.json").unwrap()).unwrap();
    let files: Vec<String> = fs::read_dir(&config.song_folder).unwrap().map(|x| x.unwrap().path().to_str().unwrap().to_owned()).filter(|x| x.ends_with(".mp3")).collect();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let (tx, rx) = channel();
    let mut player = Player::new(files, sink, rx);
    thread::spawn(move|| {
        player.main_loop();
    });
    let _ = app::main(tx);
}
