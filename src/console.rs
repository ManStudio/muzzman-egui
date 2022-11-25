use eframe::{
    egui::{self, Widget},
    epaint::Color32,
};

use crate::tab::Tab;

pub struct Console {
    pub data: Vec<(u8, String)>,
}

impl Console {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn debug(&mut self, text: &str) {
        self.data.push((LogType::Debug, text.to_owned()))
    }

    pub fn warn(&mut self, text: &str) {
        self.data.push((LogType::Warning, text.to_owned()))
    }

    pub fn error(&mut self, text: &str) {
        self.data.push((LogType::Error, text.to_owned()))
    }

    pub fn info(&mut self, text: &str) {
        self.data.push((LogType::Info, text.to_owned()))
    }
}

pub mod LogType {
    pub const Debug: u8 = 1 << 0;
    pub const Warning: u8 = 1 << 1;
    pub const Error: u8 = 1 << 2;
    pub const Info: u8 = 1 << 3;
}

pub struct ConsoleTab {
    shows: u8,
}

impl Tab for ConsoleTab {
    fn get_name(&self) -> &str {
        "console"
    }

    fn init(&mut self, storage: &mut crate::storage::Storage) {
        storage.insert(Console::new());
        let s = storage.get_mut::<Console>().unwrap();
    }

    fn draw(&mut self, ui: &mut eframe::egui::Ui, storage: &mut crate::storage::Storage) {
        {
            use LogType::*;
            let mut debug = self.shows & Debug == Debug;
            let mut warning = self.shows & Warning == Warning;
            let mut error = self.shows & Error == Error;
            let mut info = self.shows & Info == Info;

            egui::menu::bar(ui, |ui| {
                ui.checkbox(&mut debug, "debug");
                ui.checkbox(&mut warning, "warning");
                ui.checkbox(&mut error, "error");
                ui.checkbox(&mut info, "info");
            });

            let mut tmp_shows = 0;
            if debug {
                tmp_shows |= Debug;
            }
            if warning {
                tmp_shows |= Warning;
            }
            if error {
                tmp_shows |= Error;
            }
            if info {
                tmp_shows |= Info
            }

            self.shows = tmp_shows;
        }

        let console: &Console = storage.get().unwrap();
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show_rows(
                ui,
                ui.text_style_height(&egui::TextStyle::Body),
                console.data.len(),
                |ui, range| {
                    for (ty, value) in console.data[range].iter() {
                        if ty & self.shows != *ty {
                            continue;
                        }

                        if ty & LogType::Debug == *ty {
                            egui::Label::new(
                                egui::RichText::new(format!("Debug: {}", value))
                                    .color(Color32::from_rgb(0, 0, 255)),
                            )
                            .wrap(false)
                            .ui(ui);
                        } else if ty & LogType::Error == *ty {
                            egui::Label::new(
                                egui::RichText::new(format!("Error: {}", value))
                                    .color(Color32::from_rgb(255, 0, 0)),
                            )
                            .wrap(false)
                            .ui(ui);
                        } else if ty & LogType::Info == *ty {
                            egui::Label::new(
                                egui::RichText::new(value).color(Color32::from_rgb(255, 255, 255)),
                            )
                            .wrap(false)
                            .ui(ui);
                        } else if ty & LogType::Warning == *ty {
                            egui::Label::new(
                                egui::RichText::new(format!("Warning: {}", value))
                                    .color(Color32::from_rgb(255, 255, 0)),
                            )
                            .wrap(false)
                            .ui(ui);
                        }
                    }
                },
            );
    }
}

impl ConsoleTab {
    pub fn new() -> Self {
        use LogType::*;
        Self {
            shows: Debug | Warning | Error | Info,
        }
    }
}
