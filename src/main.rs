use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use clock::Clock;
use state::State;

mod clock;
mod state;
fn main() {
    println!("Welcome!");

    let state_arc = Arc::new(Mutex::new(State::new()));
    let clock1 = Clock::new(state_arc.clone());
    let clock2 = Clock::new(state_arc.clone());

    clock1.run_in_background();
    clock2.run_in_background();

    loop {
        sleep(Duration::from_secs(1));
        println!("Press ENTER to see the current tick count or 'q' to quit.");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        if input.trim() == "q" {
            break;
        }
        let state = state_arc.lock().expect("Failed to lock state");
        println!("Current tick count: {}", state.get_tick());
    }

    println!("Goodbye!");
}
