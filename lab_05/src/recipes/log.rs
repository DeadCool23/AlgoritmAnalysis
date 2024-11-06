use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::{self, OpenOptions};

use chrono::{DateTime, Timelike, Datelike, Utc};

pub const STD_LOG_FOLDER_NAME: &str = "logs";

fn create_folder_if_not_exists(folder: &str) {
    if !Path::new(folder).exists() {
        fs::create_dir(folder).unwrap();
    }
}

pub fn init_logging() -> (String, String, String, String) {
    create_folder_if_not_exists(STD_LOG_FOLDER_NAME);

    (
        format!("{}/{}", STD_LOG_FOLDER_NAME, "common.log"),
        format!("{}/{}", STD_LOG_FOLDER_NAME, "reader.log"),
        format!("{}/{}", STD_LOG_FOLDER_NAME, "parser.log"),
        format!("{}/{}", STD_LOG_FOLDER_NAME, "writer.log"),
    )
}

pub fn log_message(filename: &str, message: &str, time: Option<DateTime<Utc>>) {
    let time = time.unwrap_or_else(Utc::now);

    let file = OpenOptions::new().create(true).append(true).open(filename).unwrap();
    let mut writer = BufWriter::new(file);
    
    writeln!(writer, "[{:02}.{:02}.{:04} {:02}:{:02}:{:02}.{:03}] {}", 
        time.day(),
        time.month(),
        time.year(),
        time.hour(),
        time.minute(),
        time.second(),
        time.timestamp_subsec_millis(),  // Добавляем миллисекунды
        message
    ).unwrap();
}