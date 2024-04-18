mod task;
mod tasklist;
use crate::task::Task;
use crate::tasklist::TaskList;
use crate::tasklist::load;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::thread::JoinHandle;
use std::sync::{mpsc, mpsc::Receiver};

fn main() {
    // println!("Hello, world!");
    // Informal debugging

    let mut task_list: TaskList = TaskList::new("Daily");
    let task1: Task = Task::new("Eating", 5, 3);
    let task2: Task = Task::new("Computering", 10, 2);
    let task3: Task = Task::new("Sleeping", 10, 3);

    task_list.add(task1.clone());
    task_list.add(task2.clone());
    task_list.add(task3.clone());

    /*
    println!("{}", task_list.get_task(1).seconds_left);
    println!("{}", task_list.get_task(2).seconds_left);
    println!("{}", task_list.get_task(3).seconds_left);
    println!("{}", task_list.len());

    task_list.remove("Computering");

    println!("{}", task_list.get_task(1).seconds_left);
    println!("{}", task_list.get_task(2).seconds_left);
    println!("{}", task_list.len());
    

    let _ = task_list.save();
    let mut new_task_list = load("Daily");
    */

    println!("Before:");
    println!("{}", task_list.get_task(1).seconds_left);
    println!("{}", task_list.get_task(2).seconds_left);
    println!("{}", task_list.get_task(3).seconds_left);
    
    // General form of starting and stopping tasks
    // NOTE: CAN ONLY HAVE ONE MUTABLE GET AT A TIME (reuqires mutable reference of tasklist for each get_mut, so can only do one at a time)
    let should_stop = Arc::new(AtomicBool::new(false));
    let first_task = task_list.get_mut_task(1);
    let (handle1, rx1) = first_task.start(should_stop.clone());
    thread::sleep(Duration::from_secs(15));
    first_task.stop(handle1, rx1, should_stop.clone());

    println!("After:");
    println!("{}", task_list.get_task(1).completion_status);
    println!("{}", task_list.get_task(2).completion_status);
    println!("{}", task_list.get_task(3).completion_status);
}
