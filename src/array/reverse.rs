//! Reverse an array

/// Reverses the given array and returns a new Vec
///
/// # Examples
/// ```
/// use dsa::array::reverse::reverse_array;
/// assert_eq!(reverse_array(&[1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
/// ```
pub fn reverse_array(arr: &[i32]) -> Vec<i32> {
    arr.iter().rev().copied().collect()
}

/// Reverses the array in place
pub fn reverse_in_place(arr: &mut [i32]) {
    arr.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse_array(&[1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_in_place() {
        let mut arr = [1, 2, 3, 4, 5];
        reverse_in_place(&mut arr);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_empty() {
        assert_eq!(reverse_array(&[]), Vec::<i32>::new());
    }
}
