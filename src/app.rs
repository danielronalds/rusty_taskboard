//! This file contains the front end app built using eframe and egui

use eframe::egui;

use crate::task::List;
use crate::task::Task;

// Const for the default pixels_per_point
const DEFAULT_PIXELS_PER_POINT: f32 = 1.5;

#[derive(Default)]
pub struct RustyTaskboardApp {
    lists: Vec<List>,
}

impl RustyTaskboardApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setting lightmode
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        // Setting the default pixels_per_point
        cc.egui_ctx.set_pixels_per_point(DEFAULT_PIXELS_PER_POINT);

        let lists: Vec<List> = vec![
            List::new(
                "Main".to_owned(),
                vec![
                    Task::new("A basic task".to_owned()).unwrap(),
                    Task::new("Another basic task".to_owned()).unwrap(),
                    Task::new("A basic task".to_owned()).unwrap(),
                ],
            ),
            List::new(
                "Second".to_owned(),
                vec![
                    Task::new("A basic task".to_owned()).unwrap(),
                    Task::new("Another basic task".to_owned()).unwrap(),
                ],
            ),
        ];

        Self { lists }
    }
}

impl eframe::App for RustyTaskboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // The panel to display all the tasklists in
        egui::CentralPanel::default().show(ctx, |_| {
            for list in &mut self.lists {
                egui::Window::new(&list.name).show(ctx, |ui| {
                    for task in &mut list.tasks {
                        ui.label(task.description());
                    }
                });
            }
        });
    }
}
