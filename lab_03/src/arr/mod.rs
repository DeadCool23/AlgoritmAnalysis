pub mod searchers;

pub fn gen_sorted_vec(size: usize) -> Vec<i32> {
    let mut vec = Vec::<i32>::new();
    for i in 0..size {
        vec.push(i as i32);
    }
    vec
}