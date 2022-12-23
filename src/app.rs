//! This file contains the front end app built using eframe and egui

use eframe::egui;

use crate::task::List;
use crate::task::Task;

// Const for the default pixels_per_point
const DEFAULT_PIXELS_PER_POINT: f32 = 1.5;

#[derive(Default, serde::Deserialize, serde::Serialize)]
struct ListWindow {
    show: bool,
    new_task_description: String,
    new_list_name: String,
    list: List,
    /// The progress shown
    progress: f32,
}

impl ListWindow {
    pub fn new(list: List) -> Self {
        Self {
            show: true,
            new_task_description: String::new(),
            new_list_name: String::new(),
            list,
            progress: 0.0,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RustyTaskboardApp {
    lists: Vec<ListWindow>,
    new_tasklist: String,
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

        Self {
            lists,
            new_tasklist: String::new(),
        }
    }
}

impl eframe::App for RustyTaskboardApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Side panel for displaying the name of the app and what windows to show
        egui::SidePanel::left("Sidebar")
            .exact_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Rusty Taskboard");

                // Code for adding a new tasklist
                ui.horizontal(|ui| {
                    if ui
                        .text_edit_singleline(&mut self.new_tasklist)
                        .on_hover_text("Add a new list")
                        .lost_focus()
                    {
                        // Creating and adding a new list window with the name in the box
                        self.lists.push(ListWindow::new(List::new(
                            self.new_tasklist.clone(),
                            Vec::new(),
                        )));

                        // Reseting the textbox
                        self.new_tasklist = String::new();
                    }
                });

                // Looping through each list_window
                for list_window in &mut self.lists {
                    ui.checkbox(&mut list_window.show, list_window.list.name.clone());
                }
            });

        // The panel to display all the tasklists in
        egui::CentralPanel::default().show(ctx, |_| {
            for list_window in &mut self.lists {
                if !list_window.show {
                    continue;
                }

                // List's window
                egui::Window::new(&list_window.list.name).resizable(false).show(ctx, |ui| {
                    // Setting the width
                    ui.set_width(200.0);

                    // Progress bar to show how much of the list is done
                    // Animating the progress bar
                    if list_window.list.progress() < list_window.progress {
                        list_window.progress -= 0.01;
                        // Requesting a repaint so that the animation is smooth
                        ctx.request_repaint();
                    } else if list_window.list.progress() > list_window.progress {
                        list_window.progress += 0.01;
                        // Requesting a repaint so that the animation is smooth
                        ctx.request_repaint();
                    }
                    ui.add(egui::ProgressBar::new(list_window.progress).show_percentage());

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label("Add ");
                        // Way of adding more tasks to the list
                        if ui
                            .text_edit_singleline(&mut list_window.new_task_description)
                            .on_hover_text("Add a new task")
                            .lost_focus()
                        {
                            if let Ok(task) = Task::new(list_window.new_task_description.clone()) {
                                list_window.list.tasks.push(task);

                                // Resetting the textbox
                                list_window.new_task_description = String::new();
                            }
                        }
                    });

                    ui.add_space(10.0);

                    // Displaying the current tasks
                    for task in &mut list_window.list.tasks {
                        ui.horizontal(|ui| {
                            // Little work around the borrow checker
                            let mut value = task.completed();
                            ui.checkbox(&mut value, task.description());
                            task.set_completed(value);
                        });
                    }

                    ui.add_space(10.0);

                    ui.collapsing("Options", |ui| {
                        ui.horizontal(|ui| {
                            // Code to display the list_window name in the text edit box
                            if list_window.new_list_name.is_empty() {
                                list_window.new_list_name = list_window.list.name.clone();
                            }

                            ui.label("Name");
                            if ui
                                .text_edit_singleline(&mut list_window.new_list_name)
                                .lost_focus() && !list_window.new_list_name.is_empty()
                            {
                                list_window.list.name = list_window.new_list_name.clone();
                            }
                        });

                        if ui.button("Delete completed tasks").clicked() {
                            list_window.list.tasks.retain(|task| !task.completed());
                        }
                    });
                });
            }
        });
    }
}
