pub mod plot;

use std::time::Instant;
use lazy_static::lazy_static;

use lab_06::cities::AncientWorld;

const ALGS_CNT: usize = 2;
pub const MES_CNT: usize = 10;

lazy_static! {
    pub static ref STD_MES_SIZES: Vec<usize> = (1..=9).collect();
}

pub fn get_mes(mes_sizes: &[usize], mes_cnt: usize) -> Vec<Vec<usize>> {
    let mut mes_data = Vec::new();
    for alg in 0..ALGS_CNT {
        if alg == 0 {
            println!("Getting mes for brute force algorithm");
        } else {
            println!("Getting mes for ant colony algorithm");
        }
        let mut alg_mes_data = Vec::new();

        for mes_size in mes_sizes {
            let ancient_world = &AncientWorld::gen_rand_ancient_world(*mes_size, 10_000);

            println!("\tGetting mes for {mes_size} cities");
            
            let mut sum = 0;
            for _ in 0..mes_cnt {
                let time = Instant::now();
                if alg == 0 {
                    ancient_world.solve_tsp_by_brute_force();
                } else {
                    ancient_world.solve_tsp_by_ant_colony(1., 1., 0.5, 10, 50, 5);
                }

                let time = time.elapsed().as_nanos();
                sum += time as usize;
            }

            alg_mes_data.push(sum / mes_cnt);
        }
        mes_data.push(alg_mes_data);
    }
    mes_data
}
