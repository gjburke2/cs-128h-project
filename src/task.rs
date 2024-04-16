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