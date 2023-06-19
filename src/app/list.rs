//! This module contains the logic for the list windows

use egui::{containers::Frame, style::Margin, Color32, Context, Rounding, Ui};

use crate::task::{List, Task};

const WINDOW_WIDTH: f32 = 250.0;

/// Draws the list to a window
///
/// # Arguments
///
/// * `ctx`  - The egui handle
/// * `list` - The list to draw
///
/// # Returns
///
/// The list with any modifications that has happened
pub fn draw_list(ctx: &Context, list: List) -> List {
    let mut list = list;
    egui::Window::new("Tasklist")
        .resizable(false)
        .show(ctx, |ui| {
            draw_progress_bar(ui, list.progress());

            list = list
                .clone() // The clone is needed here due to the closure
                .into_iter()
                .map(|task| draw_task(ui, task))
                .collect();
        });
    list
}

/// Draws the progress bar
///
/// # Arguments
///
/// * `ui`         - The UI to draw the progress bar onto
/// * `progress`   - The progress of the list
fn draw_progress_bar(ui: &mut Ui, progress: f32) {
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
                    ui.add(egui::ProgressBar::new(progress).show_percentage());
                });
        });
}

/// The outer margin of the task widget
const TASK_OUTER_MARGIN: f32 = 5.0;
/// The inner margin of the task widget
const TASK_INNER_MARGIN: f32 = 10.0;
/// How rounded the corners of the task widget should be
const TASK_ROUNDING: f32 = 5.0;
/// The border width of the task widget
const TASK_BORDER_WIDTH: f32 = 1.0;

/// Draws the task UI widget
///
/// # Arguments
///
/// * `ui`   - The UI to draw the task UI widget onto
/// * `task` - The task for the widget to display
///
/// # Returns
/// 
/// The result of the user interacting with the widget, aka whether the task has been completed
fn draw_task(ui: &mut Ui, task: Task) -> Task {
    let mut task = task;

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
                    let title = task.title();
                    ui.checkbox(task.mut_completed(), title);
                    let description = task.description();
                    if !description.is_empty() {
                        ui.label(description);
                    }
                });
        });

    task
}
