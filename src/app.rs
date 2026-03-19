use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use eframe::egui::{self, Label, RichText, Sense, Vec2};
use crate::music::Packet;

pub enum StatusPacket {
    NextSong,
    Shuffle(Vec<usize>),
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
                    StatusPacket::Shuffle(vec) => {
                        let shuffled = vec.iter().map(|x| self.songs[*x].clone()).collect();
                        self.songs = shuffled;
                        self.index = 0;
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

        let mut tab_changed = false;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Controls").clicked() {
                    self.tab = Tab::Controls;
                    tab_changed = true;
                }
                if ui.button("Songs").clicked() {
                    self.tab = Tab::Songs;
                    tab_changed = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.add(egui::Label::new(&self.songs[self.index]).selectable(false).truncate().show_tooltip_when_elided(true));
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                Tab::Controls => {
                    let size = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0;
                    egui::Grid::new("controls").min_col_width(size).num_columns(2).show(ui, |ui| {
                        if ui.add_sized([size, 20.0], egui::Button::new(match self.paused {
                            true => "Play",
                            false => "Pause",
                        })).clicked() {
                            self.sender.send(Packet::Pause).unwrap();
                            self.paused = !self.paused;
                        }
                        if ui.add_sized([size, 20.0], egui::Button::new(match self.looping {
                            true => "Don't Loop",
                            false => "Loop",
                        })).clicked() {
                            self.sender.send(Packet::Loop).unwrap();
                            self.looping = !self.looping;
                        }
                        ui.end_row();
                        if ui.add_sized([size, 20.0], egui::Button::new("Skip")).clicked() {
                            self.sender.send(Packet::Skip).unwrap();
                        }
                        if ui.add_sized([size, 20.0], egui::Button::new("Shuffle")).clicked() {
                            self.sender.send(Packet::Shuffle).unwrap();
                        }
                    });
                    ui.spacing_mut().slider_width = ui.available_width();
                    if ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).show_value(false)).changed() {
                        self.sender.send(Packet::Volume(self.volume.powi(2) * 1.5)).unwrap();
                    }
                }
                Tab::Songs => {
                    egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                        egui::Grid::new("songs").show(ui, |ui| {
                            for (i, song) in self.songs.iter().enumerate() {
                                if i == self.index {
                                    let response = ui.add(Label::new(RichText::new(song).strong()).selectable(false));
                                    if tab_changed {
                                        response.scroll_to_me(Some(egui::Align::Min));
                                    }
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
    native_options.viewport.inner_size = Some(Vec2::new(275., 98.));
    eframe::run_native(
        "Music Player",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, sender, receiver, songs)))),
    )
}