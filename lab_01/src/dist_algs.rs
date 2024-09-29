pub type LevenshteinAlgorithm = fn(&str, &str) -> usize;

pub fn recursion_levenshtein(s1: &str, s2: &str) -> usize {
    if s1.is_empty() || s2.is_empty() {
        return (s1.len() as i32 - s2.len() as i32).abs() as usize;
    }

    let mut s1_chars = s1.chars();
    let mut s2_chars = s2.chars();

    let first_char_s1 = s1_chars.next().unwrap();
    let first_char_s2 = s2_chars.next().unwrap();

    if first_char_s1 == first_char_s2 {
        return recursion_levenshtein(s1_chars.as_str(), s2_chars.as_str());
    }

    1 + *[
        recursion_levenshtein(s1_chars.as_str(), s2),
        recursion_levenshtein(s1, s2_chars.as_str()),
        recursion_levenshtein(s1_chars.as_str(), s2_chars.as_str()),
    ]
    .iter()
    .min()
    .unwrap()
}

pub fn table_levenshtein(s1: &str, s2: &str) -> usize {
    let len_s1 = s1.chars().count();
    let len_s2 = s2.chars().count();

    let mut table = vec![vec![0; len_s2 + 1]; len_s1 + 1];

    for i in 0..=len_s1 {
        table[i][0] = i;
    }
    for j in 0..=len_s2 {
        table[0][j] = j;
    }

    let len_s1 = s1.len();
    let len_s2 = s2.len();

    let mut table = vec![vec![0; len_s2 + 1]; len_s1 + 1];

    for i in 0..=len_s1 {
        table[i][0] = i;
    }
    for j in 0..=len_s2 {
        table[0][j] = j;
    }

    for i in 1..=len_s1 {
        for j in 1..=len_s2 {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };

            table[i][j] = *[
                table[i - 1][j] + 1,
                table[i][j - 1] + 1,
                table[i - 1][j - 1] + cost,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    table[len_s1][len_s2]
}

pub fn damerau_levenshtein(s1: &str, s2: &str) -> usize {
    let len_s1 = s1.len();
    let len_s2 = s2.len();

    let mut table = vec![vec![0; len_s2 + 1]; len_s1 + 1];

    for i in 0..=len_s1 {
        table[i][0] = i;
    }
    for j in 0..=len_s2 {
        table[0][j] = j;
    }

    for i in 1..=len_s1 {
        for j in 1..=len_s2 {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };

            table[i][j] = *[
                table[i - 1][j] + 1,
                table[i][j - 1] + 1,
                table[i - 1][j - 1] + cost,
            ]
            .iter()
            .min()
            .unwrap();

            if i > 1
                && j > 1
                && s1.chars().nth(i - 1) == s2.chars().nth(j - 2)
                && s1.chars().nth(i - 2) == s2.chars().nth(j - 1)
            {
                table[i][j] = table[i][j].min(table[i - 2][j - 2] + 1);
            }
        }
    }

    table[len_s1][len_s2]
}
