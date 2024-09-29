use std::time::Instant;

use super::gen::GenStrFn;
use lab_01::dist_algs::LevenshteinAlgorithm;

pub fn get_lev_algs_complete_times(
    sizes: &[usize],
    algs: &[LevenshteinAlgorithm],
    gen_str_func: GenStrFn,
    mes_cnt: usize,
) -> Vec<Vec<usize>> {
    let mut times = Vec::<Vec<usize>>::new();
    for (i, alg) in algs.iter().enumerate() {
        times.push(Vec::new());
        for size in sizes.iter() {
            let (s1, s2) = gen_str_func(*size);

            let mut avg_time = 0;
            for _ in 0..mes_cnt {
                let time = Instant::now();

                alg(&s1, &s2);

                let time = time.elapsed().as_nanos();
                avg_time += time as usize;
            }
            avg_time /= mes_cnt;

            times[i].push(avg_time);
        }
    }
    times
}
