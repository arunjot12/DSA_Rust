//! # Check if Array is Sorted
//!
//! Check if an array is sorted in ascending order.
//!
//! ## Example
//! ```
//! Input: nums = [1, 2, 3, 4, 5]
//! Output: true
//!
//! Input: nums = [1, 3, 2, 4, 5]
//! Output: false
//! ```

pub struct Solution;

impl Solution {
    /// Checks if the array is sorted in non-decreasing order.
    ///
    /// # Arguments
    /// * `nums` - A slice of integers
    ///
    /// # Returns
    /// * `true` if the array is sorted, `false` otherwise
    ///
    /// # Time Complexity: O(n)
    /// # Space Complexity: O(1)
    #[allow(dead_code)]
    pub fn is_sorted(nums: &[i32]) -> bool {
        if nums.len() <= 1 {
            return true;
        }

        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_array() {
        assert!(Solution::is_sorted(&[1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_unsorted_array() {
        assert!(!Solution::is_sorted(&[1, 3, 2, 4, 5]));
    }

    #[test]
    fn test_single_element() {
        assert!(Solution::is_sorted(&[1]));
    }

    #[test]
    fn test_empty_array() {
        assert!(Solution::is_sorted(&[]));
    }

    #[test]
    fn test_equal_elements() {
        assert!(Solution::is_sorted(&[2, 2, 2, 2]));
    }

    #[test]
    fn test_descending() {
        assert!(!Solution::is_sorted(&[5, 4, 3, 2, 1]));
    }
}
