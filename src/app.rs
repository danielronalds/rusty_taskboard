//! This file contains the front end app built using eframe and egui
use eframe::egui;

mod list;
use list::ListWindow;

use crate::task::{List, Task};

/// Constant for the default pixels_per_point
const DEFAULT_PIXELS_PER_POINT: f32 = 1.5;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RustyTaskboardApp {
    list_window: ListWindow,
}

impl RustyTaskboardApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setting lightmode
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        // Setting the default pixels_per_point
        cc.egui_ctx.set_pixels_per_point(DEFAULT_PIXELS_PER_POINT);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl Default for RustyTaskboardApp {
    fn default() -> Self {
        let tasks = vec![
            Task::builder()
                .title("Task One".to_string())
                .description("This is the first task".to_string())
                .build()
                .unwrap(),
            Task::builder()
                .title("Task Two".to_string())
                .description("This is the second task, and still with a description".to_string())
                .build()
                .unwrap(),
            Task::builder()
                .title("Task Three".to_owned())
                .completed(true)
                .build()
                .unwrap(),
        ];
        let mut list = List::new();
        for task in tasks {
            list.add(task);
        }
        let list_window = ListWindow::builder()
            .name("Tasklist".to_string())
            .list(list)
            .build()
            .unwrap();
        Self { list_window }
    }
}

impl eframe::App for RustyTaskboardApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.list_window = list::draw_list_window(&ctx, self.list_window.clone());
        });
    }
}
