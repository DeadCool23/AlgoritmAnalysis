use std::time::Instant;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::{HashSet, VecDeque}, sync::{Arc, Mutex}, thread::{self, JoinHandle}};

use chrono::Utc;
use rusqlite::Connection;

use super::Recipe;
use super::database::insert_recipe;
use super::{get_recipe_html, get_recipe_from_html, get_recipe_urls_from_html};

use super::log::log_message;

pub fn create_reader_thread(
    start_timestamp: Instant,
    logfilename: String, 
    reader_times: &Arc<Mutex<Vec<(f64, f64)>>>, 
    urls_queue: &Arc<Mutex<VecDeque<String>>>,
    html_queue: &Arc<Mutex<VecDeque<String>>>,
    stop_threads: &Arc<AtomicBool>
) -> JoinHandle<()> {
    let read_log_file = Arc::new(logfilename);
    let reader_times_clone = Arc::clone(&reader_times);

    let urls_queue_clone = Arc::clone(&urls_queue);
    let html_queue_clone = Arc::clone(&html_queue);
    let stop_threads_clone = Arc::clone(&stop_threads);

    thread::spawn(move || {
        while !stop_threads_clone.load(Ordering::Relaxed) {
            if let Ok(mut urls) = urls_queue_clone.lock() {
                if let Some(url) = urls.pop_front() {
                    let start_time = Instant::now();
                    drop(urls);
                    
                    log_message(&read_log_file, &format!("Чтение URL({})", url), None);
                    
                    let time = Utc::now();
                    if let Some(html) = get_recipe_html(&url) {
                        log_message(&read_log_file, &format!("Загружен HTML для URL({})", url), Some(time));
                        
                        if let Ok(mut html_q) = html_queue_clone.lock() {
                            html_q.push_back(html);
                        }
                    } else {
                        log_message(&read_log_file, &format!("Не удалось считать страницу с URL({})", url), Some(time));
                    }
                    let end_time = Instant::now();
                    reader_times_clone.lock().unwrap().push((
                        start_time.duration_since(start_timestamp).as_secs_f64(),
                        end_time.duration_since(start_timestamp).as_secs_f64()
                    ));
                } else {
                    stop_threads_clone.store(true, Ordering::Relaxed);
                    log_message(&read_log_file, "Очередь URL адресов рецептов пуста", None);
                    println!();
                }
            }
        }
    })
}

pub fn create_parser_thread(
    start_timestamp: Instant,
    logfilename: String, 
    parser_times: &Arc<Mutex<Vec<(f64, f64)>>>, 
    visited: &Arc<Mutex<HashSet<String>>>,
    urls_queue: &Arc<Mutex<VecDeque<String>>>,
    html_queue: &Arc<Mutex<VecDeque<String>>>,
    json_queue: &Arc<Mutex<VecDeque<Recipe>>>,
    stop_threads: &Arc<AtomicBool>
) -> JoinHandle<()> {
    let parse_log_file = Arc::new(logfilename);
    let parser_times_clone = Arc::clone(&parser_times);
    
    let visited_clone = Arc::clone(&visited);
    let urls_queue_clone = Arc::clone(&urls_queue);
    let html_queue_clone = Arc::clone(&html_queue);
    let json_queue_clone = Arc::clone(&json_queue);
    let stop_threads_clone = Arc::clone(&stop_threads);

    thread::spawn(move || {
        while !stop_threads_clone.load(Ordering::Relaxed) {
            if let Ok(mut html_q) = html_queue_clone.lock() {
                if let Some(html) = html_q.pop_front() {
                    let start_time = Instant::now();
                    drop(html_q);

                    let recipe = get_recipe_from_html(&html);
                    log_message(&parse_log_file, &format!("Рецепт успешно получен с URL({})", recipe.url), None);

                    if let Ok(mut visited_set) = visited_clone.lock() {
                        let new_urls = get_recipe_urls_from_html(&html);
                        for new_url in new_urls {
                            if !visited_set.contains(&new_url) {
                                visited_set.insert(new_url.clone());
                                if let Ok(mut urls) = urls_queue_clone.lock() {
                                    urls.push_back(new_url);
                                }
                            }
                        }
                    }

                    if let Ok(mut json_q) = json_queue_clone.lock() {
                        json_q.push_back(recipe);
                    }
                    let end_time = Instant::now();
                    parser_times_clone.lock().unwrap().push((
                        start_time.duration_since(start_timestamp).as_secs_f64(),
                        end_time.duration_since(start_timestamp).as_secs_f64()
                    ));
                }
            }
        }
    })
}

pub fn create_writer_thread(
    start_timestamp: Instant,
    logfilename: String, 

    conn: &Arc<Mutex<Connection>>,
    writer_times: &Arc<Mutex<Vec<(f64, f64)>>>,
    json_queue: &Arc<Mutex<VecDeque<Recipe>>>,
    stop_threads: &Arc<AtomicBool>
) -> JoinHandle<()> {
    let write_log_file = Arc::new(logfilename);
    let writer_times_clone = Arc::clone(&writer_times);
    
    let json_queue_clone = Arc::clone(&json_queue);
    let conn_clone = Arc::clone(&conn);
    let stop_threads_clone = Arc::clone(&stop_threads);

    thread::spawn(move || {
        while !stop_threads_clone.load(Ordering::Relaxed) {
            if let Ok(mut json_q) = json_queue_clone.lock() {
                if let Some(recipe) = json_q.pop_front() {
                    let start_time = Instant::now();
                    drop(json_q);
    
                    let conn = conn_clone.lock().unwrap();
                    insert_recipe(&conn, &recipe);

                    log_message(&write_log_file, &format!("Рецепт c URL({}) успешно добавлен в БД.", &recipe.url), None);
                    let end_time = Instant::now();
                    writer_times_clone.lock().unwrap().push((
                        start_time.duration_since(start_timestamp).as_secs_f64(),
                        end_time.duration_since(start_timestamp).as_secs_f64()
                    ));
                }
            }
        }
    })
}