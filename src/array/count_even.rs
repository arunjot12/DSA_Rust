//! Count even numbers in an array

/// Counts the number of even numbers in the given array
///
/// # Examples
/// ```
/// use dsa::array::count_even::count_even_numbers;
/// assert_eq!(count_even_numbers(&[1, 2, 3, 4, 5, 6]), 3);
/// ```
pub fn count_even_numbers(arr: &[i32]) -> usize {
    arr.iter().filter(|&x| x % 2 == 0).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_even() {
        assert_eq!(count_even_numbers(&[1, 2, 3, 7, 8, 10, 12]), 4);
    }

    #[test]
    fn test_no_even() {
        assert_eq!(count_even_numbers(&[1, 3, 5, 7]), 0);
    }

    #[test]
    fn test_all_even() {
        assert_eq!(count_even_numbers(&[2, 4, 6, 8]), 4);
    }
}
