pub mod log;
mod database;

use log::*;
use database::*;

use rusqlite::Connection;
use scraper::{Html, Selector};
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use threads::{create_parser_thread, create_reader_thread, create_writer_thread};

use std::time::Instant;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread, sync::{Arc, Mutex}, collections::{HashSet, VecDeque}};

use crate::mes::TimeData;

pub const MAIN_URL: &str = "https://www.gastronom.ru";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Recipe {
    pub name: String,
    pub url: String,
    pub image_url: Option<String>,
    pub calories: Option<String>,
    pub cook_time: Option<String>,
    pub portions: Option<String>,
    pub protein: Option<String>,
    pub fat: Option<String>,
    pub carbohydrates: Option<String>,
    pub ingredients: Vec<String>,
}

fn fetch_html(url: &str) -> Option<String> {
    let client = Client::new();
    let response = client.get(url).send().ok()?.text().ok()?;
    Some(response)
}

fn get_recipe_urls_from_html(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let recipe_selector = Selector::parse("a._link_iku8o_14").unwrap(); // Это поменялось на самом сайте

    document.select(&recipe_selector)
        .filter_map(|el| {
            let href = el.value().attr("href")?;
            if href.starts_with("/recipe") {
                Some(format!("https://www.gastronom.ru{}", href))
            } else {
                None
            }
        })
        .collect()
}

fn get_recipe_urls_from_url(url: &str) -> Option<Vec<String>> {
    let html = fetch_html(url);
    if html == None { return None };
    
    Some(get_recipe_urls_from_html(&html.unwrap()))
}

fn get_recipe_html(url: &str) -> Option<String> {
    if url.contains("/recipe") {
        fetch_html(url)
    } else {
        None
    }
}

fn get_recipe_from_html(html: &str) -> Recipe {
    let document = Html::parse_document(html);

    let name_selector = Selector::parse("h1[itemprop='name']").unwrap();
    let url_selector = Selector::parse("link[rel='canonical']").unwrap();
    let image_selector = Selector::parse("img[itemprop='image']").unwrap();
    let calories_selector = Selector::parse("meta[itemprop='calories']").unwrap();
    let cook_time_selector = Selector::parse("meta[itemprop='cookTime']").unwrap();
    let portions_selector = Selector::parse("div[itemprop='recipeYield'] span").unwrap();
    let bzu_selector = Selector::parse("div[itemscope][itemtype='https://schema.org/NutritionInformation'] meta").unwrap();
    let ingredients_selector = Selector::parse("div[itemprop='recipeIngredient'] span").unwrap();

    let name = document.select(&name_selector).next().map(|el| el.text().collect::<Vec<_>>().concat()).unwrap();
    let url = document.select(&url_selector).next().and_then(|el| el.value().attr("href").map(String::from)).unwrap();
    let image_url = document.select(&image_selector).next().and_then(|el| el.value().attr("src").map(String::from));
    let calories = document.select(&calories_selector).next().and_then(|el| el.value().attr("content").map(String::from));
    let cook_time = document.select(&cook_time_selector).next().and_then(|el| el.value().attr("content").map(String::from));
    let portions = document.select(&portions_selector).next().map(|el| el.text().collect::<Vec<_>>().concat());

    let mut bzu_iter = document.select(&bzu_selector);
    let protein = bzu_iter.next().and_then(|el| el.value().attr("content").map(String::from));
    let fat = bzu_iter.next().and_then(|el| el.value().attr("content").map(String::from));
    let carbohydrates = bzu_iter.next().and_then(|el| el.value().attr("content").map(String::from));

    let ingredients = document.select(&ingredients_selector)
        .map(|el| el.text().collect::<Vec<_>>().concat())
        .collect::<Vec<String>>();

    Recipe {
        name,
        url,
        image_url,
        calories,
        cook_time,
        portions,
        protein,
        fat,
        carbohydrates,
        ingredients,
    }
}

#[allow(dead_code)]
fn get_recipe(url: &str) -> Option<Recipe> {
    match get_recipe_html(url) {
        None => None,
        Some(html) => Some(get_recipe_from_html(&html))
    }
}

mod threads;
pub fn write_recipes_to_db() -> Option<TimeData> {
    let log_files = init_logging();

    let conn = Connection::open(DATABASE_FILENAME).unwrap();
    create_recipe_table(&conn);

    let start_timestamp = Instant::now();

    let reader_times = Arc::new(Mutex::new(Vec::new()));
    let parser_times = Arc::new(Mutex::new(Vec::new()));
    let writer_times = Arc::new(Mutex::new(Vec::new()));

    let start_urls = get_recipe_urls_from_url(MAIN_URL);
    if start_urls.is_none() { 
        println!("ERROR: Не удалось получить начальные URLs");
        return None;
    }

    let urls_queue = Arc::new(Mutex::new(VecDeque::from(start_urls.unwrap())));
    let html_queue = Arc::new(Mutex::new(VecDeque::<String>::new()));
    let json_queue = Arc::new(Mutex::new(VecDeque::<Recipe>::new()));
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let conn = Arc::new(Mutex::new(conn));

    let stop_threads = Arc::new(AtomicBool::new(false));

    // ПОТОК ОСТАНОВКИ =======================================================================================
    print!("Нажмите Enter для завершения программы");
    io::stdout().flush().unwrap();

    let common_log_file = Arc::new(log_files.0.clone());
    let stop_threads_clone = Arc::clone(&stop_threads);
    thread::spawn(move || {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            stop_threads_clone.store(true, Ordering::Relaxed);
            log_message(&common_log_file, "Поток остановщик подал сигнал завершения", None);
        }
    });

    log_message(&log_files.0, "Поток остановщик создан", None);

    // ПОТОК ЧТЕНИЯ ==========================================================================================
    let reader_handle = create_reader_thread(
        start_timestamp, 
        log_files.1, 
        &reader_times, 
        &urls_queue, 
        &html_queue, 
        &stop_threads
    );

    log_message(&log_files.0, "Поток читатель создан", None);
    
    // ПОТОК ПАРСИНГА =========================================================================================
    let parser_handle = create_parser_thread(
        start_timestamp, 
        log_files.2, 
        &parser_times, 
        &visited, 
        &urls_queue, 
        &html_queue, 
        &json_queue, 
        &stop_threads
    );

    log_message(&log_files.0, "Поток парсер создан", None);
    
    // ПОТОК ЗАПИСИ ==========================================================================================
    let writer_handle = create_writer_thread(
        start_timestamp, 
        log_files.3, 
        &conn, 
        &writer_times, 
        &json_queue, 
        &stop_threads
    );

    log_message(&log_files.0, "Поток писатель создан", None);
    
    reader_handle.join().unwrap();
    log_message(&log_files.0, "Поток читатель завершился", None);
    parser_handle.join().unwrap();
    log_message(&log_files.0, "Поток парсер завершился", None);
    writer_handle.join().unwrap();
    log_message(&log_files.0, "Поток писатель завершился", None);
    
    if urls_queue.lock().unwrap().is_empty() {
        log_message(&log_files.0, "Поток остановщик завершился", None);
    }

    println!("Программа успешно завершена");
    
    let (r, p, w) = (
        reader_times.lock().unwrap().clone(),
        parser_times.lock().unwrap().clone(),
        writer_times.lock().unwrap().clone()
    );
    
    Some(TimeData::new(&r, &p, &w))
}
