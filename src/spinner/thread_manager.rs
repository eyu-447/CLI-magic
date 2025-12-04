use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Spinner {
    message: String,
    running: Arc<Mutex<bool>>,
    pattern: Vec<&'static str>,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        Spinner {
            message: message.to_string(),
            running: Arc::new(Mutex::new(false)),
            pattern: vec!["|", "/", "-", "\\"],
        }
    }

    pub fn start(&mut self) {
        let running = Arc::clone(&self.running);
        *running.lock().unwrap() = true;
        let message = self.message.clone();
        let pattern = self.pattern.clone();

        thread::spawn(move || {
            let mut i = 0;
            while *running.lock().unwrap() {
                print!("\r{} {}", pattern[i % pattern.len()], message);
                i += 1;
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn stop(&self) {
        let mut run = self.running.lock().unwrap();
        *run = false;
        println!("\r✔ {}", self.message);
    }

    pub fn success(&self, msg: &str) {
        let mut run = self.running.lock().unwrap();
        *run = false;
        println!("\r✅ {}", msg);
    }

    pub fn fail(&self, msg: &str) {
        let mut run = self.running.lock().unwrap();
        *run = false;
        println!("\r❌ {}", msg);
    }

    pub fn warning(&self, msg: &str) {
        let mut run = self.running.lock().unwrap();
        *run = false;
        println!("\r⚠️ {}", msg);
    }
}
