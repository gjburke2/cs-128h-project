mod task;
mod tasklist;
use crate::task::Task;
use crate::tasklist::TaskList;

fn main() {
    // println!("Hello, world!");
    // Informal debugging

    let mut task_list: TaskList = TaskList::new();
    let task1: Task = Task::new("Eating", 30 * 60, 3);
    let task2: Task = Task::new("Computering", 30 * 60, 2);
    let task3: Task = Task::new("Sleeping", 3 * 60 * 60, 3);

    task_list.add(task1.clone());
    task_list.add(task2.clone());
    task_list.add(task3.clone());

    println!("{}", task_list.get_task(1).name);
    println!("{}", task_list.get_task(2).name);
    println!("{}", task_list.get_task(3).name);
    println!("{}", task_list.len());

    task_list.remove("Computering");

    println!("{}", task_list.get_task(1).name);
    println!("{}", task_list.get_task(2).name);
    println!("{}", task_list.len());
}
