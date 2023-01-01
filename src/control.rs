use eframe::egui;
use muzzman_lib::{local_session::session::LocalSession, prelude::*};

use crate::{console::Console, locations::SelectedLocation, tab::Tab, Session};

pub struct ControlTab {
    location_name: String,
    element_name: String,
}

impl Tab for ControlTab {
    fn get_name(&self) -> &str {
        "control"
    }

    fn init(&mut self, _storage: &mut crate::storage::Storage) {}

    fn draw(&mut self, ui: &mut eframe::egui::Ui, storage: &mut crate::storage::Storage) {
        if ui.button("create LocalSession").clicked() {
            *storage.get_mut::<Session>().unwrap() = Some(LocalSession::new());
            storage
                .get_mut::<Console>()
                .unwrap()
                .info("Local Session Instanciated");
        }

        if storage.get::<Session>().unwrap().is_some() {
            egui::Frame::group(&*ui.style()).show(ui, |ui| {
                ui.label(egui::RichText::new("Add Location").size(20.0));

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut self.location_name);
                });

                if ui.button("Add").clicked() {
                    if let Some(s) = &storage.get::<SelectedLocation>().unwrap().selected {
                        s.create_location(&self.location_name).unwrap();

                        storage
                            .get_mut::<Console>()
                            .unwrap()
                            .info(&format!("Location added: {}", self.location_name));
                    } else {
                        storage
                            .get_mut::<Console>()
                            .unwrap()
                            .error(&format!("No location is selected!!!"));
                    }
                }
            });
        }
        if storage.get::<Session>().unwrap().is_some() {
            egui::Frame::group(&*ui.style()).show(ui, |ui| {
                ui.label(egui::RichText::new("Modules").size(20.0));

                ui.separator();

                if ui.button("Load all modules from modules folder").clicked() {
                    let dir = std::fs::read_dir("modules").unwrap();
                    for entry in dir {
                        if let Ok(file) = entry {
                            match RawModule::new(file.path().to_str().unwrap()) {
                                Ok(raw_module) => {
                                    let module_name;
                                    {
                                        let session = storage.get_mut::<Session>().unwrap();
                                        if let Some(session) = session {
                                            let module = session.add_module(raw_module).unwrap();
                                            module_name = session.get_module_name(&module).unwrap();
                                        } else {
                                            module_name = String::new();
                                        }
                                    }

                                    storage
                                        .get_mut::<Console>()
                                        .unwrap()
                                        .info(&format!("Module loaded: {}", module_name));
                                }
                                Err(err) => storage.get_mut::<Console>().unwrap().error(&format!(
                                    "when loading lib: {}, error: {:?}",
                                    file.path().to_str().unwrap(),
                                    err
                                )),
                            }
                        }
                    }
                }
            });
        }
        if storage.get::<Session>().unwrap().is_some() {
            egui::Frame::group(&*ui.style()).show(ui, |ui| {
                ui.label(egui::RichText::new("Add Element").size(20.0));

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut self.element_name);
                });

                if ui.button("Add").clicked() {
                    if let Some(s) = &storage.get::<SelectedLocation>().unwrap().selected {
                        s.create_element(&self.element_name).unwrap();

                        storage
                            .get_mut::<Console>()
                            .unwrap()
                            .info(&format!("Element added: {}", self.location_name));
                    } else {
                        storage
                            .get_mut::<Console>()
                            .unwrap()
                            .error(&format!("No location is selected!!!"));
                    }
                }
            });
        }
    }
}

impl ControlTab {
    pub fn new() -> Self {
        Self {
            location_name: String::from("Location 1"),
            element_name: String::from("Element 1"),
        }
    }
}
