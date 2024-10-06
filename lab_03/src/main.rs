mod mes;
mod arr;

use std::{fs, path::Path, vec};
use mes::{graph::*, ITERATIONS, EL_INDEXSES, STD_FILENAMES, STD_GRAPH_NAMES};

fn main() {
    let plots_dir = &PLOTS_DIR.clone();
    if !Path::new(plots_dir).exists() {
        fs::create_dir(plots_dir).expect(&format!("Не удалось создать директорию {plots_dir}"));
    }
    
    let mut plots = vec![];
    for (i, name) in STD_GRAPH_NAMES.iter().enumerate() {
        plots.push(plot_histogram(&name, &EL_INDEXSES, &ITERATIONS[i]));
    }

    let is_show = is_show_plot();
    for (i, plot) in plots.iter().enumerate() {
        if is_show {
            show_plot(plot);
            // plot.show_image(plotly::ImageFormat::SVG, 1300, 500);
        }
        plot_to_htmlfile(&STD_FILENAMES[i], plot);
    }
}
