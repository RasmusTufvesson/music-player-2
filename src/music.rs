use std::{fs::File, io::BufReader, sync::mpsc::{Receiver, RecvTimeoutError, Sender}, time::Duration};
use rodio::{Decoder, Sink};

use crate::app::StatusPacket;

pub enum Packet {
    Play(usize),
    Skip,
    Pause,
    Volume(f32),
    Loop,
}

pub struct Player {
    songs: Vec<String>,
    sink: Sink,
    receiver: Receiver<Packet>,
    sender: Sender<StatusPacket>,
    index: usize,
    looping: bool,
}

impl Player {
    pub fn new(song_paths: Vec<String>, sink: Sink, receiver: Receiver<Packet>, sender: Sender<StatusPacket>) -> Self {
        let player = Self { songs: song_paths, sink, receiver, sender, index: 0, looping: false };
        player.pause();
        player.queue_song(0);
        player
    }

    fn queue_song(&self, song_index: usize) {
        let file = BufReader::new(File::open(&self.songs[song_index]).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
    }

    fn play(&mut self, song_index: usize) {
        self.sink.stop();
        self.queue_song(song_index);
        self.index = song_index;
    }

    fn skip(&mut self) {
        self.sink.skip_one();
        self.next();
    }

    fn next(&mut self) {
        if !self.looping {
            self.sender.send(StatusPacket::NextSong).unwrap();
            self.index += 1;
            if self.index == self.songs.len() {
                self.index = 0;
            }
        }
        self.queue_song(self.index);
    }

    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    fn change_looping(&mut self) {
        self.looping = !self.looping;
    }

    fn pause(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            match self.receiver.recv_timeout(Duration::from_millis(100)) {
                Ok(packet) => {
                    match packet {
                        Packet::Play(song) => self.play(song),
                        Packet::Skip => self.skip(),
                        Packet::Pause => self.pause(),
                        Packet::Volume(vol) => self.set_volume(vol),
                        Packet::Loop => self.change_looping(),
                    }
                }
                Err(error) => {
                    match error {
                        RecvTimeoutError::Disconnected => break,
                        RecvTimeoutError::Timeout => {}
                    }
                }
            }
            if self.sink.len() == 0 {
                self.next();
            }
        }
    }
}