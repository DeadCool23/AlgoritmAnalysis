mod cache;

use scraper::{Html, Selector};
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use std::{fs, thread, sync::{Arc, Mutex}, collections::{HashSet, VecDeque}};

use super::folder;
use cache::CACHE_FILENAME;

pub const MAIN_URL: &'static str = "https://www.gastronom.ru";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Recipe {
    pub name: Option<String>,
    pub url: String,
    pub calories: Option<String>,
    pub cook_time: Option<String>,
    pub portions: Option<String>,
    pub protein: Option<String>,
    pub fat: Option<String>,
    pub carbohydrates: Option<String>,
    pub ingredients: Vec<String>,
}

// Получение html по URL
fn fetch_html(url: &str) -> Option<String> {
    let client = Client::new();
    let response = client.get(url).send().ok()?.text().ok()?;
    Some(response)
}

// Получение ссылок на блюда
fn get_dish_urls(catalog_url: &str) -> Option<Vec<String>> {
    let html = fetch_html(catalog_url);
    if html == None { return None };
    let document = Html::parse_document(&html.unwrap());
    let catalog_selector = Selector::parse("a._tag_1jo6w_50").unwrap();
    
    Some(
        document.select(&catalog_selector)
            .map(|el| el.value().attr("href").unwrap().to_string())
            .collect()
    )
}

// Получение ссылок на рецепты с определенного URL
pub fn get_recipe_urls_from_url(url: &str) -> Option<Vec<String>> {
    let html = fetch_html(url);
    if html == None { return None };
    let document = Html::parse_document(&html.unwrap());
    let recipe_selector = Selector::parse("a._name_po7s5_17").unwrap();

    Some(
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
    )
}

// Получение рецептов
// URL рецептов кэшируются в файл .cache
pub fn get_recipe_urls(print_info: bool) -> Option<Vec<String>> {
    let cache_file = CACHE_FILENAME;
    
    if let Some(cached_urls) = cache::load_cache(cache_file) {
        if print_info { println!("Reeading URLs from cache"); }
        return Some(cached_urls.into_iter().collect());
    }

    let catalog_url = "https://www.gastronom.ru/group";
    
    let dish_urls = get_dish_urls(catalog_url);
    if dish_urls == None { return None };
    
    let mut recipe_urls_set: HashSet<String> = HashSet::new();
    
    let mut i = 0;
    for dish_url in dish_urls.unwrap() {
        if print_info { println!("Getting recipes[{i}] from: {}", dish_url); } i += 1;
        let recipes = get_recipe_urls_from_url(&dish_url);
        if recipes == None { return None };
        recipe_urls_set.extend(recipes.unwrap());
    }

    cache::save_cache(cache_file, &recipe_urls_set);
    if print_info { println!("URLs cashed successfully"); }

    Some(recipe_urls_set.into_iter().collect())
}


// Получение рецепта
fn get_recipe(url: &str) -> Option<Recipe> {
    let html = fetch_html(url);
    if html == None { return None };

    let document = Html::parse_document(&html.unwrap());

    // Селекторы
    let name_selector = Selector::parse("h1[itemprop='name']").unwrap();
    let calories_selector = Selector::parse("meta[itemprop='calories']").unwrap();
    let cook_time_selector = Selector::parse("meta[itemprop='cookTime']").unwrap();
    let portions_selector = Selector::parse("div[itemprop='recipeYield'] span").unwrap();
    let bzu_selector = Selector::parse("div[itemscope][itemtype='https://schema.org/NutritionInformation'] meta").unwrap();
    let ingredients_selector = Selector::parse("div[itemprop='recipeIngredient'] span").unwrap();

    // Парсинг данных с использованием Option
    let name = document.select(&name_selector).next().map(|el| el.text().collect::<Vec<_>>().concat());
    let calories = document.select(&calories_selector).next().and_then(|el| el.value().attr("content").map(String::from));
    let cook_time = document.select(&cook_time_selector).next().and_then(|el| el.value().attr("content").map(String::from));
    let portions = document.select(&portions_selector).next().map(|el| el.text().collect::<Vec<_>>().concat());

    // БЖУ
    let mut bzu_iter = document.select(&bzu_selector);
    let protein = bzu_iter.next().and_then(|el| el.value().attr("content").map(String::from));
    let fat = bzu_iter.next().and_then(|el| el.value().attr("content").map(String::from));
    let carbohydrates = bzu_iter.next().and_then(|el| el.value().attr("content").map(String::from));

    // Ингредиенты
    let ingredients = document.select(&ingredients_selector)
        .map(|el| el.text().collect::<Vec<_>>().concat())
        .collect::<Vec<String>>();

    Some(
        Recipe {
            name,
            url: url.to_string(),
            calories,
            cook_time,
            portions,
            protein,
            fat,
            carbohydrates,
            ingredients,
        }
    )
}

