use std::path::PathBuf;

use eframe::egui::{self, Ui};
use muzzman_lib::prelude::*;

pub fn edit_file_or_data(ui: &mut Ui, ford: &mut FileOrData) {
    let mut current = match ford {
        FileOrData::File(_, _) => 0,
        FileOrData::Bytes(_) => 1,
    };

    let last = current;

    let current_str = match current {
        0 => "File",
        1 => "Data",
        _ => "",
    };

    egui::ComboBox::new("file_or_data", "File Or Data")
        .selected_text(current_str)
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut current, 0, "File");
            ui.selectable_value(&mut current, 1, "Data");
        });

    if last != current {
        *ford = match current {
            0 => FileOrData::File(PathBuf::new(), None),
            1 => FileOrData::Bytes(Bytes::new()),
            _ => panic!("Is inposibile"),
        }
    }

    match ford {
        FileOrData::File(file_path, f) => {
            let last = file_path.clone();
            let mut str = file_path.to_str().unwrap().to_owned();
            ui.text_edit_singleline(&mut str);
            *file_path = PathBuf::from(str);
            if last != *file_path {
                *f = None;
            }
        }
        FileOrData::Bytes(bytes) => {
            egui::CollapsingHeader::new(ui.id().short_debug_format()).show(ui, |ui| {
                let mut string_bytes = String::with_capacity(bytes.data.len());
                for (i, byte) in bytes.data.iter().enumerate() {
                    if i > 0 {
                        string_bytes.push(',');
                    }
                    string_bytes.push_str(&byte.to_string());
                }
                ui.text_edit_singleline(&mut string_bytes);

                let mut data = Vec::with_capacity(bytes.data.len());

                let splited = string_bytes.split(',').collect::<Vec<&str>>();

                for seg in splited {
                    if let Ok(b) = seg.parse() {
                        data.push(b)
                    }
                }

                bytes.data = data;
            });
        }
    }
}
