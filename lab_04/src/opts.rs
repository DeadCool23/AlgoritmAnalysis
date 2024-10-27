use std::io::{self, Write};

use crate::mes::plot::{is_show_plots, plot_graphs, plot_to_htmlfile, show_plot, STD_PLOTS_DIR, STD_PLOT_TITLE};
use crate::mes::{RECIPES_CNT_VEC, STD_MES_CNT, THREADS_CNT_VEC};

use super::folder::STD_FOLDER_NAME;
use crate::folder::create_folder_if_not_exists;

use crate::recipes::MAIN_URL;
use super::recipes::{get_recipe_urls, paral_reading_recipes, seq_reading_recipes};

use super::mes::time::get_time_mes_results;

enum MODE {
    PARALLEL,
    SEQUENTIAL,
}

pub fn get_help() {
    println!("Usage:");
    println!("  -i   Run in interactive mode");
    println!("  -m   Run measures");
    println!("  -h   Display this help message");
}

fn is_print_info() -> bool {
    print!("\nПоказать отладочную информацию? [Y/N]: ");
    io::stdout().flush().unwrap();

    let mut is_print = String::new();
    io::stdin().read_line(&mut is_print).expect("Неверный ввод");

    let is_print = is_print.trim().chars().next();

    let is_print = match is_print {
        Some(c) if c == 'Y' || c == 'y' => true,
        _ => false,
    };

    is_print
}

pub fn interactive_mode() {
    let recipe_urls = get_recipe_urls(false);
    if recipe_urls == None {
        println!("Ошибка загрузки рецептов");
        return;
    }

    println!("=== ПРОГРАММА ПОЛУЧАЕТ РЕЦЕПТЫ С САЙТА {} ===\n", MAIN_URL);
    print!("Введите кол-во рецептов которое хотите считать: ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Неверный ввод");

    let recipes_cnt = match buf.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Неверный ввод числа рецептов");
            return;
        }
    };

    println!("Введите режим считывания рецептов:");
    println!("S - Последовательный режим");
    println!("P - Параллельный режим");
    print!(": ");
    io::stdout().flush().unwrap();

    buf = String::new();
    io::stdin().read_line(&mut buf).expect("Неверный ввод");
    let read_mode_char = buf.trim().chars().next();
    
    let read_mode: MODE;
    match read_mode_char {
        Some(c) => {
            if c == 'S' || c == 's' {
                read_mode = MODE::SEQUENTIAL;
            } else if c == 'P' || c == 'p' {
                read_mode = MODE::PARALLEL;
            } else {
                println!("Не существует такого режима считывания рецептов");
                return;
            }
        },
        None => {
            println!("Неверный ввод");
            return;
        }
    };

    match read_mode {
        MODE::PARALLEL => {
            print!("Введите кол-во потоков: ");
            io::stdout().flush().unwrap();

            buf = String::new();
            io::stdin().read_line(&mut buf).expect("Неверный ввод");

            let threads_cnt = match buf.trim().parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Неверный ввод числа потоков");
                    return;
                }
            };

            paral_reading_recipes(STD_FOLDER_NAME, recipes_cnt, &recipe_urls.unwrap(), threads_cnt, is_print_info());
        },
        MODE::SEQUENTIAL => {
            seq_reading_recipes(STD_FOLDER_NAME, recipes_cnt, &recipe_urls.unwrap(), is_print_info());
        }
    }
}

pub fn measure_mode() {
    let recipe_urls = get_recipe_urls(false);
    if recipe_urls == None {
        println!("Ошибка загрузки рецептов");
        return;
    }
    
    let res = get_time_mes_results(&recipe_urls.unwrap(), STD_MES_CNT, &THREADS_CNT_VEC, &RECIPES_CNT_VEC);
    
    let plot = plot_graphs(STD_PLOT_TITLE, &THREADS_CNT_VEC, &RECIPES_CNT_VEC, &res);

    create_folder_if_not_exists(STD_PLOTS_DIR, false);
    plot_to_htmlfile(&format!("{STD_PLOTS_DIR}/plot.html"), &plot);

    let is_show = is_show_plots();
    if is_show { show_plot(&plot) };
}