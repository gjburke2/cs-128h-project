use crate::task::Task;
use crate::task::read;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use std::io::LineWriter;
use std::path::PathBuf;
use dialoguer::Input;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread::JoinHandle;
use std::sync::mpsc::Receiver;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TaskList {
    pub front: Option<Box<TaskNode>>,
    length: usize,
    name: String,
    pub active_tasks: HashMap<String, (JoinHandle<()>, Arc<AtomicBool>, Receiver<u32>)>, // To keep track of running tasks
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
            name: input_name.to_string(),
            active_tasks: HashMap::new(), // Initialize the hashmap here
        }
    }
    // Length getter
    pub fn len(&self) -> usize {
        self.length
    }
    // Get mutable task by index contant (input will be one-indexed) {
    pub fn get_mut_task(&mut self, index: usize) -> &mut Task {
        let mut curr = &mut self.front;
        let mut i = 1;
        while i < index {
            curr = &mut curr.as_mut().unwrap().next;
            i += 1;
        }
        return &mut curr.as_mut().unwrap().task;
    }
    pub fn get_task(&self, index: usize) -> Task {
        let mut curr = &self.front;
        let mut i = 1;
        while i < index {
            curr = &curr.as_ref().unwrap().next;
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

// Function for running the program (used in main) 
pub fn run() {
    let mut curr_list: TaskList = TaskList::new("Default");
    loop {
        // Get user's input
        let input: String = Input::new().with_prompt("->").interact_text().expect("Has to be a string");
        let args: Vec<&str> = input.trim().split(' ').collect();
        // If there are no arguments handle and continue the loop
        if args.len() == 0 {
            println!("You must provide an argument.");
            continue;
        }
        // Match the first word to certain functionalities
        match args[0] {
            "create" => {
                if args.len() != 2 {
                    println!("Too many or too few arguments.");
                    continue;
                }
                curr_list = TaskList::new(args[1]);
                task_creator(&mut curr_list);
                curr_list.save();
            },
            "load" => {
                if args.len() != 2 {
                    println!("Too many or too few arguments.");
                    continue;
                }
                curr_list = load(args[1]);
            },
            "help" => {
                println!("List of commands and what they do:");
                println!("The command 'load' followed by the name of a task list loads that task list from those previously created.");
                println!("The command 'create' followed by a name creates a new task list under that name and saves it.");
                println!("When a task list is loaded, the command 'start' followed by the name of a task commences the timer on that task.");
                println!("When a task is started, the command 'pause' pauses the timer associated with that task.");
                println!("Finally, the command 'exit' terminates the program.");
            },
            "exit" => {
                break;
            },
            "start" => {
                if args.len() != 2 {
                    println!("Please specify a task name to start.");
                    continue;
                }
                let task_name = args[1];
                if let Some(task) = curr_list.get_mut_task_by_name(task_name) {
                    if !task.completion_status {
                        let should_stop = Arc::new(AtomicBool::new(false));
                        let (handle, receiver) = task.start(should_stop.clone());
                        curr_list.active_tasks.insert(task_name.to_string(), (handle, should_stop, receiver));
                        println!("Task '{}' has been started.", task_name);
                    } else {
                        println!("Task '{}' is either already completed or currently running.", task_name);
                    }
                } else {
                    println!("Task '{}' not found.", task_name);
                }
            },
            "pause" => {
                if args.len() != 2 {
                    println!("Please specify a task name to pause.");
                    continue;
                }
                let task_name = args[1];
                if let Some((handle, should_stop, receiver)) = curr_list.active_tasks.remove(task_name) {
                    should_stop.store(true, Ordering::Relaxed);
                    handle.join().expect("Failed to join task thread");
                    if let Ok(time_left) = receiver.try_recv() {
                        if let Some(task) = curr_list.get_mut_task_by_name(task_name) {
                            task.seconds_left = time_left;
                            task.completion_status = time_left == 0;
                            println!("Task '{}' has been paused with {} seconds left.", task_name, time_left);
                        }
                    } else {
                        println!("No time left information available for task '{}'.", task_name);
                    }
                } else {
                    println!("Task '{}' is not currently running.", task_name);
                }
            },
            _ => {
                println!("Invalid command.");
            },
        }
    }
}



pub fn task_creator(task_list: &mut TaskList) {
    println!("To add a task, enter a name followed by the time it takes (integer in seconds) and a priority value (integer).");
    println!("Enter command 'done' when you no longer wish to add tasks.");
    loop {
        let prompt: &str = "->";
        let input: String = Input::new().with_prompt(prompt).interact_text().expect("Has to be a string");
        let args: Vec<&str> = input.trim().split(' ').collect();
        if args[0] == "done" {
            break;
        }
        if args.len() != 3 {
            println!("Too many or too few arguments");
            continue;
        }
        let time: u32 = args[1].to_string().parse().expect("Has to be an unsigned integer!!");
        let priority: u32 = args[2].to_string().parse().expect("Has to be an unsigned integer!!");
        let task: Task = Task::new(args[0], time, priority);
        task_list.add(task);
    }
}
