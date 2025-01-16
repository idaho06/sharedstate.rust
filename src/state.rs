pub struct State {
    tick: u64,
}

impl State {
    pub fn new() -> Self {
        State { tick: 0 }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }

    pub fn get_tick(&self) -> u64 {
        self.tick
    }
}
