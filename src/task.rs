use serde::{Deserialize, Serialize};

/// Enum for representing the status of a task
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Completed,
    InProgress,
    NotStarted,
}

/// Enum for storing possible errors
#[derive(Debug, PartialEq, Eq)]
pub struct EmptyDescription;

/// Struct to represent a task
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Task {
    new_description: String,
    description: String,
    completed: bool,
    id: usize 
}

impl Task {
    /// Returns a new Task struct with the description and status passed in
    ///
    /// Parameters
    /// description:   The task's description
    /// id:            The task's id
    pub fn new(description: String, id: usize) -> Result<Task, EmptyDescription> {
        // Return an error if the description is empty
        if description.is_empty() {
            return Err(EmptyDescription);
        }

        Ok(Task {
            new_description: String::new(),
            description,
            completed: false,
            id,
        })
    }

    /// Returns a clone of self.new_description
    pub fn new_description(&self) -> String {
        self.new_description.clone()
    }

    /// Returns a mutable reference to self.new_description
    pub fn mut_new_description(&mut self) -> &mut String {
        &mut self.new_description
    }

    /// Sets the value of new_description, currently no validation checking
    pub fn set_new_description(&mut self, new_description: String) {
        self.new_description = new_description;
    }

    /// Returns a clone of the tasks description
    pub fn description(&self) -> String {
        self.description.clone()
    }

    /// Returns if the task is completed or not
    pub fn completed(&self) -> bool {
        self.completed
    }

    /// Sets whether the task is completed or not
    pub fn set_completed(&mut self, value: bool) {
        self.completed = value;
    }

    /// Updates the description of the task
    ///
    /// Parameters
    /// new_description:   The new description of the task
    pub fn update_description(&mut self, new_description: String) -> Result<(), EmptyDescription> {
        // Return an error if the new description is empty
        if new_description.is_empty() {
            return Err(EmptyDescription);
        }

        self.description = new_description;

        Ok(())
    }
}

impl Default for Task {
    // A default implementation for task
    fn default() -> Self { 
        Task {
            new_description: String::new(),
            description: "A default task!".to_owned(),
            completed: false,
            id: 0,
        }
    }
}

/// Unit tests
mod tests {
    #![allow(unused_imports)]
    // For some reason, clippy says this isn't needed, howeveer deleting it breaks everything so
    // I've attached the allow unused_imports atribute
    use super::*;

    #[test]
    /// Checks if the constructor works with the expected input
    fn constructor_right_description() {
        let description = String::from("This is a simple task!");

        let task = Task::new(description.clone(), 0).unwrap();

        assert_eq!(task.description(), description);
    }

    #[test]
    /// Checks if the constructor will provide the correct error when passed an empty description
    fn constructor_fails_on_empty_description() {
        let description = String::new();

        let task_error = Task::new(description, 0).unwrap_err();

        assert_eq!(task_error, EmptyDescription)
    }

    #[test]
    /// Checks if the update_description method works
    fn update_description_works() {
        let description = String::from("This is the first description");

        let mut task = Task::new(description, 0).unwrap();

        let new_description = String::from("The is the new description");

        task.update_description(new_description.clone()).unwrap();

        assert_eq!(task.description(), new_description)
    }

    #[test]
    /// Checks if the update_description fails when passed an empty description
    fn update_description_fails_on_empty_description() {
        let description = String::from("This is the first description");

        let mut task = Task::new(description, 0).unwrap();

        let new_description = String::new();

        let err = task
            .update_description(new_description)
            .unwrap_err();

        assert_eq!(err, EmptyDescription)
    }
}
