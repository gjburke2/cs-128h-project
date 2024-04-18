use crate::task::Task;
use crate::task::read;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::PathBuf;

#[derive(Clone)]
pub struct TaskList {
    pub front: Option<Box<TaskNode>>,
    length: usize,
    name: String
}

#[derive(Clone)]
pub struct TaskNode {
    task: Task,
    next: Option<Box<TaskNode>>
}

impl TaskList {
    // Initializer
    pub fn new(input_name: &str) -> Self {
        TaskList {
            front: None,
            length: 0,
            name: input_name.to_string()
        }
    }
    // Length getter
    pub fn len(&self) -> usize {
        self.length
    }
    // Get task by index contant (input will be one-indexed) {
        pub fn get_task(&mut self, index: usize) -> &mut Task {
            let mut curr = &mut self.front;
            let mut i = 1;
            while i < index {
                curr = &mut curr.as_mut().unwrap().next;
                i += 1;
            }
            return &mut curr.as_mut().unwrap().task;
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
    // Saves to text file in lists folder
    pub fn save(&mut self) -> std::io::Result<()>{
        let mut path = PathBuf::new();
        path.push("lists");
        path.push(self.name.clone() + ".txt");
        let file = File::create(path)?;
        let mut file = LineWriter::new(file);
        file.write_all(self.name.clone().as_bytes())?;
        let mut ind = 1;
        while ind <= self.length {
            file.write_all(b"\n")?;
            file.write_all(self.get_task(ind).write().as_bytes())?;
            ind += 1;
        }
        let _ = file.flush();
        Ok(())
    }
}

// Loads from list name
pub fn load(task_name: &str) -> TaskList {
    // Creating file
    let mut path = PathBuf::new();
    path.push("lists");
    path.push(task_name.to_owned() + ".txt");
    let file = File::open(path).expect("Can't open file");
    let file = BufReader::new(file);
    // Getting first two lines, creating object
    let mut file_lines = file.lines().map(|line| line.expect("Can't read line"));
    let name: String = file_lines.next().unwrap().parse().unwrap();
    let mut new_task_list = TaskList::new(&name);
    // Getting the rest
    for line in file_lines {
        new_task_list.add(read(&line));
    }
    return new_task_list;
}

impl TaskNode {
    fn new(task: Task) -> Self {
        TaskNode {
            task,
            next: None,
        }
    }
}


