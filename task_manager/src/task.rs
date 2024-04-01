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
    // Start and stop may need to take in/return handles, recievers, etc if we want to multi-thread
    pub fn start() {

    }
    pub fn stop() {

    }
}