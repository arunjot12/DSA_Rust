//! LeetCode problems module

// ─── Module Declarations ────────────────────────────────────────────────────

pub mod fizzbuzz;
pub mod longest_substring;
pub mod number_of_steps_to_zero;
pub mod remove_duplicay_array;
pub mod remove_element;
pub mod two_sum;
pub mod valid_parenthesis;
pub mod reverse;

// ─── Re-exports ─────────────────────────────────────────────────────────────

pub use fizzbuzz::fizz_buzz;
pub use longest_substring::longest_substring;
pub use number_of_steps_to_zero::number_of_steps;
pub use remove_duplicay_array::Solution as RemoveDuplicatesSolution;
pub use remove_element::*;
pub use two_sum::two_sum;
use crate::leetcode::reverse::reverse;
pub use valid_parenthesis::valid_parenthesis;

// ─── Demo Functions ──────────────────────────────────────────────────────────

/// Demo: FizzBuzz
pub fn run_fizzbuzz_demo() {
    println!("📌 FizzBuzz");
    println!("───────────");
    let n = 15;
    println!("   Input: n = {}", n);
    let result = fizz_buzz(n);
    println!("   Output: {:?}", result);
}

/// Demo: Longest Substring Without Repeating Characters
pub fn run_longest_substring_demo() {
    println!("📌 Longest Substring Without Repeating Characters");
    println!("─────────────────────────────────────────────────");
    let string = "abcda";
    println!("   Input: s = \"{}\"", string);
    let total = longest_substring(string.to_string());
    println!("   Output: {:?}", total);
}

/// Demo: Number of Steps to Reduce to Zero
pub fn run_number_of_steps_demo() {
    println!("📌 Number of Steps to Zero");
    println!("──────────────────────────");
    let num = 14;
    println!("   Input: num = {}", num);
    let result = number_of_steps(num);
    println!("   Steps: {}", result);
}

/// Demo: Remove Duplicates from Sorted Array
pub fn run_remove_duplicates_demo() {
    println!("📌 Remove Duplicates from Sorted Array");
    println!("──────────────────────────────────────");
    let mut nums = vec![1, 1, 2, 2, 3, 4, 4, 5];
    println!("   Input: nums = {:?}", nums);
    let result = RemoveDuplicatesSolution::remove_duplicates(&mut nums);
    println!("   Output Length: {}", result);
}

/// Demo: Remove Element
pub fn run_remove_element_demo() {
    println!("📌 Remove Element");
    println!("─────────────────");
    let mut numbers = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let element = 2;
    println!("   Input: nums = {:?}, val = {}", numbers, element);
    let count = remove_element(&mut numbers, element);
    println!("   Output Length: {}", count);
    println!("   Modified Array: {:?}", numbers);
}

/// Demo: Two Sum
pub fn run_two_sum_demo() {
    println!("📌 Two Sum");
    println!("──────────");
    let data = vec![3, 2, 4];
    let target = 6;
    println!("   Input: nums = {:?}, target = {}", data, target);
    let index = two_sum(data, target);
    println!("   Output: {:?}", index);
}

/// Demo: Valid Parentheses
pub fn run_valid_parenthesis_demo() {
    println!("📌 Valid Parentheses");
    println!("────────────────────");
    let string = "([)]";
    println!("   Input: s = \"{}\"", string);
    let condition = valid_parenthesis(string.to_string());
    println!("   Output: {:?}", condition);
}

pub fn run_reverse(){
    println!("Run Reverse");

    let data = 65;
    let reverse_data = reverse(data);
    println!("Input data is : {:?}, Reverse data is : {:?}",data,reverse_data);

}