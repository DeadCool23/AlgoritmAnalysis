use std::io::{self, Write};

use lazy_static::lazy_static;
use plotly::{common::Mode, layout::Axis, Layout, Plot, Scatter};

lazy_static! {
    pub static ref PLOT_DIR: String = "mes".to_string();
    pub static ref STD_ALGS_NAMES: Vec<String> = vec!["Полный перебор".to_string(), "Муравьиный алгоритм".to_string()];
}

pub fn plot_graphs(plot_name: &str, graph_names: &[String], x: &[usize], ys: &[Vec<usize>]) -> Plot {
    let mut plot = Plot::new();

    let layout = Layout::new()
        .title(plot_name)
        .x_axis(Axis::new().title("Кол-во вершин графа"))
        .y_axis(Axis::new().title("Время выполнения(нс)"));

    plot.set_layout(layout);

    for (i, y) in ys.iter().enumerate() {
        let scatter = Scatter::new(x.to_vec(), y.to_vec())
            .mode(Mode::LinesMarkers)
            .name(&graph_names[i]);

        plot.add_trace(scatter);
    }

    plot
}

pub fn is_show_plots() -> bool {
    print!("\nПоказать графики? [Y/N]: ");
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
