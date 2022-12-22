//! This file contains the front end app built using eframe and egui

use eframe::egui;

use crate::task::List;
use crate::task::Task;

// Const for the default pixels_per_point
const DEFAULT_PIXELS_PER_POINT: f32 = 1.5;

#[derive(Default)]
pub struct RustyTaskboardApp {
    lists: Vec<ListWindow>,
}

struct ListWindow {
    show: bool,
    new_task_description: String,
    list: List,
}

impl ListWindow {
    pub fn new(list: List) -> Self {
        Self {
            show: true,
            new_task_description: String::new(),
            list,
        }
    }
}

impl RustyTaskboardApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setting lightmode
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        // Setting the default pixels_per_point
        cc.egui_ctx.set_pixels_per_point(DEFAULT_PIXELS_PER_POINT);

        let lists: Vec<ListWindow> = vec![
            ListWindow::new(List::new(
                "Main".to_owned(),
                vec![
                    Task::new("A basic task".to_owned()).unwrap(),
                    Task::new("Another basic task".to_owned()).unwrap(),
                    Task::new("A basic task".to_owned()).unwrap(),
                ],
            )),
            ListWindow::new(List::new(
                "Second".to_owned(),
                vec![
                    Task::new("A basic task".to_owned()).unwrap(),
                    Task::new("Another basic task".to_owned()).unwrap(),
                ],
            )),
        ];

        Self { lists }
    }
}

impl eframe::App for RustyTaskboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // The panel to display all the tasklists in
        egui::CentralPanel::default().show(ctx, |_| {
            for list_window in &mut self.lists {
                if !list_window.show {
                    continue
                }

                egui::Window::new(&list_window.list.name).show(ctx, |ui| {
                    // Way of adding more tasks to the list
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut list_window.new_task_description);

                        if ui.button("Add").clicked() {
                            if let Ok(task) = Task::new(list_window.new_task_description.clone()) {
                                list_window.list.tasks.push(task);
                            }
                        }
                    });

                    // Displaying the current tasks
                    for task in &mut list_window.list.tasks {
                        ui.label(task.description());
                    }
                });
            }
        });
    }
}
