//! This module contains the logic for the list windows

use egui::{Context, Ui};

use crate::task::{List, Task};

const SPACE_BETWEEN_TASKS: f32 = 10.0;

pub fn draw_list(ctx: &Context, list: List) -> List {
    let mut list = list;
    egui::Window::new("Tasklist")
        .resizable(false)
        .show(ctx, |ui| {
            list = list.clone().into_iter() // The clone is needed here due to the closure
                .map(|task| {
                    ui.add_space(SPACE_BETWEEN_TASKS);
                    draw_task(ui, task)
                })
                .collect();
        });
    list
}

fn draw_task(ui: &mut Ui, task: Task) -> Task {
    let mut task = task;

    ui.separator();
    let title = task.title();
    ui.checkbox(task.mut_completed(), title);
    ui.label("This is the begining of a task");
    ui.separator();

    task
}
