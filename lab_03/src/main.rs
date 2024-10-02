mod mes;
mod arr;

use mes::{graph::*, ITERATIONS, SIZES, STD_FILENAMES, STD_GRAPH_NAMES};
use std::{fs, path::Path};

fn main() {
    let plots_dir = &PLOTS_DIR.clone();
    if !Path::new(plots_dir).exists() {
        fs::create_dir(plots_dir).expect(&format!("Не удалось создать директорию {plots_dir}"));
    }
    
    let mut plots = vec![];
    for (i, name) in STD_GRAPH_NAMES.iter().enumerate() {
        plots.push(plot_histogram(&name, &SIZES, &ITERATIONS[i]));
    }

    let is_show = is_show_plot();
    for (i, plot) in plots.iter().enumerate() {
        if is_show {
            show_plot(plot);
        }
        plot_to_htmlfile(&STD_FILENAMES[i], plot);
    }
}
