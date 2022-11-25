use eframe::egui::{self, Ui};
use muzzman_lib_internals::prelude::*;

use crate::{storage::Storage, tab::Tab, Session};

pub struct SelectedLocation {
    pub selected: Option<LInfo>,
}

pub struct LocationsTab {}

impl Tab for LocationsTab {
    fn get_name(&self) -> &str {
        "locations"
    }

    fn init(&mut self, storage: &mut crate::storage::Storage) {
        storage.insert(SelectedLocation { selected: None });
    }

    fn draw(&mut self, ui: &mut eframe::egui::Ui, storage: &mut crate::storage::Storage) {
        if let Some(session) = storage.get::<Session>().unwrap() {
            self.draw_locations(ui, storage, session.get_default_location().unwrap());
        }
    }
}

impl LocationsTab {
    pub fn new() -> Self {
        Self {}
    }

    fn draw_locations(&mut self, ui: &mut Ui, storage: &mut Storage, location: LInfo) {
        let is_selected;

        {
            if let Some(l) = &storage.get::<SelectedLocation>().unwrap().selected {
                is_selected = *l.read().unwrap() == *location.read().unwrap();
            } else {
                is_selected = false;
            }
        }

        let res = egui::CollapsingHeader::new(location.get_name().unwrap())
            .selectable(true)
            .selected(is_selected)
            .show(ui, |ui| {
                for location in location
                    .get_locations(0..location.get_locations_len().unwrap())
                    .unwrap()
                {
                    self.draw_locations(ui, storage, location)
                }
            });

        if res.header_response.clicked() {
            storage.get_mut::<SelectedLocation>().unwrap().selected = Some(location.clone());
        }
    }
}
