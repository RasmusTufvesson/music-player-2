use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use eframe::egui::{self, Vec2};
use crate::music::Packet;

pub enum StatusPacket {
    NextSong,
}

pub struct App {
    sender: Sender<Packet>,
    receiver: Receiver<StatusPacket>,
    volume: f32,
    looping: bool,
    paused: bool,
    songs: Vec<String>,
    index: usize,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>, sender: Sender<Packet>, receiver: Receiver<StatusPacket>, songs: Vec<String>) -> Self {
        return Self {
            sender,
            receiver,
            volume: 1.0,
            looping: false,
            paused: true,
            songs,
            index: 0,
        };
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.receiver.try_recv() {
            Ok(packet) => {
                match packet {
                    StatusPacket::NextSong => {
                        self.index += 1;
                        if self.index == self.songs.len() {
                            self.index = 0;
                        }
                    }
                }
            }
            Err(error) => {
                match error {
                    TryRecvError::Disconnected => {
                        panic!("Player status channel disconnected")
                    }
                    TryRecvError::Empty => {}
                }
            }
        }
        egui::SidePanel::left("side_panel").exact_width(20.).resizable(false).show(ctx, |ui| {
            ui.add_space(ui.spacing().item_spacing.y);
            ui.spacing_mut().slider_width = ui.available_height() - ui.spacing().item_spacing.y;
            if ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.5).show_value(false).vertical()).changed() {
                self.sender.send(Packet::Volume(self.volume)).unwrap();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout { main_dir: egui::Direction::TopDown, main_wrap: false, main_align: eframe::emath::Align::Min, main_justify: false, cross_align: eframe::emath::Align::Center, cross_justify: true }, |ui: &mut egui::Ui| {
                ui.heading(&self.songs[self.index]);
                if ui.button(match self.paused {
                    true => "Play",
                    false => "Pause",
                }).clicked() {
                    self.sender.send(Packet::Pause).unwrap();
                    self.paused = !self.paused;
                }
                if ui.button("Skip").clicked() {
                    self.sender.send(Packet::Skip).unwrap();
                }
                if ui.checkbox(&mut self.looping, "Loop").changed() {
                    self.sender.send(Packet::Loop).unwrap();
                }
            });
        });
    }
}

pub fn main(sender: Sender<Packet>, receiver: Receiver<StatusPacket>, songs: Vec<String>) -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.inner_size = Some(Vec2::new(300., 120.));
    eframe::run_native(
        "Music Player",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, sender, receiver, songs))),
    )
}