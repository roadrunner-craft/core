use log::{Level, Log, Metadata, Record};
use std::io::{stdout, Stdout, Write};
use std::sync::Mutex;

use colored::*;

use crate::utils::time::time_since_launched;

pub struct LogStdOut {
    stdout: Mutex<Stdout>,
    level: Level,
}

impl LogStdOut {
    pub fn new(level: Level) -> Self {
        Self {
            stdout: Mutex::new(stdout()),
            level,
        }
    }
}

impl Log for LogStdOut {
    fn enabled(&self, record: &Metadata<'_>) -> bool {
        record.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        let mut lock = self.stdout.lock().unwrap();
        let lvl = record.level().to_string();
        let line = format!(
            "[{}] - ({}): {}\n",
            time_since_launched().cyan(),
            match record.level() {
                log::Level::Error => lvl.red(),
                log::Level::Warn => lvl.yellow(),
                log::Level::Info => lvl.green(),
                _ => lvl.white(),
            },
            record.args()
        );
        lock.write(line.as_bytes()).unwrap();
    }

    fn flush(&self) {
        let mut lock = self.stdout.lock().unwrap();
        lock.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_enabled_info() {
        let logger = LogStdOut::new(log::Level::Info);
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Error).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Warn).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Info).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Debug).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Trace).build()));
    }

    #[test]
    fn log_enabled_warn() {
        let logger = LogStdOut::new(log::Level::Warn);
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Error).build()));
        assert!(logger.enabled(&log::Metadata::builder().level(log::Level::Warn).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Info).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Debug).build()));
        assert!(!logger.enabled(&log::Metadata::builder().level(log::Level::Trace).build()));
    }
}
