use std::{collections::HashMap, path::PathBuf};

use eframe::egui::{self, Id, Ui};
use muzzman_lib::prelude::*;

use crate::file_or_data;

macro_rules! selectabile {
    ($ui: ident; $value: expr; $($next: expr),+) => {
        $(selectabile!($ui; $value => $next);)+
    };
    ($ui: ident; $value: expr => $next: expr) => {
        $ui.selectable_value($value, $next, $next.to_string());
    };
}

pub fn edit_value(ui: &mut Ui, value: &mut Value, id: Id) {
    let mut current = value.value.to_tag();

    let last = current.clone();

    ui.push_id(id, |ui| {
        egui::ComboBox::new("select_type", "")
            .selected_text(current.to_string())
            .show_ui(ui, |ui| {
                if false{ // show all
                    selectabile!{ui; &mut current;
                    TypeTag::U8, TypeTag::U16, TypeTag::U32, TypeTag::U64, TypeTag::U128, TypeTag::USize,
                    TypeTag::I8, TypeTag::I16, TypeTag::I32, TypeTag::I64, TypeTag::I128, TypeTag::ISize,
                    TypeTag::F32, TypeTag::F64,
                    TypeTag::Bool,
                    TypeTag::String, TypeTag::Path, TypeTag::HashMapSS, TypeTag::FileOrData, TypeTag::Bytes, TypeTag::None
                };
                }else{
                    for what in value.should_be.iter(){
                        ui.selectable_value(&mut current, what.clone(), what.to_string()) ;
                    }
                }
            });
    });
    if last != current {
        value.value = match current {
            TypeTag::U8 => Type::U8(0),
            TypeTag::U16 => Type::U16(0),
            TypeTag::U32 => Type::U32(0),
            TypeTag::U64 => Type::U64(0),
            TypeTag::U128 => Type::U128(0),
            TypeTag::USize => Type::USize(0),
            TypeTag::I8 => Type::I8(0),
            TypeTag::I16 => Type::I16(0),
            TypeTag::I32 => Type::I32(0),
            TypeTag::I64 => Type::I64(0),
            TypeTag::I128 => Type::I128(0),
            TypeTag::ISize => Type::ISize(0),
            TypeTag::F32 => Type::F32(0.0),
            TypeTag::F64 => Type::F64(0.0),
            TypeTag::Bool => Type::Bool(false),
            TypeTag::String => Type::String(String::new()),
            TypeTag::Url => Type::String(String::new()),
            TypeTag::Path => Type::Path(PathBuf::new()),
            TypeTag::HashMapSS => Type::HashMapSS(HashMap::new()),
            TypeTag::HashMapS(_) => todo!(),
            TypeTag::FileOrData => Type::FileOrData(FileOrData::Bytes(Bytes::default())),
            TypeTag::Any => todo!(),
            TypeTag::CustomEnum(_) => value.default.clone(),
            TypeTag::AdvancedEnum(_) => value.default.clone(),
            TypeTag::Vec(_) => value.default.clone(),
            TypeTag::Bytes => Type::Bytes(Vec::new()),
            TypeTag::None => Type::None,
        }
    }

    edit_type(ui, &mut value.value, id);
}

pub fn edit_type(ui: &mut Ui, _type: &mut Type, id: Id) {
    match _type {
        Type::U8(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::U16(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::U32(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::U64(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::U128(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::USize(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::I8(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::I16(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::I32(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::I64(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::I128(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::ISize(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::F32(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::F64(value) => {
            let mut data = value.to_string();
            ui.text_edit_singleline(&mut data);
            if let Ok(data) = data.parse() {
                *value = data;
            }
        }
        Type::Bool(value) => {
            ui.checkbox(value, "");
        }
        Type::String(value) => {
            ui.text_edit_singleline(value);
        }
        Type::Path(_) => {}
        Type::HashMapSS(h) => {
            egui::CollapsingHeader::new(format!("hashmapss{}", id.short_debug_format())).show(
                ui,
                |ui| {
                    ui.vertical(|ui| {
                        for (key, value) in h.iter_mut() {
                            ui.horizontal(|ui| {
                                ui.label(key);
                                ui.label(":");
                                ui.text_edit_singleline(value);
                            });
                        }
                    });
                },
            );
        }
        Type::HashMapS(_) => {}
        Type::FileOrData(ford) => {
            file_or_data::edit_file_or_data(ui, ford);
        }
        Type::Any(_) => {}
        Type::CustomEnum(cenum) => {
            egui::ComboBox::new(format!("custom_enum{}", id.short_debug_format()), "")
                .selected_text(cenum.get_active().unwrap())
                .show_ui(ui, |ui| {
                    let mut selected = cenum.active.unwrap();
                    for (i, field) in cenum.get_fields().iter().enumerate() {
                        ui.selectable_value(&mut selected, i, field);
                    }
                    cenum.set_active(Some(selected));
                });
        }
        Type::AdvancedEnum(_) => {}
        Type::Vec(vec) => {
            egui::CollapsingHeader::new(format!("vec{}", id.short_debug_format())).show(ui, |ui| {
                ui.vertical(|ui| {
                    for element in vec.iter_mut() {
                        edit_type(ui, element, id);
                    }
                });

                if ui.button("clone").clicked() {
                    if let Some(last) = vec.last() {
                        vec.push(last.clone())
                    }
                }
            });
        }
        Type::Bytes(_) => {}
        Type::None => {}
    }
}
