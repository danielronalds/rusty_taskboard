use crate::task::Task;

/// A struct representing a list of tasks, with a name
#[derive(Default, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct List {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl List {
    /// Creates a new List struct
    ///
    /// Parameters
    /// name:    The name of the list
    /// tasks:   The list of tasks in the list
    ///
    /// Returns
    /// A new List struct
    pub fn new(name: String, tasks: Vec<Task>) -> Self {
        Self { name, tasks }
    }

    /// Calculates the progress of the list
    ///
    /// Returns
    /// A f32 between 0.0 and 1.0 that represents the list's progress as a percentage
    pub fn progress(&self) -> f32 {
        // Calculating what the max completed_value would be
        let completed_value = self.tasks.len() as f32;

        // Adding the completed tasks together
        let mut completed_task: f32 = 0.0;

        for task in &self.tasks {
            if task.completed() {
                completed_task += 1.0;
            }
        }

        // Returning a percentage
        completed_task / completed_value
    }
}

// Const for the width of the list windows
pub const DEFAULT_LIST_WINDOW_WIDTH: f32 = 250.0;

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
/// A struct representing the a window to display of a List
pub struct ListWindow {
    show: bool,
    new_task_description: String,
    new_list_name: String,
    edit_mode: bool,
    list: List,
    progress: f32,
}

impl ListWindow {
    // Creates a new ListWindow struct
    //
    // Parameters
    // list:   The list that the ListWindow uses
    //
    // Returns
    // A new ListWindow Struct
    pub fn new(list: List) -> Self {
        Self {
            show: true,
            new_task_description: String::new(),
            // Setting the value of new_list_name to the list_name so that the textbox starts with
            // the name in it
            new_list_name: list.name.clone(),
            edit_mode: false,
            list,
            progress: 0.0,
        }
    }

    /// Displays the list window, and returns whether it should be deleted or not
    ///
    /// Parameters
    /// ctx:                 EGUI Context
    /// list_window_width:   The width to display the window with
    ///
    /// Returns
    /// None if the list shouldn't be deleted, and a clone of self wrapped in Some() if this list
    /// should be deleted
    pub fn display(&mut self, ctx: &egui::Context, list_window_width: f32) -> Option<Self> {
        let mut list_to_delete = None;

        egui::Window::new(self.list_name())
            .resizable(false)
            .show(ctx, |ui| {
                // Setting the width
                ui.set_width(list_window_width);

                self.animate_bar(ctx);

                ui.add(egui::ProgressBar::new(self.progress).show_percentage());

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Add ");
                    // Way of adding more tasks to the list
                    if ui
                        .text_edit_singleline(&mut self.new_task_description)
                        .on_hover_text("Add a new task")
                        .lost_focus()
                    {
                        // Getting the tasks id
                        let id = self.task_vec().len();

                        // Attempting to create the task
                        if let Ok(task) = Task::new(self.new_task_description.clone(), id) {
                            // Adding the created task
                            self.list.tasks.push(task);

                            // Resetting the textbox by setting its value to a new String
                            self.new_task_description = String::new();
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
                for i in 0..self.task_vec().len() {
                    ui.horizontal_wrapped(|ui| {
                        // Displaying the delete button if edit mode is enabled
                        if self.edit_mode && ui.button("X").clicked() {
                            task_to_be_deleted = Some(self.task_vec()[i].clone());
                        }

                        // Displaying the textbox for editing a task's description if edit_mode is
                        // enabled
                        if self.edit_mode {
                            ui.text_edit_singleline(self.mut_task_vec()[i].mut_new_description());
                            return;
                        }

                        // If update_task is false, then the user must be finished editing
                        // tasks. If new_description is empty, set it to the current
                        // description, else set it as the new description
                        if self.task_vec()[i].new_description().is_empty() {
                            let new_description = self.task_vec()[i].description();
                            self.mut_task_vec()[i].set_new_description(new_description);
                        } else {
                            let new_description = self.task_vec()[i].new_description();
                            self.mut_task_vec()[i]
                                .update_description(new_description)
                                .unwrap_or(());
                        }

                        // Little work around the borrow checker
                        let mut value = self.task_vec()[i].completed();
                        ui.checkbox(&mut value, self.task_vec()[i].description());
                        self.mut_task_vec()[i].set_completed(value);
                    });
                }

                // Currently the problem with this is that it doesnt only delete the one task
                if let Some(task_to_delete) = task_to_be_deleted {
                    // Deleting the task
                    self.list.tasks.retain(|task| task != &task_to_delete);
                }

                ui.add_space(10.0);

                ui.collapsing("Options", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        if ui
                            .text_edit_singleline(&mut self.new_list_name)
                            .lost_focus()
                            && !self.new_list_name.is_empty()
                        {
                            self.list.name = self.new_list_name.clone();
                        }
                    });

                    ui.add_space(10.0);
                    ui.checkbox(&mut self.edit_mode, "Edit mode");

                    ui.add_space(10.0);
                    if ui.button("Delete completed tasks").clicked() {
                        // Deleting completed tasks
                        self.list.tasks.retain(|task| !task.completed());
                    }

                    ui.add_space(10.0);
                    if ui.button("Delete list").clicked() {
                        list_to_delete = Some(self.clone());
                    }
                });
            });

        list_to_delete
    }

    /// Returns whether to show the ListWindow or not
    pub fn show(&self) -> bool {
        self.show
    }

    /// Sets the value of self.show
    pub fn set_show(&mut self, value: bool) {
        self.show = value
    }

    /// Returns the list name
    pub fn list_name(&self) -> String {
        self.list.name.clone()
    }

    /// Returns a reference to self.list.tasks 
    ///
    /// Keeping this one as it cleans up the code pretty nicely
    fn task_vec(&self) -> &Vec<Task> {
        &self.list.tasks
    }

    /// Returns a mutable reference to self.list.tasks
    ///
    /// Also keeping this for the same reason
    fn mut_task_vec(&mut self) -> &mut Vec<Task> {
        &mut self.list.tasks
    }

    /// Returns a clone of self.list.tasks
    pub fn task_vec_clone(&self) -> Vec<Task> {
        self.list.tasks.clone()
    }

    /// Animates the progress bar
    ///
    /// Parameters
    /// ctx:   The GUI context... I don't super love that this is apart of the method however the
    /// refactor can come later
    fn animate_bar(&mut self, ctx: &egui::Context) {
        // Progress bar to show how much of the list is done
        // If the progress is within a certain range, just set it to exactly the
        // progress
        if (self.progress - self.list.progress()) < 0.01
            && (self.progress - self.list.progress()) > -0.01
            && !(self.progress == self.list.progress())
        {
            self.progress = self.list.progress();
        } else if self.list.progress() < self.progress {
            self.progress -= 0.01;
            // Requesting a repaint so that the animation is smooth
            ctx.request_repaint();
        } else if self.list.progress() > self.progress {
            self.progress += 0.01;
            // Requesting a repaint so that the animation is smooth
            ctx.request_repaint();
        }
    }
}

#[cfg(test)]
/// Module for unit tests
mod tests {}
