pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: true }
    }

    pub fn run(&mut self) {
        println!("mangsh running... (placeholder)");
        self.running = false;
    }
}
