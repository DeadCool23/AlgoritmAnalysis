mod mes;
mod opt;
mod cities;
mod folder;
mod parametrization;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.contains(&String::from("-h")) {
        opt::get_help();
        return;
    }

    if args.contains(&String::from("-i")) && args.contains(&String::from("-m")) {
        println!("Выберите один режим работы")
    } else if args.contains(&String::from("-i")) {
        opt::interactive_mode();
    } else if args.contains(&String::from("-m")) {
        opt::measure_mode();
    } else if args.contains(&String::from("-a")) {
        opt::ant_algorithm_automatic_parametrization();
    } else {
        println!("Incorrect option\ncheck -h")
    }
}