use std::{fs, path::Path};

pub fn create_folder_if_not_exists(folder: &str) {
    if !Path::new(folder).exists() {
        fs::create_dir(folder).unwrap();
    }
}