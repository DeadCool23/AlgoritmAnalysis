use std::{fs, path::Path};

pub const STD_FOLDER_NAME: &str = "recipes_data";

// Создание папки, если её нет
pub fn create_folder_if_not_exists(folder: &str, print_info: bool) {
    if !Path::new(folder).exists() {
        fs::create_dir(folder).unwrap();
        if print_info { println!("Created new dir {}", folder) };
    }
}