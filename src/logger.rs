use crate::constants;
use chrono::Local;
use directories::ProjectDirs;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    sync::{Mutex, OnceLock},
};

pub struct Logger {
    file: Mutex<File>,
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

impl Logger {
    fn init() -> Self {
        let proj_dirs = ProjectDirs::from(
            constants::QUALIFIER,
            constants::ORGANIZATION,
            constants::APP_NAME,
        )
        .expect("Failed to get application directory");

        let log_dir = proj_dirs.cache_dir().join("logs");

        fs::create_dir_all(&log_dir).expect("Failed to create log directory");

        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let path = log_dir.join(format!("{timestamp}.log"));

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("Failed to create log file");

        Self {
            file: Mutex::new(file),
        }
    }

    fn instance() -> &'static Logger {
        LOGGER.get_or_init(|| Logger::init())
    }

    fn write(level: &str, message: &str) {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let line = format!("{} [{}] {}\n", now, level, message);

        print!("{}", line);

        let logger = Logger::instance();

        let mut file = logger.file.lock().unwrap();

        file.write_all(line.as_bytes()).unwrap();
    }

    pub fn info(message: &str) {
        Self::write("INFO", message);
    }

    pub fn warn(message: &str) {
        Self::write("WARN", message);
    }

    pub fn error(message: &str) {
        Self::write("ERROR", message);
    }

    pub fn debug(message: &str) {
        Self::write("DEBUG", message);
    }
}
