pub mod core {
    pub mod config;
    pub mod error;
    pub mod macros;
    pub mod utils;
}

pub mod color;
pub mod style;
pub mod spinner;
pub mod progress;
pub mod table;
pub mod logger;
pub mod prompt;

pub use crate::core::config::{print_divider, sleep_ms, clear_screen, Config};
pub use crate::core::error::CliMagicError;
pub use crate::core::utils::capitalize_first;

pub use crate::color::engine::*;
pub use crate::color::palette::*;
pub use crate::color::themes::*;

pub use crate::style::chain::style;

pub use crate::spinner::animations::*;
pub use crate::spinner::thread_manager::Spinner;

pub use crate::progress::bar::ProgressBar;
pub use crate::progress::download::download_file_sim;

pub use crate::table::cell::Cell;
pub use crate::table::renderer::Table;
pub use crate::table::row::Row;

pub use crate::logger::formatter::Logger;
pub use crate::logger::level::LogLevel;

pub use crate::prompt::input::input;
pub use crate::prompt::select::select_option;
pub use crate::prompt::system::Prompt;
pub use crate::prompt::validator::validate_number_range;
