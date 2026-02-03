//! # 1480. Running Sum of 1D Array
//!
//! LeetCode: https://leetcode.com/problems/running-sum-of-1d-array/
//!
//! Given an array `nums`, return the running sum of `nums`.
//!
//! Running sum is defined as `runningSum[i] = sum(nums[0]…nums[i])`.
//!
//! ## Example
//! ```
//! Input: nums = [1,2,3,4]
//! Output: [1,3,6,10]
//! Explanation: Running sum is [1, 1+2, 1+2+3, 1+2+3+4]
//! ```

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut running_sum = 0;
    let mut result = Vec::with_capacity(nums.len());

    for num in nums {
        running_sum += num;
        result.push(running_sum);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::running_sum(vec![1, 1, 1, 1, 1]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::running_sum(vec![3, 1, 2, 10, 1]),
            vec![3, 4, 6, 16, 17]
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::running_sum(vec![5]), vec![5]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::running_sum(vec![]), vec![]);
    }
}
