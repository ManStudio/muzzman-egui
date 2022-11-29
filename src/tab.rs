use std::sync::{Arc, RwLock};

use eframe::egui;

use crate::storage::Storage;

pub trait Tab {
    fn get_name(&self) -> &str;

    fn init(&mut self, storage: &mut Storage);
    fn draw(&mut self, ui: &mut egui::Ui, storage: &mut Storage);
}

pub struct TabManager {
    tabs: Vec<Arc<RwLock<Box<dyn Tab>>>>,
    tree: egui_dock::Tree<Arc<RwLock<Box<dyn Tab>>>>,
    storage: Arc<RwLock<Storage>>,
    tab_viewer: TabViewer,
}

impl TabManager {
    pub fn new(storage: Arc<RwLock<Storage>>) -> Self {
        Self {
            tabs: Vec::new(),
            tree: egui_dock::Tree::new(Vec::new()),
            tab_viewer: TabViewer {
                storage: storage.clone(),
            },
            storage,
        }
    }

    pub fn register_tab<T: 'static + Tab>(&mut self, tab: T) -> usize {
        let mut tab = Box::new(tab);
        tab.init(&mut self.storage.write().unwrap());
        let id = self.tabs.len();
        self.tabs.push(Arc::new(RwLock::new(tab)));
        id
    }

    pub fn tabs(&self) -> Vec<String> {
        let mut res = Vec::new();

        for tab in self.tabs.iter() {
            res.push(tab.read().unwrap().get_name().to_string());
        }

        res
    }

    #[allow(dead_code)]
    pub fn get_tab_name(&self, id: usize) -> Option<String> {
        if let Some(tab) = self.tabs.get(id) {
            Some(tab.read().unwrap().get_name().to_string())
        } else {
            None
        }
    }

    pub fn is_open(&self, id: usize) -> bool {
        let mut res = false;
        let registered_tab = &self.tabs[id];
        'm: for node in self.tree.iter() {
            match node {
                egui_dock::Node::Leaf { tabs, .. } => {
                    for tab in tabs.iter() {
                        if tab.read().unwrap().get_name()
                            == registered_tab.read().unwrap().get_name()
                        {
                            res = true;
                            break 'm;
                        }
                    }
                }
                _ => {}
            }
        }
        res
    }

    pub fn open(&mut self, id: usize) {
        if !self.is_open(id) {
            let tab = self.tabs[id].clone();
            self.tree.push_to_focused_leaf(tab);
        }
    }

    pub fn close(&mut self, id: usize) {
        if self.is_open(id) {
            let registered_tab = &self.tabs[id];

            for node in self.tree.iter_mut() {
                match node {
                    egui_dock::Node::Leaf { tabs, .. } => {
                        let mut index = 0;
                        let mut finded = false;
                        for (i, tab) in tabs.iter().enumerate() {
                            if tab.read().unwrap().get_name()
                                == registered_tab.read().unwrap().get_name()
                            {
                                finded = true;
                                index = i;
                                break;
                            }
                        }
                        if finded {
                            tabs.remove(index);
                        }
                    }
                    _ => {}
                }
            }
            self.tree.remove_empty_leaf();
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui_dock::DockArea::new(&mut self.tree).show(ctx, &mut self.tab_viewer);
    }
}

pub struct TabViewer {
    storage: Arc<RwLock<Storage>>,
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = Arc<RwLock<Box<dyn Tab>>>;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.write()
            .unwrap()
            .draw(ui, &mut self.storage.write().unwrap())
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.read().unwrap().get_name().into()
    }
}
