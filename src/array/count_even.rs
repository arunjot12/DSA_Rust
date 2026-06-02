//! # Count Even Numbers
//!
//! Count the number of even numbers in an array.
//!
//! ## Example
//! ```
//! Input: nums = [1, 2, 3, 4, 5, 6]
//! Output: 3
//! Explanation: 2, 4, and 6 are even numbers
//! ```

pub struct Solution;

impl Solution {
    /// Counts the number of even integers in the array.
    ///
    /// # Arguments
    /// * `nums` - A slice of integers
    ///
    /// # Returns
    /// * The count of even numbers
    ///
    /// # Time Complexity: O(n)
    /// # Space Complexity: O(1)
    #[allow(dead_code)]
    pub fn count_even(nums: &[i32]) -> i32 {
        nums.iter().filter(|&x| x % 2 == 0).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_array() {
        assert_eq!(Solution::count_even(&[1, 2, 3, 4, 5, 6]), 3);
    }

    #[test]
    fn test_all_even() {
        assert_eq!(Solution::count_even(&[2, 4, 6, 8]), 4);
    }

    #[test]
    fn test_all_odd() {
        assert_eq!(Solution::count_even(&[1, 3, 5, 7]), 0);
    }

    #[test]
    fn test_empty_array() {
        assert_eq!(Solution::count_even(&[]), 0);
    }

    #[test]
    fn test_with_zero() {
        assert_eq!(Solution::count_even(&[0, 1, 2]), 2); // 0 is even
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(Solution::count_even(&[-2, -1, 0, 1, 2]), 3);
    }
}
