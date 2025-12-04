pub fn print_divider() {
    println!("==============================");
}

pub fn sleep_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub struct Config {
    pub theme: String,
    pub verbose: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            theme: "light".to_string(),
            verbose: false,
        }
    }
}
