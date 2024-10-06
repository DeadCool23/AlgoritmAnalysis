use lab_03::arr::searchers;

#[test]
fn in_arr_test() {
    let arr = vec![1, 2, 3, 4];
    let res = searchers::binary_search(&arr, &2);
    assert_eq!(res.0, Some(1));
}

#[test]
fn not_in_arr_test() {
    let arr = vec![1, 2, 3, 4];
    let res = searchers::binary_search(&arr, &5);
    assert_eq!(res.0, None);
}