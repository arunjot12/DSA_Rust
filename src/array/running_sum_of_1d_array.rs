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
