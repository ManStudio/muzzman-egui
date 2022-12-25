use crate::{tab::Tab, Session};
use egui_kman_additions::table::*;
use muzzman_lib::prelude::*;

#[derive(Default)]
pub struct Modules {
    pub selected: Option<MInfo>,
}

pub struct ModulesTab {}

impl ModulesTab {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tab for ModulesTab {
    fn get_name(&self) -> &str {
        "Modules"
    }

    fn init(&mut self, storage: &mut crate::storage::Storage) {
        storage.insert(Modules::default());
    }

    fn draw(&mut self, ui: &mut eframe::egui::Ui, storage: &mut crate::storage::Storage) {
        let selected;
        let modules;
        if let Some(session) = storage.get::<Session>().unwrap() {
            let len = session.get_modules_len().unwrap();
            modules = session.get_modules(0..len).unwrap();
            selected = Table::new_with("modules")
                .set_values(modules.clone())
                .column(Column::new("Name", |ui, value: &MInfo| {
                    ui.label(value.get_name().unwrap());
                }))
                .column(Column::new("Proxy", |ui, value| {
                    ui.label(format!("{}", value.get_proxy().unwrap()));
                }))
                .column(Column::new("Desc", |ui, value| {
                    ui.label(value.get_desc().unwrap());
                }))
                .show(ui)
                .selected;
        } else {
            selected = None;
            modules = Vec::new();
        }

        if let Some(m) = storage.get_mut::<Modules>() {
            match selected {
                Some(selected) => {
                    if let Some(module) = modules.get(selected) {
                        m.selected = Some(module.clone());
                    } else {
                        m.selected = None;
                    }
                }
                None => m.selected = None,
            }
        }
    }
}
