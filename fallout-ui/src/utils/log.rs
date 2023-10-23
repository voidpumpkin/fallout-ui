//! The log service

use std::fmt::Write;

use log::set_logger;
use log::set_max_level;
use log::Level;
use log::LevelFilter;
use log::Log;
use log::Metadata;
use log::Record;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(a: &str, b: &str, c: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(a: &str, b: &str, c: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str, b: &str, c: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn debug(a: &str, b: &str, c: &str);

}

/// The public static logger instance
static LOGGER: LogService = LogService;

/// Initialize the static logger
pub fn init_logger() {
    set_logger(&LOGGER)
        .map(|()| set_max_level(LevelFilter::Debug))
        .expect("Couldn't set logger")
}

/// The service used for logging purposes
struct LogService;

impl Log for LogService {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        // Verify that the logger is enabled
        if self.enabled(record.metadata()) {
            // Create the log entry
            let mut log_entry = format!("%c{}: %c", record.level());

            // Add file and line if available
            if let (Some(file), Some(line)) = (record.file(), record.line()) {
                write!(log_entry, "{file}:{line}: ").unwrap();
            }

            // Add the body
            write!(log_entry, "{}", record.args()).unwrap();

            // Log the entry
            const BOLD: &str = "font-weight: bold";
            const NORMAL: &str = "font-weight: normal";
            match record.level() {
                Level::Error => {
                    error(&log_entry, BOLD, NORMAL);
                }
                Level::Warn => {
                    warn(&log_entry, BOLD, NORMAL);
                }
                Level::Info => {
                    log(&log_entry, BOLD, NORMAL);
                }
                Level::Debug => {
                    debug(&log_entry, BOLD, NORMAL);
                }
                Level::Trace => {
                    debug(&log_entry, BOLD, NORMAL);
                }
            }
        }
    }

    fn flush(&self) {}
}
