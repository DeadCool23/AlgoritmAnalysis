pub mod gen;
pub mod graph;

pub mod time;

use lab_01::dist_algs::*;
use lazy_static::lazy_static;

pub const STD_MES_TIME: usize = 1000;

lazy_static! {
    pub static ref STD_SIZES: Vec<usize> = (1..=8).collect();
    pub static ref STD_ALGS: Vec<LevenshteinAlgorithm> = vec![
        recursion_levenshtein, 
        table_levenshtein, 
        damerau_levenshtein
    ];
    pub static ref STD_ALGS_NAMES: Vec<String> = vec![
        "Левенштейн(р)".to_string(),
        "Левенштейн(т)".to_string(),
        "Дамерау-Левенштейн(т)".to_string()
    ];
}
