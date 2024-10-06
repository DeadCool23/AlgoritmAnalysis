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

fn get_iterations_cnt(arr: &[i32], els_indexes: &[isize], searcher: SearchAlgorithm) -> Vec<usize> {
    let mut not_el = arr.len() as i32;
    let mut iters = vec![];
    for el_ind in els_indexes.iter() {
        let (_, iter_cnt) = if *el_ind < 0 { 
            not_el -= *el_ind as i32;
            searcher(arr, &not_el)
        } else { 
            searcher(arr, &arr[*el_ind as usize])
        };
        iters.push(iter_cnt);
    }
    iters
}

lazy_static! {
    pub static ref ARR_SIZE: usize = arr_size_calc(MY_INDIVIDUAL_NUMBER);
    static ref STD_ARR: Vec<i32> =  arr::gen_sorted_vec(*ARR_SIZE);

    pub static ref EL_INDEXSES: Vec<isize> = (-1..(*ARR_SIZE as isize)).collect();
    pub static ref ITERATIONS: Vec<Vec<usize>> = vec![
        get_iterations_cnt(&STD_ARR, &EL_INDEXSES, searchers::linear_search),
        get_iterations_cnt(&STD_ARR, &EL_INDEXSES, searchers::binary_search),
        { 
            let mut v = get_iterations_cnt(&STD_ARR, &EL_INDEXSES, searchers::binary_search);
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
