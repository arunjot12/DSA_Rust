//! Check if an array is sorted

/// Checks if the given array is sorted in ascending order
///
/// # Examples
/// ```
/// use dsa::array::sorted::is_sorted;
/// assert!(is_sorted(&[1, 2, 3, 4, 5]));
/// assert!(!is_sorted(&[1, 3, 2, 4, 5]));
/// ```
pub fn is_sorted(arr: &[i32]) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_array() {
        assert!(is_sorted(&[1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_unsorted_array() {
        assert!(!is_sorted(&[1, 2, 111, 7, 8, 10, 12]));
    }

    #[test]
    fn test_empty_array() {
        assert!(is_sorted(&[]));
    }
}
