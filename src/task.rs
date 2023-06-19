#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct List(Vec<Task>);

impl List {
    pub fn new() -> Self {
        Self { 0: vec![] }
    }

    pub fn add(&mut self, task: Task) {
        self.0.push(task);
    }

    /// The progress of the list as a percentage
    pub fn progress(&self) -> f32 {
        let completed_value = self.0.len() as f32;
        let completed_tasks = self.0.iter().filter(|x| x.completed).count() as f32;
        completed_tasks / completed_value
    }
}

impl IntoIterator for List {
    type Item = Task;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Task> for List {
    fn from_iter<T: IntoIterator<Item = Task>>(iter: T) -> Self {
        let mut list = Self::new();
        for i in iter {
            list.add(i);
        }
        list
    }
}

#[derive(Builder, Clone, serde::Deserialize, serde::Serialize)]
pub struct Task {
    /// Whether the task has been completed
    #[builder(default = "false")]
    completed: bool,
    /// The title of the task
    title: String,
    /// The description of the task
    #[builder(default = "String::new()")]
    description: String,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        TaskBuilder::default()
    }

    /// Gets a mutable reference to the completed field of the task
    pub fn mut_completed(&mut self) -> &mut bool {
        &mut self.completed
    }

    /// A clone of the tasks title
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// A clone of the tasks description
    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let task = Task::builder()
            .title("Testing".to_string())
            .build()
            .unwrap();
        assert_eq!(task.completed, false);
        assert_eq!(task.description, String::new());
    }
}
