use std::time::Duration;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender };
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

pub struct TimeLeft {
    pub sender: Sender<u32>,
    pub time_left: u32,
}

impl TimeLeft {
    fn new(time_left: u32) -> (Self, Receiver<u32>) {
        let (sender, receiver) = mpsc::channel();
        (TimeLeft{sender, time_left}, receiver)
    }
    fn send_value(&self, value: u32) {
        self.sender.send(value).expect("Couldn't send value");
    }
}
impl Drop for TimeLeft {
    fn drop(&mut self) {
        if self.time_left <= 0 {
            println!("Task Finished");
        }
        self.send_value(self.time_left);
    }
}

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub seconds_left: u32,
    pub priority: u32,
    pub completion_status: bool,
}

impl Task {
    // Constructor
    pub fn new(name: &str, seconds_left: u32, priority: u32) -> Self {
        Task {
            name: name.to_string(),
            seconds_left,
            priority,
            completion_status: false,
        }
    }
    // Main functions (WIP, SIGNATURES MAY NEED MODIFYING)
    // Start and stop are functions that only modify instance variables
    // Thread handling must be done in main
    pub fn start(&self, should_stop: Arc<AtomicBool>) -> (JoinHandle<()>, Receiver<u32>) {
        let self_clone = self.clone();
        let (time_left, receiver) = TimeLeft::new(self_clone.seconds_left);
        let handle: JoinHandle<()> = thread::spawn( move || {
            let mut time = time_left;
            while time.time_left > 0 && !should_stop.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(5));
                time.time_left -= 5;
            }
        });
        return (handle, receiver)
    }
    pub fn stop(&mut self, handle: JoinHandle<()>, receiver: Receiver<u32>, should_stop: Arc<AtomicBool>) {
        should_stop.store(true, Ordering::Relaxed);
        handle.join().unwrap();
        let new_time = match receiver.recv() {
            Ok(new_time) => new_time,
            Err(_) => 0,
        };
        self.seconds_left = new_time;
        if self.seconds_left <= 0 {
            self.completion_status = true;
        }
    }
    // Write function for data storage
    pub fn write(&self) -> String{
        return self.name.clone() + "," + &self.seconds_left.to_string() + "," + &self.priority.to_string() + "," + &self.completion_status.to_string() + ",";
    }
}

// Reading the output of the data storage, creating an object based off it
pub fn read(input: &str) -> Task {
    let mut params: Vec<String> = Vec::new();
    let mut param: String = String::from("");
    for c in input.chars() {
        match c {
            ',' => {
                params.push(param);
                param = String::from("");
            },
            _ => param.push(c),
        };
    }
    return Task {
        name: params[0].clone(),
        seconds_left: match params[1].trim().parse::<u32>() {
            Ok(parsed) => parsed,
            Err(_) => 0,
        },
        priority: match params[2].trim().parse::<u32>() {
            Ok(parsed) => parsed,
            Err(_) => 0,
        },
        completion_status: match params[3].trim().parse::<bool>() {
            Ok(parsed) => parsed,
            Err(_) => false,
        }, 
    };
}