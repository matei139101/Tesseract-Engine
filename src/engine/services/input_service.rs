use crate::prelude::*;

pub enum Input {
    Axis(f64),
    Button(bool),
}

pub struct InputService {
    input_registry: HashMap<String, Input>,
    mappings: HashMap<String, String>,
}

impl InputService {
    pub fn new() -> Self {
        InputService { input_registry: HashMap::new(), mappings: HashMap::new() }
    }
}
