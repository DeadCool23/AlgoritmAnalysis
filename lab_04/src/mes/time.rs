use std::time::Instant;
use crate::folder::STD_FOLDER_NAME;

use super::super::recipes::paral_reading_recipes;

fn get_time_mes(recipe_urls: &[String], mes_cnt: usize, threads_cnt: usize, recipes_cnt: usize) -> usize {
    let mut mes = 0;
    for _ in 0..mes_cnt {
        let time = Instant::now();

        paral_reading_recipes(STD_FOLDER_NAME, recipes_cnt, recipe_urls, threads_cnt, false);

        let time = time.elapsed().as_nanos();
        mes += time as usize;
    }
    mes /= mes_cnt;
    mes
}

pub fn get_time_mes_results(recipe_urls: &[String], mes_cnt: usize, threds_cnt_vec: &[usize], recipes_cnt_vec: &[usize]) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = vec![];

    for threads_cnt in threds_cnt_vec {
        println!("Getting mesure results for {threads_cnt} threads");
        let mut thread_res: Vec<usize> = vec![];
        for recipes_cnt in recipes_cnt_vec {
            thread_res.push(get_time_mes(recipe_urls, mes_cnt, *threads_cnt, *recipes_cnt));
            println!("\tGetted time for {recipes_cnt} recipes");
        }
        println!("Getted mesure results for {threads_cnt} threads");
        println!();
        res.push(thread_res);
    }

    res
}