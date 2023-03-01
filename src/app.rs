//! This file contains the front end app built using eframe and egui

use eframe::egui;

use crate::list::{List, ListWindow, DEFAULT_LIST_WINDOW_WIDTH};

// Const for the default pixels_per_point
const DEFAULT_PIXELS_PER_POINT: f32 = 1.5;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RustyTaskboardApp {
    lists: Vec<ListWindow>,
    list_window_width: f32,
    new_tasklist: String,
    show_settings: bool,
    show_progress_bars: bool,
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
        let lists: Vec<ListWindow> = Vec::new();

        Self {
            lists,
            list_window_width: DEFAULT_LIST_WINDOW_WIDTH,
            new_tasklist: String::new(),
            show_settings: false,
            show_progress_bars: true,
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
                        if list.list_name() == self.new_tasklist {
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

                // Displaying the list window
                list_to_delete =
                    list_window.display(ctx, self.list_window_width, self.show_progress_bars);
            }

            // Deleting the list if it
            if let Some(list_to_delete) = list_to_delete {
                self.lists.retain(|list| {
                    list.task_vec_clone() != list_to_delete.task_vec_clone()
                        && list.list_name() != list_to_delete.list_name()
                        && list.id() != list_to_delete.id()
                });
            }

            // Showing the settings window if it should be open
            if self.show_settings {
                egui::Window::new("Settings")
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.checkbox(&mut self.dark_mode, "Dark mode");

                        ui.add_space(10.0);
                        ui.checkbox(&mut self.show_progress_bars, "Show progress bars");

                        ui.add_space(10.0);
                        ui.label("Zoom Factor");
                        let mut slider_value = ctx.pixels_per_point();
                        ui.add(egui::Slider::new(&mut slider_value, 1.0..=2.0));
                        ctx.set_pixels_per_point(slider_value);

                        ui.add_space(10.0);
                        ui.label("Window Width");
                        ui.add(egui::Slider::new(
                            &mut self.list_window_width,
                            100.0..=500.0,
                        ));

                        ui.add_space(10.0);
                        ui.label(format!("Rusty Taskboards v{}", env!("CARGO_PKG_VERSION")))
                    });
            }
        });
    }
}
