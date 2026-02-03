//! Linear Search Algorithm

#[allow(dead_code)]
fn linear() {
    let arr = [1, 2, 44, 53];
    let target = 98;
    let position = linear_search(&arr, target);
    println!("Position is {:?}", position)
}

/// Performs linear search on an array
///
/// # Returns
/// Index of target if found, -1 otherwise
pub fn linear_search(arr: &[i32], target: i32) -> i32 {
    let mut index = -1;

    let mut check_index_aval = 0;

    for i in arr {
        if *i == target {
            check_index_aval += 1;
        }
    }

    if check_index_aval <= 0 {
        return -1;
    }

    for i in arr {
        index = index + 1;
        if *i == target {
            break;
        }
    }
    index
}
