use log::{info, warn, error, debug};
use std::fs::OpenOptions;
use std::io::{Write, Result};
use chrono::{Utc, Timelike};


pub fn init_logger() {
    let log_file = "lyzerai.log";


    let log_level = log::LevelFilter::Info;
    let log_format = |record: &log::Record| {
        let timestamp = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        format!("{} - [{}] - {}: {}", timestamp, record.level(), record.target(), record.args())
    };

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .expect("Cannot open log file");

    let log_writer = std::sync::Mutex::new(log_file);


    log::set_boxed_logger(Box::new(FileLogger::new(log_writer)))
        .map(|()| log::set_max_level(log_level))
        .expect("Failed to initialize logger");


    let stdout_logger = simplelog::TermLogger::new(
        log_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    ).expect("Failed to initialize stdout logger");


    log::set_logger(&stdout_logger)
        .map(|()| log::set_max_level(log_level))
        .expect("Failed to set logger");
}


pub struct FileLogger {
    file: std::sync::Mutex<std::fs::File>,
}

impl FileLogger {
    pub fn new(file: std::sync::Mutex<std::fs::File>) -> Self {
        FileLogger { file }
    }
}


impl log::Log for FileLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        log::LevelFilter::Info <= metadata.level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut file = self.file.lock().unwrap();
            let log_message = format!(
                "{} - [{}] - {}: {}\n",
                Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
                record.level(),
                record.target(),
                record.args()
            );
            if let Err(e) = file.write_all(log_message.as_bytes()) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
    }

    fn flush(&self) {}
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_warning(message: &str) {
    warn!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}
