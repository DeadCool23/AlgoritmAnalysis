use lab_01::dist_algs::table_levenshtein;

#[test]
fn empty_strs() {
    assert_eq!(table_levenshtein("", ""), 0)
}

#[test]
fn one_empty_str() {
    assert_eq!(table_levenshtein("", "abc"), 3)
}

#[test]
fn diff_size_std_without_rep() {
    assert_eq!(table_levenshtein("a", "bcd"), 3)
}

#[test]
fn diff_size_std_with_rep() {
    assert_eq!(table_levenshtein("a", "abc"), 2)
}

#[test]
fn eq_char_strs() {
    assert_eq!(table_levenshtein("a", "a"), 0)
}

#[test]
fn neq_char_strs() {
    assert_eq!(table_levenshtein("a", "b"), 1)
}

const BIG_STR_LEN: usize = 10;

#[test]
fn eq_big_str() {
    let len = BIG_STR_LEN;
    let str1 = "a".repeat(len);
    let str2 = "a".repeat(len);
    assert_eq!(table_levenshtein(&str1, &str2), 0)
}

#[test]
fn neq_big_str() {
    let len = BIG_STR_LEN;
    let str1 = "a".repeat(len);
    let str2 = "b".repeat(len);
    assert_eq!(table_levenshtein(&str1, &str2), len)
}

#[test]
fn halfeq_big_str() {
    let len = BIG_STR_LEN;
    let half_len = len / 2;

    let str1 = "a".repeat(len);
    let str2 = "b".repeat(half_len) + &"a".repeat(len - half_len);
    assert_eq!(table_levenshtein(&str1, &str2), half_len)
}

#[test]
fn nums_str() {
    assert_eq!(table_levenshtein("1234", "2143"), 3)
}

#[test]
fn russian_strs() {
    assert_eq!(table_levenshtein("мука", "река"), 2)
}
