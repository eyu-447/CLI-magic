use std::io::{stdout, Write};
use std::time::Instant;

pub struct ProgressBar {
    total: u64,
    position: u64,
    start_time: Instant,
    width: u64,
}

impl ProgressBar {
    pub fn new(total: u64) -> Self {
        ProgressBar {
            total,
            position: 0,
            start_time: Instant::now(),
            width: 40,
        }
    }

    pub fn set_position(&mut self, pos: u64) {
        self.position = pos;
        self.draw();
    }

    fn draw(&self) {
        let percentage = self.position as f64 / self.total as f64;
        let filled = (self.width as f64 * percentage).round() as u64;
        let empty = self.width - filled;
        let bar = format!(
            "[{}{}] {}%",
            "█".repeat(filled as usize),
            "░".repeat(empty as usize),
            (percentage * 100.0).round()
        );
        let elapsed = self.start_time.elapsed().as_secs();
        print!("\r{} Elapsed: {}s", bar, elapsed);
        stdout().flush().unwrap();
    }

    pub fn finish(&self, msg: &str) {
        self.draw();
        println!("\n{}", msg);
    }
}
