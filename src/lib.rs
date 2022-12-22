// Public facing modules
pub mod task;

use crate::task::{Task, TaskErrors, TaskStatus};

/// Consumes the given vec and returns two vecs of tasks, with the first one containing all the
/// tasks in the current list, and the second containg the rest of the tasks
///
/// Parameters
/// task_vec:   The unfiltered Vec<Task>
/// config:     The user's config
pub fn filter_task_vec(task_vec: Vec<Task>, current_list: &str) -> (Vec<Task>, Vec<Task>) {
    let mut tagged_tasks: Vec<Task> = Vec::new();

    let mut other_tasks: Vec<Task> = Vec::new();

    for task in task_vec {
        if task.list() == current_list {
            tagged_tasks.push(task);
            continue;
        }
        other_tasks.push(task);
    }

    (tagged_tasks, other_tasks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests if the filters_tasks function works as expected
    fn filter_tasks_works() {
        let tasks_vec = vec![
            Task::new(
                String::from("A basic task"),
                TaskStatus::Completed,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task"),
                TaskStatus::Completed,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("A basic task"),
                TaskStatus::Completed,
                String::from("Other"),
            )
            .unwrap(),
            Task::new(
                String::from("A basic task"),
                TaskStatus::Completed,
                String::from("Another"),
            )
            .unwrap(),
        ];

        let filtered_vecs = filter_task_vec(tasks_vec, "Main");

        assert_eq!(
            filtered_vecs.0,
            vec![
                Task::new(
                    String::from("A basic task"),
                    TaskStatus::Completed,
                    String::from("Main"),
                )
                .unwrap(),
                Task::new(
                    String::from("Another basic task"),
                    TaskStatus::Completed,
                    String::from("Main"),
                )
                .unwrap(),
            ]
        );

        assert_eq!(
            filtered_vecs.1,
            vec![
                Task::new(
                    String::from("A basic task"),
                    TaskStatus::Completed,
                    String::from("Other"),
                )
                .unwrap(),
                Task::new(
                    String::from("A basic task"),
                    TaskStatus::Completed,
                    String::from("Another"),
                )
                .unwrap(),
            ]
        );
    }
}
