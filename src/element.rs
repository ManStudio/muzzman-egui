use eframe::egui::{self, Id, Layout, Ui, Widget};
use muzzman_lib_internals::prelude::*;

use crate::{
    console::Console, edit_value, elements::Elements, file_or_data::edit_file_or_data,
    modules::Modules, tab::Tab,
};

pub struct ElementTab {
    editing: bool,
    name: String,
    desc: String,
    meta: String,
    enabled: bool,
    element_data: Data,
    module_data: Data,
    data: FileOrData,
}

impl ElementTab {
    pub fn new() -> Self {
        Self {
            editing: false,
            name: String::new(),
            desc: String::new(),
            meta: String::new(),
            enabled: false,
            element_data: Data::default(),
            module_data: Data::default(),
            data: FileOrData::Bytes(Bytes::new()),
        }
    }

    fn draw_data(ui: &mut Ui, editing: bool, data: &mut Data, id: &str) {
        if editing {
            let mut to_remove = None;
            for (name, value) in data.iter_mut() {
                ui.with_layout(Layout::left_to_right(egui::Align::default()), |ui| {
                    ui.label(name);
                    ui.label("=");
                    edit_value(ui, value, Id::new(format!("{}{}", id, name)));
                    if ui.button("remove").clicked() {
                        to_remove = Some(name.clone());
                    }
                });
            }
            if let Some(to_remove) = to_remove {
                data.remove(&to_remove);
            }
            ui.separator();
            ui.with_layout(Layout::left_to_right(egui::Align::default()), |ui| {
                ui.label("key: ");
                let mut k = String::new();
                if let Some(key_value) = data.get_mut("__edit__key__") {
                    if let Type::String(key) = key_value {
                        k = key.clone();
                        ui.text_edit_singleline(key);
                    }
                } else {
                    data.set("__edit__key__", Type::String(String::from("url")));
                }
                ui.label("=");
                if let Some(value) = data.get_mut_value("__edit__value__") {
                    edit_value(ui, value, Id::new(format!("{}__edit_value__{}", k, id)))
                } else {
                    data.set("__edit__value__", Type::String(String::from("http://")));
                }
                if ui.button("Add").clicked() {
                    let mut key = None;
                    let mut value = None;

                    if let Some(k) = data.get("__edit__key__") {
                        if let Type::String(k) = k {
                            key = Some(k.clone());
                        }
                    }

                    if let Some(v) = data.get("__edit__value__") {
                        value = Some(v.clone());
                    }

                    if let Some(key) = key {
                        if let Some(value) = value {
                            data.set(&key, value);
                        }
                    }
                }
            });
        } else {
            for (name, value) in data.iter() {
                ui.horizontal(|ui| {
                    ui.label(name);
                    ui.label("=");
                    ui.label(value.value.to_tag().to_string())
                        .on_hover_text(&value.desc);
                    ui.label(value.value.to_string()).on_hover_text(&value.desc);
                });
            }
        }
    }
}

impl Tab for ElementTab {
    fn get_name(&self) -> &str {
        "Element"
    }

    fn init(&mut self, storage: &mut crate::storage::Storage) {}

    fn draw(&mut self, ui: &mut eframe::egui::Ui, storage: &mut crate::storage::Storage) {
        if let Some(element) = &storage.get::<Elements>().unwrap().selected.clone() {
            ui.horizontal(|ui| {
                ui.label("Name: ");
                if self.editing {
                    ui.text_edit_singleline(&mut self.name);
                } else {
                    ui.label(element.get_name().unwrap());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Desc: ");
                if self.editing {
                    ui.text_edit_singleline(&mut self.desc);
                } else {
                    ui.label(element.get_desc().unwrap());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Meta: ");
                if self.editing {
                    ui.text_edit_singleline(&mut self.meta);
                } else {
                    ui.label(element.get_meta().unwrap());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Data: ");
                edit_file_or_data(ui, &mut self.data);
            });
            ui.horizontal(|ui| {
                ui.label("Progress: ");
                ui.label(format!("{}%", element.get_progress().unwrap() * 100.0));
                egui::ProgressBar::new(element.get_progress().unwrap()).ui(ui);
            });
            ui.horizontal(|ui| {
                self.enabled = element.is_enabled().unwrap();

                if self.editing {
                    if ui.checkbox(&mut self.enabled, "Enabled").clicked() {
                        element.set_enabled(self.enabled).unwrap();
                    };
                } else {
                    ui.label("Enabled: ");
                    if element.is_enabled().unwrap() {
                        ui.label("True");
                    } else {
                        ui.label("False");
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("Status: ");
                ui.label(element.get_status_msg().unwrap());
                if self.editing {
                    if ui.button("Set status 0").clicked() {
                        element.set_status(0).unwrap();
                    }
                }
            });

            if let Some(module) = element.get_module().unwrap() {
                ui.horizontal(|ui| {
                    ui.label("Module Name: ");
                    ui.label(module.get_name().unwrap());
                });
            } else {
                ui.label("Don't have a module!!!");
            }

            ui.separator();
            ui.label("Element Data");
            Self::draw_data(ui, self.editing, &mut self.element_data, "element data");

            ui.separator();
            ui.label("Module Data");
            Self::draw_data(ui, self.editing, &mut self.module_data, "module data");

            ui.separator();
            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.editing, "Editing").clicked() {
                    self.load(element);
                }
                if self.editing {
                    if ui.button("reset").clicked() {
                        self.load(element);
                    }
                    if ui.button("Save").clicked() {
                        self.save(element);
                    }
                }
            });

            if ui
                .button("set module")
                .on_hover_text("set module from, selected module in Modules")
                .clicked()
            {
                if let Some(modules) = storage.get::<Modules>() {
                    element.set_module(modules.selected.clone()).unwrap();
                }
            }
            if ui
                .button("init element")
                .on_hover_text("will use selected module from Modules")
                .clicked()
            {
                let res = element.init().unwrap();
                storage.get_mut::<Console>().unwrap().info(&format!(
                    "Element {} initializated: {}",
                    element.get_name().unwrap(),
                    res
                ));
                self.load(element);
            }

            if ui.button("resolv module").clicked() {
                let res = element.resolv_module().unwrap();
                storage.get_mut::<Console>().unwrap().info(&format!(
                    "Element {} resulved: {}",
                    element.get_name().unwrap(),
                    res
                ));
            }
        }
    }
}

impl ElementTab {
    fn save(&mut self, element: &EInfo) {
        element.set_name(&self.name).unwrap();
        element.set_desc(&self.desc).unwrap();
        element.set_meta(&self.meta).unwrap();
        element.set_element_data(self.element_data.clone()).unwrap();
        element.set_module_data(self.module_data.clone()).unwrap();
        element.set_data(self.data.clone()).unwrap();
    }

    pub fn load(&mut self, element: &EInfo) {
        self.name = element.get_name().unwrap();
        self.desc = element.get_desc().unwrap();
        self.meta = element.get_meta().unwrap();
        self.element_data = element.get_element_data().unwrap();
        self.module_data = element.get_module_data().unwrap();
        self.data = element.get_data().unwrap();
    }
}
