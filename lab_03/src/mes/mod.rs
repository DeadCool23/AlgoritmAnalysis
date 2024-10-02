pub mod graph;

use graph::PLOTS_DIR;
use lazy_static::lazy_static;
use super::arr::{self, searchers};

const MY_INDIVIDUAL_NUMBER: usize = 8104;

fn arr_size_calc(x: usize) -> usize {
    x / 8 + match (x >> 2) % 10 == 0 { 
        true => x % 1000,
        false => (x >> 2) % 10 * (x % 10) + (x >> 1) % 10
    }
}

type SearchAlgorithm = fn(&[i32], &i32) -> (Option<usize>, usize);

fn get_iterations_cnt(arr: &[i32], searcher: SearchAlgorithm) -> Vec<usize> {
    let mut iters = vec![];
    for el in arr.iter() {
        let (_, iter_cnt) = searcher(arr, el);
        iters.push(iter_cnt);
    }
    iters
}

lazy_static! {
    static ref VEC_SIZE: usize = arr_size_calc(MY_INDIVIDUAL_NUMBER);
    static ref STD_ARR: Vec<i32> =  arr::gen_sorted_vec(*VEC_SIZE);

    pub static ref SIZES: Vec<usize> = (1..=*VEC_SIZE).collect();
    pub static ref ITERATIONS: Vec<Vec<usize>> = vec![
        get_iterations_cnt(&STD_ARR, searchers::linear_search),
        get_iterations_cnt(&STD_ARR, searchers::binary_search),
        { 
            let mut v = get_iterations_cnt(&STD_ARR, searchers::binary_search);
            v.sort();
            v
        }
    ];

    pub static ref STD_GRAPH_NAMES: Vec<String> = vec![
        "Линейный поиск".to_string(),
        "Бинарный поиск".to_string(),
        "Отсортированный бинарный поиск".to_string()
    ];

    pub static ref STD_FILENAMES: Vec<String> = vec![
        format!("{}/linear.html", *PLOTS_DIR),
        format!("{}/binary.html", *PLOTS_DIR),
        format!("{}/sort_binary.html", *PLOTS_DIR),
    ];
}