// Сохранение рецепта в json
fn save_recipe_to_json(recipe: &Recipe, folder: &str, print_info: bool) {
    let last_path_segment = recipe.url.rsplit('/').next().unwrap();
    let recipe_file = format!("{}/{}.json", folder, last_path_segment);
    let json = serde_json::to_string_pretty(&recipe).unwrap();
    fs::write(&recipe_file, json).unwrap();
    if print_info { println!("Recipe saved to {}", recipe_file); }
}

// Последовательное чтение рецептов
pub fn seq_reading_recipes(dirname: &str, recipes_cnt: usize, recipe_urls: &[String], print_info: bool) {
    folder::create_folder_if_not_exists(dirname, print_info);

    let mut queue = VecDeque::from(recipe_urls.to_vec());
    if print_info { println!("Parsing {} recipes\n", recipes_cnt); }

    let mut parsed_recipes = 0;
    for i in 0..recipes_cnt {
        let recipe_url = queue.pop_front();
        if print_info { 
            match &recipe_url {
                Some(url) => println!("Parsing ricipe[{}] from URL: {}", i + 1, url),
                None => {
                    println!("No more URLs to parse");
                    break;
                },
            }
        }
        parsed_recipes += 1;
        let recipe = get_recipe(&recipe_url.as_ref().unwrap());
        if recipe == None { 
            println!("Error getting recipe from URL {}", &recipe_url.as_ref().unwrap());
            continue;
        }
        save_recipe_to_json(&recipe.unwrap(), dirname, print_info);
        if print_info { println!() }
    }

    if print_info { println!("Parcing {parsed_recipes} recipes completed"); }
}

// Параллельное чтение рецептов
pub fn paral_reading_recipes(dirname: &str, recipes_cnt: usize, recipe_urls: &[String], threads_cnt: usize, print_info: bool) {
    if threads_cnt == 0 {
        return seq_reading_recipes(dirname, recipes_cnt, recipe_urls, print_info);
    }

    folder::create_folder_if_not_exists(dirname, print_info);

    if print_info { println!("Parsing {} recipes with {} threads\n", recipes_cnt, threads_cnt); }

    let queue = Arc::new(Mutex::new(VecDeque::from(recipe_urls.to_vec())));
    let dirname = Arc::new(dirname.to_string());
    let print_info = Arc::new(print_info);
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for thread_id in 0..threads_cnt {
        let queue = Arc::clone(&queue);
        let dirname = Arc::clone(&dirname);
        let print_info = Arc::clone(&print_info);
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            loop {
                let url = {
                    let mut queue = queue.lock().unwrap();
                    queue.pop_front()
                };

                match url {
                    Some(url) => {
                        let mut count = counter.lock().unwrap();
                        if *count >= recipes_cnt {
                            break;
                        }
                        *count += 1;
                        drop(count);  // Освобождаем мьютекс счетчика

                        if *print_info {
                            println!("Thread {}: Parsing recipe from URL: {}", thread_id + 1, url);
                        }
                        let recipe = get_recipe(&url);
                        if recipe == None { 
                            println!("Error getting recipe from URL {}", &url);
                            continue;
                        }
                        save_recipe_to_json(&recipe.unwrap(), &dirname, *print_info);
                    }
                    None => {
                        if *print_info {
                            println!("Thread {}: No more URLs to parse", thread_id + 1);
                        }
                        break;
                    }
                }
            }

            if *print_info {
                println!("Thread {} completed", thread_id + 1);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    if *print_info {
        println!("Parsing {} recipes completed", *counter.lock().unwrap());
    }
}