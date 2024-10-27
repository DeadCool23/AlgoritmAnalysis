use std::fs::File;
use std::collections::HashSet;
use std::io::{BufReader, BufWriter, Write, Read};

pub const CACHE_FILENAME: &str = ".cache";

// Чтение из файла кэшированных данных
pub fn load_cache(file_path: &str) -> Option<HashSet<String>> {
    if let Ok(file) = File::open(file_path) {
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content).unwrap();
        let urls: HashSet<String> = serde_json::from_str(&content).unwrap();
        return Some(urls);
    }
    None
}

// Запись кэша в файл
pub fn save_cache(file_path: &str, urls: &HashSet<String>) {
    let file = File::create(file_path).unwrap();
    let mut writer = BufWriter::new(file);
    let content = serde_json::to_string(urls).unwrap();
    writer.write_all(content.as_bytes()).unwrap();
}