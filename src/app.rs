//! This file contains the front end app built using eframe and egui

use eframe::egui;

use crate::task::List;
use crate::task::Task;

// Const for the default pixels_per_point
const DEFAULT_PIXELS_PER_POINT: f32 = 1.5;

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
struct ListWindow {
    show: bool,
    new_task_description: String,
    new_list_name: String,
    delete_mode: bool,
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
            delete_mode: false,
            list,
            progress: 0.0,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct RustyTaskboardApp {
    lists: Vec<ListWindow>,
    new_tasklist: String,
    show_settings: bool,
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
            show_settings: false,
        }
    }
}

impl eframe::App for RustyTaskboardApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel for displaying the name of the app and what windows to show
        egui::TopBottomPanel::top("Sidebar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Setting height
                ui.set_height(40.0);

                ui.heading("Rusty Taskboard");

                ui.separator();

                ui.label("Add List ");

                // Code for adding a new tasklist
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

                ui.separator();

                // Looping through each list_window
                for list_window in &mut self.lists {
                    ui.checkbox(&mut list_window.show, list_window.list.name.clone());
                }

                // Checkbox for showing the setting window
                if ui.button("Settings").clicked() {
                    self.show_settings = !self.show_settings;
                }
            });
        });

        // The panel to display all the tasklists in
        egui::CentralPanel::default().show(ctx, |_ui| {
            let mut list_to_delete = None;

            for list_window in &mut self.lists {
                if !list_window.show {
                    continue;
                }

                // List's window
                egui::Window::new(&list_window.list.name).show(ctx, |ui| {
                    // Setting the width
                    ui.set_width(200.0);

                    // Progress bar to show how much of the list is done

                    // If the progress is within a certain range, just set it to exactly the
                    // progress
                    if (list_window.progress - list_window.list.progress()) < 0.01
                        && (list_window.progress - list_window.list.progress()) > -0.01
                        && !(list_window.progress == list_window.list.progress())
                    {
                        list_window.progress = list_window.list.progress();
                    } else if list_window.list.progress() < list_window.progress {
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

                    let mut task_to_be_deleted = None;

                    // Displaying the current tasks
                    for task in &mut list_window.list.tasks {
                        ui.horizontal(|ui| {
                            // Allows the user to delete a task, only if the mode is enabled
                            if list_window.delete_mode && ui.button("X").clicked() {
                                task_to_be_deleted = Some(task.clone());
                            }

                            // Little work around the borrow checker
                            let mut value = task.completed();
                            ui.checkbox(&mut value, task.description());
                            task.set_completed(value);
                        });
                    }

                    // Currently the problem with this is that it doesnt only delete the one task
                    if let Some(task_to_delete) = task_to_be_deleted {
                        list_window
                            .list
                            .tasks
                            .retain(|task| task != &task_to_delete);
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
                                .lost_focus()
                                && !list_window.new_list_name.is_empty()
                            {
                                list_window.list.name = list_window.new_list_name.clone();
                            }
                        });

                        ui.add_space(10.0);
                        ui.checkbox(&mut list_window.delete_mode, "Remove tasks");

                        ui.add_space(10.0);
                        if ui.button("Delete completed tasks").clicked() {
                            list_window.list.tasks.retain(|task| !task.completed());
                        }

                        ui.add_space(10.0);
                        if ui.button("Delete list").clicked() {
                            list_to_delete = Some(list_window.clone());
                        }
                    });
                });
            }

            // Deleting the list if it
            if let Some(list_to_delete) = list_to_delete {
                self.lists.retain(|list| {
                    list.list != list_to_delete.list && list.list.name != list_to_delete.list.name
                });
            }

            // Showing the settings window if it should be open
            if self.show_settings {
                egui::Window::new("Settings").show(ctx, |ui| {
                    ui.label("Zoom Factor");
                    let mut slider_value = ctx.pixels_per_point();
                    ui.add(egui::Slider::new(&mut slider_value, 1.0..=2.0));
                    ctx.set_pixels_per_point(slider_value);
                });
            }
        });
    }
}
