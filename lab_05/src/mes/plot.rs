use plotly::Plot;
use plotly::Scatter;
use plotly::layout::{Axis, Layout};
use plotly::common::{Fill, Mode, Line};

use super::TimeData;

use std::io::{self, Write};

pub const STD_PLOTNAME: &'static str = "plot.html";

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

pub fn plot_timing_data(timing_data: &TimeData) -> Plot {
    let mut plot = Plot::new();

    let y_ranges = [
        ("reading", 2.0, 3.0, "darkcyan"), 
        ("parsing", 1.0, 2.0, "goldenrod"),
        ("writing", 0.0, 1.0, "deeppink")
    ];

    for (activity, y_min, y_max, color) in y_ranges.iter() {
        let times = match *activity {
            "reading" => &timing_data.reader_times,
            "parsing" => &timing_data.parser_times,
            "writing" => &timing_data.writer_times,
            _ => continue,
        };

        for (start, end) in times {
            plot.add_trace(Scatter::new(vec![*start, *start, *end, *end, *start], vec![*y_min, *y_max, *y_max, *y_min, *y_min])
                .mode(Mode::Lines)
                .line(Line::new().color("darkgray"))
                .fill(Fill::ToSelf)
                .fill_color(*color)
            );
        }
    }

    let layout = Layout::new()
        .y_axis(
            Axis::new()
                .tick_values(vec![0.5, 1.5, 2.5])
                .tick_text(vec!["reading", "parsing", "writing"])
        )
        .x_axis(
            Axis::new().title("Время(секунды)")
        )
        .show_legend(false);

    plot.set_layout(layout);

    plot
}

pub fn show_plot(plt: &Plot) { plt.show() }

pub fn plot_to_htmlfile(filename: &str, plt: &Plot) {
    let file_path = filename;
    let html_plt = plt.to_html();

    std::fs::write(&file_path, html_plt).expect(&format!("Не удалось сохранить график в файл {}", filename));
}