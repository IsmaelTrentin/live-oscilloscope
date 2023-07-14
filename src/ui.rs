use crate::audio::*;
use crate::State;
use std::sync::{Arc, Mutex};

pub fn update_ui_samples(state_clone: Arc<Mutex<State>>) {
    let stream = get_input_stream().expect("could not open audio stream");
    stream.start().expect("failed to start audio stream");

    loop {
        let data = stream.read(1024).unwrap();
        // println!("tx: sending {} samples", data.len());
        state_clone.lock().unwrap().current_samples = data;
    }
}
