use log::{Level, Log, Metadata, Record};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::utils::time::{ms_since_epoch, PROGRAM_START};

const LOG_DIR: &str = "logs/";
const CURRENT_LOG: &str = "current.log";
const MAX_LOG_SIZE: u64 = 5 * 1024 * 1024; // 5MB
const LOGS_TO_KEEP: usize = 10;

pub struct LogFile {
    level: Level,
}

impl LogFile {
    pub fn new(level: Level) -> Self {
        Self { level }
    }
}

/// path to the current log file
fn log_path() -> PathBuf {
    Path::new(LOG_DIR).join(CURRENT_LOG)
}

/// handle to the current log file, creating it if necessary
fn log_file() -> fs::File {
    let create_file = |_| {
        let _ = fs::create_dir_all(Path::new(LOG_DIR));
        fs::File::create(log_path())
    };
    fs::OpenOptions::new()
        .append(true)
        .open(log_path())
        .or_else(create_file)
        .unwrap()
}

/// save current log but only keep n most recent files
fn rotate_logs() {
    // backup current log by timestamping it
    let new_file_path = Path::new(LOG_DIR).join(format!("{}.log", ms_since_epoch()));
    let _ = fs::rename(log_path(), new_file_path);

    // remove oldest log file
    let count = fs::read_dir(LOG_DIR).map(|dir| dir.count()).ok();
    if count > Some(LOGS_TO_KEEP) {
        let _ = fs::read_dir(LOG_DIR).map(|dir_entry: fs::ReadDir| {
            // remove first file in lexicographical order (oldest for timestamped files)
            dir_entry
                .filter_map(|entry| entry.ok())
                .flat_map(|entry: fs::DirEntry| {
                    entry
                        .path()
                        .file_stem()
                        .and_then(|s| s.to_str().map(|s| s.to_string()))
                }) // iterator over file stems, as String (representing timestamps)
                .min() // gets the first, so the oldest
                .and_then(|stem: String| {
                    fs::remove_file(Path::new(LOG_DIR).join(format!("{}.log", stem))).ok()
                })
        });
    }
}

/// rotating file implementation for log
impl Log for LogFile {
    fn enabled(&self, record: &Metadata<'_>) -> bool {
        record.level() <= self.level
    }

    fn log(&self, record: &Record<'_>) {
        let line = format!(
            "[{:010}] - ({}): {}\n",
            PROGRAM_START.elapsed().as_millis(),
            record.level(),
            record.args()
        );
        let mut file = log_file();
        let _ = file.write(line.as_bytes());
        let n = file.metadata().map(|metadata| metadata.len()).ok();
        if n >= Some(MAX_LOG_SIZE) {
            rotate_logs();
        }
    }

    fn flush(&self) {}
}