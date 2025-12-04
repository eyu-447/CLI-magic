use crate::color::engine::*;
use chrono::Local;

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Success,
}

pub struct Logger;

impl Logger {
    pub fn log(level: LogLevel, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        match level {
            LogLevel::Debug => println!("[{}][DEBUG] {}", timestamp, message),
            LogLevel::Info => info(&format!("[{}][INFO] {}", timestamp, message)),
            LogLevel::Warn => warn(&format!("[{}][WARN] {}", timestamp, message)),
            LogLevel::Error => error(&format!("[{}][ERROR] {}", timestamp, message)),
            LogLevel::Success => print_colored(&format!("[{}][SUCCESS] {}", timestamp, message), "green"),
        }
    }

    pub fn debug(message: &str) {
        Logger::log(LogLevel::Debug, message);
    }

    pub fn info(message: &str) {
        Logger::log(LogLevel::Info, message);
    }

    pub fn warn(message: &str) {
        Logger::log(LogLevel::Warn, message);
    }

    pub fn error(message: &str) {
        Logger::log(LogLevel::Error, message);
    }

    pub fn success(message: &str) {
        Logger::log(LogLevel::Success, message);
    }
}
