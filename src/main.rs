#![allow(dead_code)]

mod window;
mod prelude;
mod engine;

fn main() {
    window::WindowHandle::default().start_event_loop();
}
