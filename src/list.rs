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
    /// Parameters:
    /// name:    The name of the list
    /// tasks:   The list of tasks in the list
    pub fn new(name: String, tasks: Vec<Task>) -> Self {
        Self { name, tasks }
    }

    /// Calculates the progress of the list
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

#[derive(Clone, serde::Deserialize, serde::Serialize)]
/// A struct representing the a window to display of a List
pub struct ListWindow {
    show: bool,
    new_task_description: String,
    new_list_name: String,
    delete_mode: bool,
    update_tasks: bool,
    list: List,
    progress: f32,
}

impl ListWindow {
    // Creates a new ListWindow struct
    pub fn new(list: List) -> Self {
        Self {
            show: true,
            new_task_description: String::new(),
            // Setting the value of new_list_name to the list_name so that the textbox starts with
            // the name in it
            new_list_name: list.name.clone(),
            delete_mode: false,
            update_tasks: false,
            list,
            progress: 0.0,
        }
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

    /// Sets the list name
    pub fn set_list_name(&mut self, new_name: String) {
        self.list.name = new_name;
    }

    /// Returns self.delete_mode
    pub fn delete_mode(&self) -> bool {
        self.delete_mode
    }

    /// Returns a mutable reference to self.delete_mode
    pub fn mut_delete_mode(&mut self) -> &mut bool {
        &mut self.delete_mode
    }

    /// Returns self.update_tasks
    pub fn update_tasks(&self) -> bool {
        self.update_tasks
    }

    /// Returns a mutable reference to self.update_tasks
    pub fn mut_update_tasks(&mut self) -> &mut bool {
        &mut self.update_tasks
    }

    /// Returns the progress bar progress
    pub fn progress(&self) -> f32 {
        self.progress
    }

    /// Returns a clone of the new_task_description field
    pub fn new_task_description(&self) -> String {
        self.new_task_description.clone()
    }

    /// Returns a mutable reference to the new_task_description field
    pub fn mut_new_task_description(&mut self) -> &mut String {
        &mut self.new_task_description
    }

    /// Resets the new_task_description field to String::new()
    pub fn reset_new_task_description(&mut self) {
        self.new_task_description = String::new();
    }

    /// Returns a clone of new_list_name
    pub fn new_list_name(&self) -> String {
        self.new_list_name.clone()
    }

    /// Returns a mutable reference of new_list_name
    pub fn mut_new_list_name(&mut self) -> &mut String {
        &mut self.new_list_name
    }
    
    /// Returns a reference to self.list.tasks
    pub fn task_vec(&self) -> &Vec<Task> {
        &self.list.tasks
    }

    /// Returns a mutable reference to self.list.tasks
    pub fn mut_task_vec(&mut self) -> &mut Vec<Task> {
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
    pub fn animate_bar(&mut self, ctx: &egui::Context) {
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

    /// Adds a task to the list
    pub fn add_task(&mut self, task: Task) {
        self.list.tasks.push(task);
    }

    /// Deletes a task from the list
    pub fn delete_task(&mut self, task_to_delete: Task) {
        self.list.tasks.retain(|task| task != &task_to_delete);
    }

    /// Deletes completed tasks from the list
    pub fn delete_completed_tasks(&mut self) {
        self.list.tasks.retain(|task| !task.completed());
    }
}

#[cfg(test)]
/// Module for unit tests
mod tests {}
