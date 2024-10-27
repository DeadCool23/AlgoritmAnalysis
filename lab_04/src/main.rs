mod mes;
mod opts;
mod folder;
mod recipes;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.contains(&String::from("-h")) {
        opts::get_help();
        return;
    }

    if args.contains(&String::from("-i")) && args.contains(&String::from("-m")) {
        println!("Выберите один режим работы")
    } else if args.contains(&String::from("-i")) {
        opts::interactive_mode();
    } else if args.contains(&String::from("-m")) {
        opts::measure_mode();
    } else if args.contains(&String::from("-t")) {
        opts::print_tex_table();
    } else {
        println!("Incorrect option\ncheck -h")
    }
}