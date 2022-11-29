use crate::{locations::SelectedLocation, tab::Tab};
use egui_kman_additions::table::*;
use muzzman_lib_internals::prelude::*;

pub struct Elements {
    pub selected: Option<EInfo>,
}

pub struct ElementsTab {}

impl Tab for ElementsTab {
    fn get_name(&self) -> &str {
        "Elements"
    }

    fn init(&mut self, storage: &mut crate::storage::Storage) {
        storage.insert(Elements::new());
    }

    fn draw(&mut self, ui: &mut eframe::egui::Ui, storage: &mut crate::storage::Storage) {
        let elements;
        {
            let selected_location = storage.get::<SelectedLocation>().unwrap();
            if let Some(location) = &selected_location.selected {
                let len = location.get_elements_len().unwrap();
                elements = location.get_elements(0..len).unwrap();
            } else {
                elements = Vec::new();
            }
        }
        let res = Table::new_with("elements")
            .column(Column::new("Name", |ui, value: &EInfo| {
                ui.label(value.get_name().unwrap());
            }))
            .column(Column::new("Progress", |ui, value: &EInfo| {
                ui.label((value.get_progress().unwrap() * 100.0).to_string());
            }))
            .column(Column::new("Enabled", |ui, element| {
                let mut enabled = element.is_enabled().unwrap();
                ui.checkbox(&mut enabled, "");
            }))
            .column(Column::new("Status", |ui, element| {
                ui.label(element.get_status_msg().unwrap());
            }))
            .set_values(elements.clone())
            .show(ui);

        storage.get_mut::<Elements>().unwrap().selected = None;

        if let Some(index) = res.selected {
            if let Some(element) = elements.get(index) {
                storage.get_mut::<Elements>().unwrap().selected = Some(element.clone());
            }
        }
    }
}

impl ElementsTab {
    pub fn new() -> Self {
        Self {}
    }
}

impl Elements {
    pub fn new() -> Self {
        Self { selected: None }
    }
}
