use crate::task::Task;

#[derive(Clone)]
pub struct TaskList {
    pub front: Option<Box<TaskNode>>,
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
    pub fn get_task(&mut self, index: usize) -> Task {
        let mut i = 1;
        let mut curr = &mut self.front;
        while i < index {
            curr = &mut curr.as_mut().unwrap().next;
            i += 1;
        }
        return curr.as_ref().unwrap().task.clone();
    }
    // Add task (sorted by priority and time)
    pub fn add(&mut self, task: Task) {
        // Consider when list is empty
        if self.front.is_none() {
            self.front = Some(Box::new(TaskNode::new(task)));
            self.length += 1;
            return;
        }
        let mut curr = &mut self.front;
        // Consider when adding at the front
        if curr.as_ref().unwrap().task.priority < task.priority || 
                (curr.as_ref().unwrap().task.priority == task.priority && 
                curr.as_ref().unwrap().task.seconds_left < task.seconds_left) {
            let mut new_node = Some(Box::new(TaskNode::new(task)));
            new_node.as_mut().unwrap().next = curr.take();
            self.front = new_node;
            self.length += 1;
            return;
        }
        // Consider when list has one node
        if curr.as_ref().unwrap().next.is_none() {
            curr.as_mut().unwrap().next = Some(Box::new(TaskNode::new(task)));
            self.length += 1;
            return;
        }
        // Rest of the cases
        while curr.as_ref().unwrap().next.is_some() && (
            curr.as_ref().unwrap().next.as_ref().unwrap().task.priority > task.priority || 
            (curr.as_ref().unwrap().next.as_ref().unwrap().task.priority == task.priority && curr.as_ref().unwrap().next.as_ref().unwrap().task.seconds_left > task.seconds_left)
        ) {
            curr = &mut curr.as_mut().unwrap().next;
        }
        let temp = &mut curr.as_mut().unwrap().next;
        let mut new_node = Some(Box::new(TaskNode::new(task)));
        new_node.as_mut().unwrap().next = temp.take();
        curr.as_mut().unwrap().next = new_node.take();
        self.length += 1;
    }
    // Take task away (by name)
    pub fn remove(&mut self, name: &str) -> Option<Task> {
        if self.front.is_none() {
            return None;
        }
        let mut index: u32 = 0;
        let mut current = &mut self.front;
        while current.clone().unwrap().next.is_some() {
            if index == 0 && current.as_mut().unwrap().task.name == String::from(name) {
                let task: Task = self.front.clone().unwrap().task;
                self.front = self.front.clone().unwrap().next;
                self.length -= 1;
                return Some(task);
            }
            if current.clone().unwrap().next.unwrap().task.name == String::from(name) {
                let task: Task = current.clone().unwrap().next.unwrap().task.clone();
                current.as_mut().unwrap().next = current.clone().unwrap().next.unwrap().next;
                self.length -= 1;
                return Some(task);
            }
            index += 1;
            current = &mut current.as_mut().unwrap().next;
        }
        return None;
    }
    // TODO: User interactive save and load functions
}

impl TaskNode {
    fn new(task: Task) -> Self {
        TaskNode {
            task,
            next: None,
        }
    }
}