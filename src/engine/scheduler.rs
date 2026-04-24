use std::thread;

struct Scheduler {
    
}

impl Scheduler {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn start(&self) {
        thread::spawn(|| {

        });
    }
}
