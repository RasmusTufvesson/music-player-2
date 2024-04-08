use std::sync::mpsc::Sender;
use eframe::egui::{self, Vec2};
use crate::music::Packet;

pub struct App {
    sender: Sender<Packet>,
    volume: f32,
    looping: bool,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>, sender: Sender<Packet>) -> Self {
        return Self {
            sender,
            volume: 1.0,
            looping: false,
        };
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").exact_width(20.).resizable(false).show(ctx, |ui| {
            ui.add_space(ui.spacing().item_spacing.y);
            ui.spacing_mut().slider_width = ui.available_height() - ui.spacing().item_spacing.y;
            if ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.5).show_value(false).vertical()).changed() {
                self.sender.send(Packet::Volume(self.volume)).unwrap();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout { main_dir: egui::Direction::TopDown, main_wrap: false, main_align: eframe::emath::Align::Min, main_justify: false, cross_align: eframe::emath::Align::Center, cross_justify: true }, |ui: &mut egui::Ui| {
                ui.heading("Music Player");
                if ui.button("Play").clicked() {
                    self.sender.send(Packet::Continue).unwrap();
                }
                if ui.button("Stop").clicked() {
                    self.sender.send(Packet::Stop).unwrap();
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

pub fn main(sender: Sender<Packet>) -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.inner_size = Some(Vec2::new(300., 120.));
    eframe::run_native(
        "Music Player",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, sender))),
    )
}