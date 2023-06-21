//! This module contains the logic of the topbar
use egui::{containers::Frame, style::Margin, Color32, FontId, RichText, Rounding, Ui};

use rand::Rng;

use crate::app::ListWindow;

const TOPBAR_OUTER_MARGIN: f32 = 5.0;
const TOPBAR_OUTER_MARGIN_SIDE: f32 = 2.5;
const TOPBAR_INNER_MARGIN: f32 = 10.0;
const TOPBAR_ROUNDING: f32 = 5.0;
const TOPBAR_BORDER_WIDTH: f32 = 1.0;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TopBar {
    list_to_add: AddListResult,
    show_settings: bool,
}

impl TopBar {
    /// The entry point for drawing the topbar
    ///
    /// # Arguments
    ///
    /// * `ui` - The UI to draw the topbar on
    pub fn draw(&mut self, ui: &mut Ui) -> Option<ListWindow> {
        ui.horizontal(|ui| {
            draw_logo(ui);
            self.list_to_add = add_list(ui, &self.list_to_add.unwrap());
            self.show_settings = draw_show_settings(ui, self.show_settings);
        });

        match self.list_to_add.clone() {
            AddListResult::AddList(list) => {
                self.list_to_add = AddListResult::ContinueTyping(String::new());
                Some(
                    ListWindow::builder()
                        .name(list)
                        .id(egui::Id::new(rand::thread_rng().gen_range(0..u64::MAX)))
                        .build()
                        .unwrap(),
                )
            }
            AddListResult::ContinueTyping(_) => None,
        }
    }

    pub fn show_settings(&self) -> bool {
        self.show_settings
    }
}

impl Default for TopBar {
    fn default() -> Self {
        TopBar {
            list_to_add: AddListResult::ContinueTyping(String::new()),
            show_settings: false,
        }
    }
}

/// This function draws the "logo"
///
/// # Arguments
///
/// * `ui` - The UI to draw the topbar on
fn draw_logo(ui: &mut Ui) {
    Frame::none()
        .fill(Color32::LIGHT_GRAY)
        .outer_margin(Margin::symmetric(
            TOPBAR_OUTER_MARGIN_SIDE,
            TOPBAR_OUTER_MARGIN,
        ))
        .rounding(Rounding::same(TOPBAR_ROUNDING))
        .show(ui, |ui| {
            Frame::none()
                .outer_margin(Margin::same(TOPBAR_BORDER_WIDTH))
                .inner_margin(Margin::same(TOPBAR_INNER_MARGIN))
                .rounding(Rounding::same(TOPBAR_ROUNDING))
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.label(RichText::new("Rusty Taskboard").font(FontId::proportional(18.0)));
                });
        });
}

/// This function draws the show settings button
///
/// # Arguments
///
/// * `ui` - The UI to draw the topbar on
fn draw_show_settings(ui: &mut Ui, show_settings: bool) -> bool {
    let mut show_settings = show_settings;
    Frame::none()
        .fill(Color32::LIGHT_GRAY)
        .outer_margin(Margin::symmetric(
            TOPBAR_OUTER_MARGIN_SIDE,
            TOPBAR_OUTER_MARGIN,
        ))
        .rounding(Rounding::same(TOPBAR_ROUNDING))
        .show(ui, |ui| {
            Frame::none()
                .outer_margin(Margin::same(TOPBAR_BORDER_WIDTH))
                .inner_margin(Margin::same(TOPBAR_INNER_MARGIN))
                .rounding(Rounding::same(TOPBAR_ROUNDING))
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    if ui.button("Settings").clicked() {
                        show_settings = !show_settings;
                    }
                });
        });

    show_settings
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
/// Enum to model the result of the add_list function
enum AddListResult {
    /// The user has finished typing, add the list
    AddList(String),
    /// The user has not finished typing yet
    ContinueTyping(String),
}

impl AddListResult {
    /// Unwraps the string in the enum
    pub fn unwrap(&self) -> String {
        match self {
            AddListResult::AddList(list) => list,
            AddListResult::ContinueTyping(list) => list,
        }
        .to_string()
    }
}

/// Draws the textbox for adding a new list
///
/// # Arguments
///
/// * `ui`        - The UI to draw the textbox on
/// * `list_name` - The contents of the textbox
///
/// # Return
///
/// The result of the user's interaction with the textbox
fn add_list(ui: &mut Ui, list_name: &str) -> AddListResult {
    let mut list_name = list_name.to_string();
    let mut add_list = false;
    Frame::none()
        .fill(Color32::LIGHT_GRAY)
        .outer_margin(Margin::symmetric(
            TOPBAR_OUTER_MARGIN_SIDE,
            TOPBAR_OUTER_MARGIN,
        ))
        .rounding(Rounding::same(TOPBAR_ROUNDING))
        .show(ui, |ui| {
            Frame::none()
                .outer_margin(Margin::same(TOPBAR_BORDER_WIDTH))
                .inner_margin(Margin::same(TOPBAR_INNER_MARGIN))
                .rounding(Rounding::same(TOPBAR_ROUNDING))
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.label("Add List");
                    Frame::none()
                        .fill(Color32::LIGHT_GRAY)
                        .inner_margin(Margin::same(TOPBAR_BORDER_WIDTH))
                        .rounding(Rounding::same(TOPBAR_ROUNDING - 2.0))
                        .show(ui, |ui| {
                            if ui
                                .text_edit_singleline(&mut list_name)
                                .on_hover_text("Add a new list")
                                .lost_focus()
                            {
                                add_list = true;
                            }
                        });
                });
        });

    match (add_list, list_name.is_empty()) {
        (true, false) => AddListResult::AddList(list_name),
        (_, _) => AddListResult::ContinueTyping(list_name),
    }
}
