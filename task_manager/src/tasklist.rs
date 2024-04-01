use crate::task::Task;

#[derive(Clone)]
pub struct TaskList {
    pub front: Option<Box<Task>>,
    length: usize,
}

#[derive(Clone)]
pub struct TaskNode {
    task: Task,
    next: Option<Box<TaskNode>>
}

impl TaskList {
    // Initializer
    pub fn new() -> Self {
        TaskList {
            front: None,
            length: 0,
        }
    }
    // Length getter
    pub fn len(&self) -> usize {
        self.length
    }
    // Get task by index (input will be one-indexed) {
    pub fn get_task(index: usize) -> Task {
        Task::new("", 0, 0)
    }
    // Add task (sorted by priority and time)
    pub fn add(&mut self, task: TaskNode) {

    }
    // Take task away (by name)
    pub fn remove(&mut self, name: &str) {

    }

}

impl TaskNode {
    fn new(task: Task) -> Self {
        TaskNode {
            task,
            next: None,
        }
    }
}