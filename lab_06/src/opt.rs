
use std::io::{self, Write};

use lazy_static::lazy_static;
use crate::folder::create_folder_if_not_exists;
use crate::parametrization::auto_parametrization;
use super::cities::{AncientWorld, example_countries::*};
use super::mes::{plot::*, get_mes, MES_CNT, STD_MES_SIZES};

pub fn get_help() {
    println!("Welcome to the lab_06!");
    println!("Usage: lab_06 [command]");
    println!("Available commands:");
    println!("  -h: Display this help message");
    println!("  -i: Enter interactive mode");
    println!("  -m: Get comprehensions between brute force and ant algorithm");
    println!("  -a: Automatically tune parameters of the ant algorithm");
}

lazy_static! {
    static ref countries_names: Vec<String> = vec!["Ханаан".to_string(), "Мясопотамия".to_string(), "Древний Рим".to_string()];
    static ref countries: Vec<&'static AncientWorld> = MAIN_COUNTRIES.to_vec();
}

fn is_print_ancient_world() -> bool {
    print!("\nПоказать данные страны? [Y/N]: ");
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
    println!("Выберете страну:");
    for (i, ancient_world_name) in countries_names.iter().enumerate() {
        println!("{} - {}", i + 1, ancient_world_name);
    }
    print!(": ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Неверный ввод");

    let ancient_world_num = match buf.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Неверный ввод номера страны");
            return;
        }
    };

    if ancient_world_num <= 0 || ancient_world_num > countries.len() {
        println!("Страна с таким номером не найдена");
        return;
    }

    let ancient_world = countries[ancient_world_num - 1];
    println!("Выбрана страна: {}", countries_names[ancient_world_num - 1]);
    let print_ancient_world = is_print_ancient_world();
    if print_ancient_world { println!("{:?}", ancient_world) }

    println!("Выберете алгоритм решения задачи коммивояжера:");
    println!("1 - Алгоритм полного перебора");
    println!("2 - Муравьиный алгоритм");
    print!(": ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Неверный ввод");

    let alg_num = match buf.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Неверный ввод номера алгоритма");
            return;
        }
    };

    let data: (usize, Vec<String>);
    match alg_num {
        1 => {
            data = ancient_world.solve_tsp_by_brute_force();
        }
        2 => {
            match input_ant_colony_data() {
                Some((alpha, beta, evaporation, days, ants, elite_ants)) => {
                    data = ancient_world.solve_tsp_by_ant_colony(
                        alpha, 
                        beta, 
                        evaporation, 
                        ants,
                        days, 
                        elite_ants
                    );
                },
                None => { return; }
            }
        }
        _ => {
            println!("Неверный ввод номера алгоритма");
            return;
        }
    }

    println!("\nДлина кратчайшего пути: {}\nКратчайший путь: {:?}\n", data.0, data.1);
}

fn input_value<T: std::str::FromStr + PartialOrd + Default>(prompt: &str, is_greater_zero: bool) -> Option<T> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Неверный ввод");

    match buf.trim().parse::<T>() {
        Ok(value) => {
            if is_greater_zero && value <= T::default() {
                println!("Значение должно быть больше нуля");
                return None;
            }
            Some(value)
        }
        Err(_) => {
            println!("Неверный ввод данных");
            None
        }
    }
}

fn input_ant_colony_data() -> Option<(f64, f64, f64, usize, usize, usize)> {
    let alpha = input_value::<f64>("Введите коэффициент alpha: ", true)?;
    let beta = input_value::<f64>("Введите коэффициент beta: ", true)?;
    let evaporation = input_value::<f64>("Введите коэффициент evaporation: ", true)?;
    let days = input_value::<usize>("Введите количество дней: ", true)?;
    let ants = input_value::<usize>("Введите количество муравьев: ", true)?;
    let elite_ants = input_value::<usize>("Введите количество элитных муравьев: ", false)?;

    Some((alpha, beta, evaporation, days, ants, elite_ants))
}

pub fn measure_mode() {
    let data = get_mes(&STD_MES_SIZES, MES_CNT);
    let plt = plot_graphs("", &STD_ALGS_NAMES, &STD_MES_SIZES, &data);

    let is_show = is_show_plots();

    if is_show { show_plot(&plt); }
    create_folder_if_not_exists(&PLOT_DIR);

    plot_to_htmlfile(&format!("{}/plot.html", PLOT_DIR.as_str()), &plt);
}

const STD_COUNTRY_SIZE_FOR_PARAMETRIZATION: usize = 10;

pub fn ant_algorithm_automatic_parametrization() {
    let err = auto_parametrization(&[
        AncientWorld::gen_rand_ancient_world(STD_COUNTRY_SIZE_FOR_PARAMETRIZATION, 5),
        AncientWorld::gen_rand_ancient_world(STD_COUNTRY_SIZE_FOR_PARAMETRIZATION, 50),
        AncientWorld::gen_rand_ancient_world(STD_COUNTRY_SIZE_FOR_PARAMETRIZATION, 100)
    ], &format!("{}/parametrization.tex", PLOT_DIR.as_str()));
    if let Err(_) = err { return; }
}