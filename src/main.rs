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
    let mut files = vec![];
    let mut names = vec![];
    for path in fs::read_dir(&config.song_folder).unwrap().map(|x| x.unwrap().path()).filter(|x| x.extension().is_some_and(|x| x == "mp3")) {
        files.push(path.to_str().unwrap().to_owned());
        names.push(path.file_stem().unwrap().to_str().unwrap().to_owned());
    }
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let (tx, rx) = channel();
    let (tx_status, rx_status) = channel();
    thread::spawn(move|| {
        let mut player = Player::new(files, sink, rx, tx_status);
        player.main_loop();
    });
    let _ = app::main(tx, rx_status, names);
}
