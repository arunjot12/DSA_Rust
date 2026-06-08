/// Checks if an array is sorted in ascending order
pub fn sorted_array(arr: &[i32]) -> bool {
    if arr.is_empty() {
        return true;
    }
    
    let mut prev = arr[0];
    for &item in arr.iter().skip(1) {
        if item < prev {
            return false;
        }
        prev = item;
    }
    true
}
