mod task;
mod tasklist;
use crate::tasklist::run;

fn main() {
    println!("Welcome to the task manager! \nType 'help' for a list of commands you might find useful!");
    // This command launches the program
    run();
}
