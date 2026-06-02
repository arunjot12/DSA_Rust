//! # Reverse Array
//!
//! Reverse the elements of an array.
//!
//! ## Example
//! ```
//! Input: nums = [1, 2, 3, 4, 5]
//! Output: [5, 4, 3, 2, 1]
//! ```

pub struct Solution;

impl Solution {
    /// Returns a new vector with elements in reverse order.
    ///
    /// # Arguments
    /// * `nums` - A slice of integers
    ///
    /// # Returns
    /// * A new vector with reversed elements
    ///
    /// # Time Complexity: O(n)
    /// # Space Complexity: O(n)
    #[allow(dead_code)]
    pub fn reverse(nums: &[i32]) -> Vec<i32> {
        nums.iter().rev().cloned().collect()
    }

    /// Reverses the array in place using two-pointer technique.
    ///
    /// # Arguments
    /// * `nums` - A mutable slice of integers
    ///
    /// # Time Complexity: O(n)
    /// # Space Complexity: O(1)
    #[allow(dead_code)]
    pub fn reverse_in_place(nums: &mut [i32]) {
        let n = nums.len();
        if n <= 1 {
            return;
        }

        let mut left = 0;
        let mut right = n - 1;

        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(&[1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_single() {
        assert_eq!(Solution::reverse(&[1]), vec![1]);
    }

    #[test]
    fn test_reverse_empty() {
        assert_eq!(Solution::reverse(&[]), vec![]);
    }

    #[test]
    fn test_reverse_in_place() {
        let mut arr = [1, 2, 3, 4, 5];
        Solution::reverse_in_place(&mut arr);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_in_place_even() {
        let mut arr = [1, 2, 3, 4];
        Solution::reverse_in_place(&mut arr);
        assert_eq!(arr, [4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_in_place_single() {
        let mut arr = [1];
        Solution::reverse_in_place(&mut arr);
        assert_eq!(arr, [1]);
    }
}
