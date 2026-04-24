use crate::prelude::*;

pub struct Axis(f64);

pub struct Button(bool);

pub enum Input {
    Axis,
    Button,
}

pub struct InputService {
    input_registry: HashMap<String, Input>,
}
