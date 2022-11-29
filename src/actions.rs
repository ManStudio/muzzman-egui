use crate::{edit_type::edit_value, tab::Tab, Session};

use eframe::{egui::Id, epaint::Vec2};
use egui_kman_additions::table::*;
use muzzman_lib_internals::prelude::*;

pub struct ActionsTab {
    values: Vec<(String, Value)>,
    last: usize,
}

impl ActionsTab {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            last: usize::MAX,
        }
    }
}

impl Tab for ActionsTab {
    fn get_name(&self) -> &str {
        "Actions"
    }

    fn init(&mut self, _storage: &mut crate::storage::Storage) {}

    fn draw(&mut self, ui: &mut eframe::egui::Ui, storage: &mut crate::storage::Storage) {
        if storage.get::<Session>().unwrap().is_some() {
            let session = storage.get_mut::<Session>().unwrap();
            if let Some(session) = session {
                ui.vertical(|ui| {
                    let len = session.get_actions_len().unwrap();
                    let actions = session.get_actions(0..len).unwrap();
                    let res = ui.allocate_ui(
                        Vec2::new(ui.available_width(), ui.available_height() - 25.0),
                        |ui| {
                            Table::new()
                                .column(Column::new(
                                    "Name",
                                    |ui, value: &(String, MInfo, Vec<(String, Value)>)| {
                                        ui.label(&value.0);
                                    },
                                ))
                                .column(Column::new(
                                    "Module Name",
                                    |ui, value: &(String, MInfo, Vec<(String, Value)>)| {
                                        ui.label(&value.1.get_name().unwrap());
                                    },
                                ))
                                .set_values(actions.clone())
                                .show(ui)
                        },
                    );
                    let res = res.inner;
                    if let Some(selected) = res.selected {
                        if let Some(action) = actions.get(selected) {
                            if self.last != selected {
                                self.values = action.2.clone();
                                self.last = selected;
                            }
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    for (i, value) in self.values.iter_mut().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.label(&value.0);
                                            ui.label(":");
                                            edit_value(
                                                ui,
                                                &mut value.1,
                                                Id::new(format!("action{}", i)),
                                            );
                                        });
                                    }
                                });
                                if ui.button("Reset").clicked() {
                                    self.values = action.2.clone();
                                }
                                if ui.button("Run").clicked() {
                                    let mut values = Vec::new();
                                    for value in self.values.iter() {
                                        values.push(value.1.value.clone());
                                    }
                                    let _ = session.run_action(
                                        action.1.clone(),
                                        action.0.clone(),
                                        values,
                                    );
                                }
                            });
                        }
                    }
                });
            }
        }
    }
}
