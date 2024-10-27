pub mod time;
pub mod plot;

use lazy_static::lazy_static;

pub const STD_MES_CNT: usize = 1;

lazy_static! {
    pub static ref RECIPES_CNT_VEC: Vec<usize> = || -> Vec<usize> { 
        let mut _vec: Vec<usize> = (1..5).collect();
        _vec.append(&mut (5..=20).step_by(5).collect());
        _vec 
    }();
    pub static ref THREADS_CNT_VEC: Vec<usize> = vec![0, 1, 2, 4, 6, 8, 16, 20, 32, 64];
}