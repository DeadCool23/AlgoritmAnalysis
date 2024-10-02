mod mes;

use mes::graph::is_show_plots;
use plotly::Plot;
use std::{fs, path::Path};

use mes::{STD_ALGS, STD_ALGS_NAMES, STD_MES_TIME, STD_SIZES};
use mes::{time::get_lev_algs_complete_times, gen, graph::{PLOTS_DIR, plot_graphs, show_plots, plots_to_htmlfile}};

/*
    MARK: Защита
    приает => длина 6 => время затраченное на ДЛ = 1108нс
    6 * 6 * 4 * 6 * 6 * 4 = 20273
    20273 * 1108нс ~ 22мс
 */

fn main() {
    let plt_titles = vec![
        "Замеры времени на одинаковых строках",
        "Замеры времени на разных строках"
    ];

    let strs_algs_times = vec![
        get_lev_algs_complete_times(&STD_SIZES, &STD_ALGS, gen::eq_strs, STD_MES_TIME),
        get_lev_algs_complete_times(&STD_SIZES, &STD_ALGS, gen::neq_strs, STD_MES_TIME)
    ];
    println!("Получены замеры времени");
    let mut plts: Vec<Plot> = Vec::new();

    for (i, plt_title) in plt_titles.iter().enumerate() {
        plts.push(plot_graphs(&plt_title,&STD_ALGS_NAMES, &STD_SIZES, &strs_algs_times[i]));
    }
    println!("Построены графики");

    let plots_dir = &PLOTS_DIR.clone();
    if !Path::new(plots_dir).exists() {
        fs::create_dir(plots_dir).expect(&format!("Не удалось создать директорию {plots_dir}"));
    }

    let plt_filenames = vec![
        format!("{}/eq_strs_plt.html", plots_dir),
        format!("{}/neq_strs_plt.html", plots_dir)
    ];

    let is_print = is_show_plots();

    if is_print { show_plots(&plts) }
    plots_to_htmlfile(&plt_filenames, &plts);

    println!("Графики сохранены в директорию {plots_dir}")
}
