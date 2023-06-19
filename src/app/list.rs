//! This module contains the logic for the list windows

use egui::{containers::Frame, style::Margin, Color32, Context, Rounding, Ui};

use crate::task::{List, Task};

const SPACE_BETWEEN_TASKS: f32 = 10.0;

pub fn draw_list(ctx: &Context, list: List) -> List {
    let mut list = list;
    egui::Window::new("Tasklist")
        .resizable(false)
        .show(ctx, |ui| {
            list = list
                .clone() // The clone is needed here due to the closure
                .into_iter()
                .map(|task| draw_task(ui, task))
                .collect();
        });
    list
}

const TASK_OUTER_MARGIN: f32 = 5.0;
const TASK_INNER_MARGIN: f32 = 10.0;
const TASK_ROUNDING: f32 = 5.0;
const TASK_BORDER_WIDTH: f32 = 1.0;
const TASK_WIDTH: f32 = 200.0;

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
                    ui.set_width(TASK_WIDTH);
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
