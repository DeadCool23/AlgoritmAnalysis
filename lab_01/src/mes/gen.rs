use rand::Rng; // Модуль для генерации случайных чисел
pub type GenStrFn = fn(usize) -> (String, String);

// # генерация одинаковых слов размера size
pub fn eq_strs(size: usize) -> (String, String) {
    let s = "a".repeat(size).to_string();
    let clone_s = s.clone();
    (s, clone_s)
}

// # генерация разных слов размера size
pub fn neq_strs(size: usize) -> (String, String) {
    let s1 = "a".repeat(size).to_string();
    let s2 = "b".repeat(size).to_string();
    (s1, s2)
}

// # генерация рандомного слова размера size
fn generate_random_string(size: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = ('a'..='z').collect();
    
    (0..size)
    .map(|_| chars[rng.gen_range(0..chars.len())])
    .collect()
}

#[allow(dead_code)]
// # генерация рандомных слов размера size
pub fn random_strs(size: usize) -> (String, String) {
    let s1 = generate_random_string(size);
    let s2 = generate_random_string(size);
    (s1, s2)
}
