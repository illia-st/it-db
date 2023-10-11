pub enum Action {
    Tick,
    Increment,
    Decrement,
    Quit,
    None,
}

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}