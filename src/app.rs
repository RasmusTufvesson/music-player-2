use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use eframe::egui::{self, Label, RichText, Sense, Vec2};
use crate::music::Packet;

pub enum StatusPacket {
    NextSong,
}

#[derive(PartialEq)]
enum Tab {
    Controls,
    Songs,
}

pub struct App {
    sender: Sender<Packet>,
    receiver: Receiver<StatusPacket>,
    volume: f32,
    looping: bool,
    paused: bool,
    songs: Vec<String>,
    index: usize,
    tab: Tab,
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
            tab: Tab::Controls,
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Controls").clicked() {
                    self.tab = Tab::Controls;
                }
                if ui.button("Songs").clicked() {
                    self.tab = Tab::Songs;
                }
            });
        });
        if self.tab == Tab::Controls {
            egui::SidePanel::left("side_panel").exact_width(20.).resizable(false).show(ctx, |ui| {
                ui.add_space(ui.spacing().item_spacing.y);
                ui.spacing_mut().slider_width = ui.available_height() - ui.spacing().item_spacing.y;
                if ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.5).show_value(false).vertical()).changed() {
                    self.sender.send(Packet::Volume(self.volume)).unwrap();
                }
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                Tab::Controls => {
                    ui.with_layout(egui::Layout { main_dir: egui::Direction::TopDown, main_wrap: false, main_align: eframe::emath::Align::Min, main_justify: false, cross_align: eframe::emath::Align::Center, cross_justify: true }, |ui: &mut egui::Ui| {
                        ui.add(Label::new(RichText::new(&self.songs[self.index]).heading()).selectable(false).truncate(true));
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
                }
                Tab::Songs => {
                    egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                        egui::Grid::new("songs").show(ui, |ui| {
                            for (i, song) in self.songs.iter().enumerate() {
                                if i == self.index {
                                    ui.add(Label::new(RichText::new(song).strong()).selectable(false));
                                } else {
                                    if ui.add(Label::new(song).sense(Sense::click()).selectable(false)).clicked() {
                                        self.sender.send(Packet::Play(i)).unwrap();
                                        self.index = i;
                                    }
                                }
                                ui.end_row();
                            }
                        });
                    });
                }
            }
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