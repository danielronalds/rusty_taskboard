//! This file contains the front end app built using eframe and egui

use eframe::egui;

use crate::list::{List, ListWindow};
use crate::task::Task;

// Const for the default pixels_per_point
const DEFAULT_PIXELS_PER_POINT: f32 = 1.5;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RustyTaskboardApp {
    lists: Vec<ListWindow>,
    new_tasklist: String,
    show_settings: bool,
    dark_mode: bool,
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
            dark_mode: false,
        }
    }
}

impl eframe::App for RustyTaskboardApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Updating the apps color scheme depending on whether dark_mode is true or not
        match self.dark_mode {
            true => ctx.set_visuals(egui::Visuals::dark()),
            false => ctx.set_visuals(egui::Visuals::light()),
        }

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
                    let mut list_exists = false;

                    // Checking to make sure a list with the same name doesn't already exist
                    for list in &self.lists {
                        if &list.list_name() == &self.new_tasklist {
                            list_exists = true;
                        }
                    }

                    // Creating and adding a new list window with the name in the box
                    if !list_exists {
                        self.lists.push(ListWindow::new(List::new(
                            self.new_tasklist.clone(),
                            Vec::new(),
                        )));

                        // Reseting the textbox
                        self.new_tasklist = String::new();
                    }
                }

                ui.separator();

                // Looping through each list_window
                for list_window in &mut self.lists {
                    let mut new_show = list_window.show();
                    ui.checkbox(&mut new_show, list_window.list_name());
                    list_window.set_show(new_show);
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
                if !list_window.show() {
                    continue;
                }

                // List's window
                egui::Window::new(&list_window.list_name()).show(ctx, |ui| {
                    // Setting the width
                    ui.set_width(200.0);

                    list_window.animate_bar(ctx);

                    ui.add(egui::ProgressBar::new(list_window.progress()).show_percentage());

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label("Add ");
                        // Way of adding more tasks to the list
                        if ui
                            .text_edit_singleline(list_window.mut_new_task_description())
                            .on_hover_text("Add a new task")
                            .lost_focus()
                        {
                            if let Ok(task) = Task::new(list_window.new_task_description()) {
                                list_window.add_task(task);

                                // Resetting the textbox
                                list_window.reset_new_task_description();
                            }
                        }
                    });

                    ui.add_space(10.0);

                    let mut task_to_be_deleted = None;

                    // Displaying the current tasks
                    //
                    // The for loop with a range is used so that an immutable borrow can be used in
                    // the closure along side the mutable borrow used to change the status of a
                    // task
                    for i in 0..list_window.task_vec().len() {
                        ui.horizontal_wrapped(|ui| {
                            // Allows the user to delete a task, only if the mode is enabled
                            if list_window.delete_mode() && ui.button("X").clicked() {
                                task_to_be_deleted = Some(list_window.task_vec()[i].clone());
                            }

                            // Displaying the textbox for editing a task's description
                            if list_window.update_tasks() {
                                ui.text_edit_singleline(
                                    list_window.mut_task_vec()[i].mut_new_description(),
                                );
                                return;
                            }

                            // If update_task is false, then the user must be finished editing
                            // tasks. If new_description is empty, set it to the current
                            // description, else set it as the new description
                            if list_window.task_vec()[i].new_description().is_empty() {
                                let new_description = list_window.task_vec()[i].description();
                                list_window.mut_task_vec()[i].set_new_description(new_description);
                            } else {
                                let new_description = list_window.task_vec()[i].new_description();
                                list_window.mut_task_vec()[i]
                                    .update_description(new_description)
                                    .unwrap_or(());
                            }

                            // Little work around the borrow checker
                            let mut value = list_window.task_vec()[i].completed();
                            ui.checkbox(&mut value, list_window.task_vec()[i].description());
                            list_window.mut_task_vec()[i].set_completed(value);
                        });
                    }

                    // Currently the problem with this is that it doesnt only delete the one task
                    if let Some(task_to_delete) = task_to_be_deleted {
                        list_window.delete_task(task_to_delete);
                    }

                    ui.add_space(10.0);

                    ui.collapsing("Options", |ui| {
                        ui.horizontal(|ui| {
                            // Code to display the list_window name in the text edit box
                            if list_window.new_list_name().is_empty() {
                                list_window.set_new_list_name(list_window.list_name());
                            }

                            ui.label("Name");
                            if ui
                                .text_edit_singleline(list_window.mut_new_list_name())
                                .lost_focus()
                                && !list_window.new_list_name().is_empty()
                            {
                                list_window.set_list_name(list_window.new_list_name());
                            }
                        });

                        ui.add_space(10.0);
                        ui.checkbox(list_window.mut_delete_mode(), "Remove tasks");

                        ui.add_space(10.0);
                        ui.checkbox(list_window.mut_update_tasks(), "Enable task editing");

                        ui.add_space(10.0);
                        if ui.button("Delete completed tasks").clicked() {
                            list_window.delete_completed_tasks();
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
                    list.task_vec_clone() != list_to_delete.task_vec_clone()
                        && list.list_name() != list_to_delete.list_name()
                });
            }

            // Showing the settings window if it should be open
            if self.show_settings {
                egui::Window::new("Settings").show(ctx, |ui| {
                    ui.checkbox(&mut self.dark_mode, "Dark mode");

                    ui.add_space(10.0);
                    ui.label("Zoom Factor");
                    let mut slider_value = ctx.pixels_per_point();
                    ui.add(egui::Slider::new(&mut slider_value, 1.0..=2.0));
                    ctx.set_pixels_per_point(slider_value);

                    ui.add_space(10.0);
                    ui.label(format!("Rusty Taskboards v{}", env!("CARGO_PKG_VERSION")))
                });
            }
        });
    }
}
