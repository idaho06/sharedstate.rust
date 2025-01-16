use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use crate::state::State;

pub struct Clock {
    state_arc: Arc<Mutex<State>>,
}

impl Clock {
    pub fn new(state_arc: Arc<Mutex<State>>) -> Self {
        Self { state_arc }
    }

    // pub fn tick(&self) {
    //     let mut state = self.state_arc.lock().unwrap();
    //     state.tick();
    //     eprintln!("tick: {}", state.get_tick());
    //     drop(state);
    // }

    // run ticks
    pub fn run(&self) {
        loop {
            let mut state = self.state_arc.lock().expect("Failed to lock state");
            state.tick();
            drop(state);
            sleep(Duration::from_millis(33));
        }
    }

    // create a new thread to run ticks
    pub fn run_in_background(&self) {
        let state_arc = self.state_arc.clone();
        thread::spawn(move || {
            let clock = Clock { state_arc };
            clock.run();
        });
    }
}
