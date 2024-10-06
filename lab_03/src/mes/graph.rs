use std::io::{self, Write};

use lazy_static::lazy_static;
use plotly::{Bar, layout::{Axis, BarMode}, Layout, Plot};

lazy_static! {
    pub static ref PLOTS_DIR: String = "plots".to_string();
}

pub fn plot_histogram(plot_name: &str, x: &[isize], y: &[usize]) -> Plot {
    let mut plot = Plot::new();

    let layout = Layout::new()
        .title(plot_name)
        .x_axis(Axis::new().title("Позиция элемента в массиве"))
        .y_axis(Axis::new().title("Количество итераций"))
        .bar_mode(BarMode::Group);

    plot.set_layout(layout);

    let bar = Bar::new(x.to_vec(), y.to_vec());

    plot.add_trace(bar);

    plot
}

pub fn is_show_plot() -> bool {
    print!("\nПоказать график? [Y/N]: ");
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

pub fn show_plot(plt: &Plot) { plt.show() }

pub fn plot_to_htmlfile(filename: &str, plt: &Plot) {
    let file_path = filename;
    let html_plt = plt.to_html();

    std::fs::write(&file_path, html_plt).expect(&format!("Не удалось сохранить график в файл {}", filename));
}
