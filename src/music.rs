use std::{fs::File, io::BufReader, sync::mpsc::{Receiver, RecvTimeoutError}, time::Duration};
use rodio::{Decoder, Sink};

pub enum Packet {
    Play(usize),
    Stop,
    Skip,
    Continue,
}

pub struct Player {
    songs: Vec<String>,
    sink: Sink,
    receiver: Receiver<Packet>,
    index: usize,
}

impl Player {
    pub fn new(song_paths: Vec<String>, sink: Sink, receiver: Receiver<Packet>) -> Self {
        Self { songs: song_paths, sink, receiver, index: 0 }
    }

    fn queue_song(&self, song_index: usize) {
        let file = BufReader::new(File::open(&self.songs[song_index]).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
    }

    fn play(&mut self, song_index: usize) {
        self.stop();
        self.queue_song(song_index);
        self.index = song_index;
    }

    fn stop(&self) {
        self.sink.stop();
    }

    fn skip(&self) {
        self.sink.skip_one();
    }

    fn next(&mut self) {
        self.index += 1;
        if self.index == self.songs.len() {
            self.index = 0;
        }
        self.queue_song(self.index);
    }

    pub fn main_loop(&mut self) {
        loop {
            match self.receiver.recv_timeout(Duration::from_millis(100)) {
                Ok(packet) => {
                    match packet {
                        Packet::Play(song) => self.play(song),
                        Packet::Skip => self.skip(),
                        Packet::Stop => self.stop(),
                        Packet::Continue => self.next(),
                    }
                }
                Err(error) => {
                    match error {
                        RecvTimeoutError::Disconnected => break,
                        RecvTimeoutError::Timeout => {}
                    }
                }
            }
            if self.sink.len() == 1 {
                self.next();
            }
        }
    }
}