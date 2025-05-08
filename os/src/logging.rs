//! Global logger

use log::{Level, LevelFilter, Log, Metadata, Record};

/// a simple logger
struct SimpleLogger;
use crate::console::print_color;


 
impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };
        print_color(
            format_args!("[{:>5}] {}\n", record.level(), record.args()),
            color
        );
    }
    fn flush(&self) {}
}

/// initiate logger
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    if log::set_logger(&LOGGER).is_ok() {
        let level = match option_env!("LOG") {
            Some("ERROR") => LevelFilter::Error,
            Some("WARN") => LevelFilter::Warn,
            Some("INFO") => LevelFilter::Info,
            Some("DEBUG") => LevelFilter::Debug,
            Some("TRACE") => LevelFilter::Trace,
            _ => LevelFilter::Off,
        };
        log::set_max_level(level);
        println!("[logger] Initialized with level {:?}", level);
    } else {
        println!("[logger] Already initialized");
    }
}
