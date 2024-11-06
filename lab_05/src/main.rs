mod mes;
mod recipes;

use recipes::{log::{init_logging, log_message}, write_recipes_to_db};
use mes::{plot::{is_show_plots, plot_timing_data, plot_to_htmlfile, show_plot, STD_PLOTNAME}, write_timedata_to_csv, STD_MES_FILENAME, STD_TIMEDATA_SIZES};

fn main() {
    let time_data = write_recipes_to_db();
    if time_data.is_none() { return; }

    let time_data = time_data.unwrap();
    let plot_time_data = time_data.cut(STD_TIMEDATA_SIZES.0, STD_TIMEDATA_SIZES.1, STD_TIMEDATA_SIZES.2);
    
    if let Some(td) = plot_time_data {
        let plot = plot_timing_data(&td);
    
        let is_show = is_show_plots();
        if is_show { show_plot(&plot); }
        plot_to_htmlfile(&STD_PLOTNAME, &plot);
    } else {
        println!("\nINFO: Недостаточно данных для построения графика")
    }

    write_timedata_to_csv(&STD_MES_FILENAME, &time_data);
    let common_log = init_logging().0;
    log_message(&common_log, &format!("Среднее время существования задачи {} с.", time_data.get_process_avg_time()), None);
    log_message(&common_log, &format!("Данные о времени работы потоков загружены в {}", STD_MES_FILENAME), None);
}
