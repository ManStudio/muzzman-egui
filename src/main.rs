use std::sync::RwLock;

use actions::ActionsTab;
use console::ConsoleTab;
use control::ControlTab;
use eframe::egui;
use eframe::egui_wgpu::WgpuConfiguration;
use eframe::App;
use eframe::NativeOptions;

use element::ElementTab;
use elements::ElementsTab;
use locations::LocationsTab;
use modules::ModulesTab;
use muzzman_lib_internals::prelude::*;
use storage::Storage;
use tab::TabManager;

mod actions;
mod console;
mod control;
mod edit_type;
mod element;
mod elements;
mod file_or_data;
mod locations;
mod modules;
mod storage;
mod tab;
use edit_type::edit_value;

pub type Session = Option<Box<dyn TSession>>;

#[allow(dead_code)]
struct Context {
    session: Option<Box<dyn TSession>>,
    storage: Arc<RwLock<Storage>>,
    tab_manager: TabManager,
}

impl Default for Context {
    fn default() -> Self {
        let mut storage = Storage::new();
        storage.insert::<Session>(None);

        let storage = Arc::new(RwLock::new(storage));

        let mut tab_manager = TabManager::new(storage.clone());
        tab_manager.register_tab(ConsoleTab::new());
        tab_manager.register_tab(ControlTab::new());
        tab_manager.register_tab(LocationsTab::new());
        tab_manager.register_tab(ElementsTab::new());
        tab_manager.register_tab(ElementTab::new());
        tab_manager.register_tab(ModulesTab::new());
        tab_manager.register_tab(ActionsTab::new());
        tab_manager.open(0);

        Context {
            session: None,
            storage,
            tab_manager,
        }
    }
}

impl App for Context {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Tabs", |ui| {
                    for (i, tab) in self.tab_manager.tabs().iter().enumerate() {
                        let mut open = self.tab_manager.is_open(i);
                        if ui.checkbox(&mut open, tab).changed() {
                            if open {
                                self.tab_manager.open(i);
                            } else {
                                self.tab_manager.close(i);
                            }
                        }
                    }
                })
            });
        });
        self.tab_manager.show(ctx);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> eframe::egui::Vec2 {
        eframe::egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> eframe::egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        eframe::egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}

fn main() {
    eframe::run_native(
        "MuzzMan eframe",
        NativeOptions {
            centered: true,
            event_loop_builder: None,
            mouse_passthrough: false,
            shader_version: None,
            wgpu_options: WgpuConfiguration::default(),
            always_on_top: false,
            maximized: false,
            decorated: true,
            fullscreen: false,
            drag_and_drop_support: false,
            icon_data: None,
            initial_window_pos: None,
            initial_window_size: None,
            min_window_size: None,
            max_window_size: None,
            resizable: true,
            transparent: false,
            vsync: true,
            multisampling: 1,
            depth_buffer: 8,
            stencil_buffer: 8,
            hardware_acceleration: eframe::HardwareAcceleration::Preferred,
            renderer: eframe::Renderer::Wgpu,
            follow_system_theme: true,
            default_theme: eframe::Theme::Dark,
            run_and_return: false,
        },
        Box::new(|_ctx| Box::new(Context::default())),
    )
}
