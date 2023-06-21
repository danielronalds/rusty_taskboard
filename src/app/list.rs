//! This module contains the logic for the list windows

use egui::{containers::Frame, style::Margin, Color32, Context, Rounding, Ui};

use crate::task::{List, Task};

const WINDOW_WIDTH: f32 = 250.0;

#[derive(Builder, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListWindow {
    /// The name of the list
    name: String,
    /// The id of the window
    id: egui::Id,
    #[builder(default = "List::new()")]
    /// The list of tasks to display in the list window
    list: List,
    #[builder(default = "String::new()")]
    /// Variable to store the contents of the add task box
    task_to_add: String,
    #[builder(default = "false")]
    /// Whether the window is being edited or not
    editing: bool,
    #[builder(default = "true")]
    /// Whether to show the window or not
    visible: bool,
}

impl ListWindow {
    /// Returns the ListWindowBuilder
    pub fn builder() -> ListWindowBuilder {
        ListWindowBuilder::default()
    }

    /// Gets a clone of the name of the ListWindow
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Sets the name of the ListWindow
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns a mutable reference to the visible field
    pub fn mut_visible(&mut self) -> &mut bool {
        &mut self.visible
    }
}

/// Draws the list to a window
///
/// # Arguments
///
/// * `ctx`  - The egui handle
/// * `list` - The list window to draw
///
/// # Returns
///
/// The list with any modifications that has happened, or None if the list has been deleted
pub fn draw_list_window(ctx: &Context, list: ListWindow) -> Option<ListWindow> {
    if !list.visible {
        return Some(list);
    }

    let mut list_window = list;
    let mut delete_list = false;
    egui::Window::new(&list_window.name)
        .resizable(false)
        .id(list_window.id)
        .show(ctx, |ui| {
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
                            draw_progress_bar(ui, list_window.list.progress());
                            match add_task(ui, &list_window.task_to_add) {
                                AddTaskResult::ContinueTyping(task_to_add) => {
                                    list_window.task_to_add = task_to_add
                                }
                                AddTaskResult::AddTask(task_to_add) => {
                                    if let Ok(task) = Task::builder().title(task_to_add).build() {
                                        list_window.list.add(task);
                                        list_window.task_to_add = String::new();
                                    }
                                }
                            }
                            ui.horizontal(|ui| {
                                list_window.editing = draw_edit_button(ui, list_window.editing);
                                list_window.list = draw_sort_button(ui, list_window.list.clone());
                                list_window.list = draw_delete_completed_tasks_button(
                                    ui,
                                    list_window.list.clone(),
                                );
                                delete_list = draw_delete_list(ui);
                            })
                        });
                });

            list_window.list = list_window
                .list
                .clone() // The clone is needed here due to the closure
                .into_iter()
                .filter_map(|task| draw_task(ui, list_window.editing, task))
                .collect();
        });

    match delete_list {
        false => Some(list_window),
        true => None,
    }
}

/// Function to draw the edit button
///
/// # Arguments
///
/// * `ui`      - The UI to draw the button onto
/// * `editing` - The current value of editing
///
/// # Returns
///
/// The new value of editing
fn draw_edit_button(ui: &mut Ui, editing: bool) -> bool {
    if ui.button("Edit").clicked() {
        return !editing;
    }
    editing
}

/// Function to draw the sort button
///
/// # Arguments
///
/// * `ui`   - The UI to draw the button onto
/// * `list` - The list of tasks to sort
///
/// # Returns
///
/// `list` sorted with completed tasks at the top if the button is clicked, otherwise the original
/// List
fn draw_sort_button(ui: &mut Ui, list: List) -> List {
    if ui.button("Sort").clicked() {
        return list
            .clone()
            .into_iter()
            .filter(|task| task.completed())
            .chain(list.into_iter().filter(|task| !task.completed()))
            .collect();
    }
    list
}

/// Function to draw the delete completed tasks button
///
/// # Arguments
///
/// * `ui`   - The UI to draw the button onto
/// * `list` - The list of tasks to filter
///
/// # Returns
///
/// `list` sorted with completed tasks deleted if the button is clicked, otherwise the original
/// List
fn draw_delete_completed_tasks_button(ui: &mut Ui, list: List) -> List {
    if ui.button("Delete Completed").clicked() {
        return list.into_iter().filter(|task| !task.completed()).collect();
    }
    list
}

/// Function to draw the delete list button
///
/// # Arguments
///
/// * `ui` - The UI to draw the button onto
///
/// # Returns
///
/// Whether the button was clicked or not
fn draw_delete_list(ui: &mut Ui) -> bool {
    ui.button("Delete").clicked()
}

/// Function to draw the progress bar
///
/// # Arguments
///
/// * `ui`         - The UI to draw the progress bar onto
/// * `progress`   - The progress of the list
fn draw_progress_bar(ui: &mut Ui, progress: f32) {
    ui.add(egui::ProgressBar::new(progress).show_percentage());
}

/// Enum to model the result of the add_task function
enum AddTaskResult {
    /// The user has finished typing, add the task
    AddTask(String),
    /// The user has not finished typing
    ContinueTyping(String),
}

/// A function that draws the textedit box for adding a new task
///
/// # Arguments
///
/// * `ui`        - The UI to draw the textbox on
/// * `task_name` - The tasks to adds name, aka the contents of the textbox
///
/// # Returns
///
/// An AddTaskResult containg the end contents of the textbox
fn add_task(ui: &mut Ui, task_name: &str) -> AddTaskResult {
    let mut task_name = task_name.to_string();
    let mut add_task = false;
    Frame::none()
        .fill(Color32::LIGHT_GRAY)
        .outer_margin(Margin::symmetric(0.0, TASK_OUTER_MARGIN))
        .inner_margin(Margin::same(TASK_BORDER_WIDTH))
        .rounding(Rounding::same(TASK_ROUNDING - 2.0))
        .show(ui, |ui| {
            if ui
                .text_edit_singleline(&mut task_name)
                .on_hover_text("Add a new task")
                .lost_focus()
            {
                add_task = true;
            }
        });
    match (add_task, task_name.is_empty()) {
        // Only add the the task if the textbox has lost focus and the taskname is not empty
        (true, false) => AddTaskResult::AddTask(task_name),
        (_, _) => AddTaskResult::ContinueTyping(task_name),
    }
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
/// * `ui`      - The UI to draw the task UI widget onto
/// * `editing` - Whether the list window is in editing mode
/// * `task`    - The task for the widget to display
///
/// # Returns
///
/// An option containg either the task with modifications, or None if the task has been deleted
fn draw_task(ui: &mut Ui, editing: bool, task: Task) -> Option<Task> {
    let mut task = task;
    let mut delete_task = false;

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

                    if editing {
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
                                        delete_task = true;
                                    }
                                });
                            task.set_title(textfield(ui, task.title()));
                        });

                        task.set_description(textfield(ui, task.description()));
                    } else {
                        let title = task.title();
                        ui.checkbox(task.mut_completed(), title);

                        let description = task.description();
                        if !description.is_empty() {
                            ui.label(description);
                        }
                    }
                });
        });

    match delete_task {
        false => Some(task),
        true => None,
    }
}

/// Draws a singleline text edit to the UI with a frame border
///
/// # Arguments
///
/// * `ui`       - The UI to draw the text edit on
/// * `contents` - The contents of the text edit
///
/// # Returns
///
/// The contents of the text edit after user interaction
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
