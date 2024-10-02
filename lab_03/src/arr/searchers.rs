pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> (Option<usize>, usize) {
    let mut iterations = 0;

    for (index, item) in arr.iter().enumerate() {
        iterations += 1;
        if item == target {
            return (Some(index), iterations);
        }
    }

    (None, iterations)
}

pub fn binary_search<T: PartialOrd>(arr: &[T], target: &T) -> (Option<usize>, usize) {
    let mut low = 0;
    let mut high = arr.len() as isize - 1;
    let mut iterations = 0;

    while low <= high {
        iterations += 1;
        let mid = ((low + high) / 2) as usize;

        if &arr[mid] == target {
            return (Some(mid), iterations);
        } else if &arr[mid] < target {
            low = mid as isize + 1;
        } else {
            high = mid as isize - 1;
        }
    }

    (None, iterations)
}