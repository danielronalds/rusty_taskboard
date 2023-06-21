//! This module contains the logic for the settings window
use egui::{containers::Frame, style::Margin, Color32, Context, Rounding, Ui};

use crate::app::list::ListWindow;

const WINDOW_WIDTH: f32 = 250.0;

pub fn draw_settings(ctx: &Context, lists: &[ListWindow]) -> Vec<ListWindow> {
    let mut lists = lists.to_vec();
    egui::Window::new("Settings")
        .resizable(false)
        .show(ctx, |ui| {
            lists = draw_lists(ui, lists.clone());
        });
    lists
}

/// The outer margin of the task widget
const TASK_OUTER_MARGIN: f32 = 5.0;
/// The inner margin of the task widget
const TASK_INNER_MARGIN: f32 = 10.0;
/// How rounded the corners of the task widget should be
const TASK_ROUNDING: f32 = 5.0;
/// The border width of the task widget
const TASK_BORDER_WIDTH: f32 = 1.0;

fn draw_lists(ui: &mut Ui, lists: Vec<ListWindow>) -> Vec<ListWindow> {
    let mut lists = lists;
    Frame::none()
        .fill(Color32::LIGHT_GRAY)
        .outer_margin(Margin::same(TASK_OUTER_MARGIN))
        .rounding(Rounding::same(TASK_ROUNDING))
        .show(ui, |ui| {
            Frame::none()
                .outer_margin(Margin::same(TASK_BORDER_WIDTH))
                .inner_margin(Margin::same(TASK_INNER_MARGIN))
                .rounding(Rounding::same(TASK_ROUNDING))
                .fill(Color32::WHITE)
                .show(ui, |ui| {
                    ui.set_width(WINDOW_WIDTH);
                    ui.label("Lists");
                    lists = lists.iter().filter_map(|list| draw_list(ui, list.clone())).collect();
                });
        });
    lists
}

fn draw_list(ui: &mut Ui, list: ListWindow) -> Option<ListWindow> {
    let mut delete_list = false;
    let mut list = list;

    ui.horizontal(|ui| {
        // Having a border frame here so that the button lines up with the text
        // edit field
        Frame::none()
            .fill(Color32::LIGHT_GRAY)
            .outer_margin(Margin::symmetric(0.0, TASK_OUTER_MARGIN))
            .inner_margin(Margin::same(TASK_BORDER_WIDTH))
            .rounding(Rounding::same(TASK_ROUNDING - 2.0))
            .show(ui, |ui| {
                if ui.button("X").clicked() {
                    delete_list = true;
                }
            });
        list.set_name(textfield(ui, list.name()));
    });
    
    match delete_list {
        false => Some(list),
        true => None,
    }
}

fn textfield(ui: &mut Ui, contents: String) -> String {
    let mut contents = contents;
    Frame::none()
        .fill(Color32::LIGHT_GRAY)
        .outer_margin(Margin::symmetric(0.0, TASK_OUTER_MARGIN))
        .inner_margin(Margin::same(TASK_BORDER_WIDTH))
        .rounding(Rounding::same(TASK_ROUNDING - 2.0))
        .show(ui, |ui| ui.text_edit_singleline(&mut contents));
    contents
}
