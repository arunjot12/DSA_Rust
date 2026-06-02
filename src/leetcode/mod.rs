//! LeetCode problems module

pub mod fizzbuzz;
pub mod longest_substring;
pub mod two_sum;
pub mod number_of_steps_to_zero;

pub use fizzbuzz::fizz_buzz;
pub use number_of_steps_to_zero::number_of_steps;
pub use two_sum::two_sum;
pub use longest_substring::longest_substring;

/// Demo: FizzBuzz
pub fn run_fizzbuzz_demo() {
    println!("📌 FizzBuzz");
    println!("───────────");
    let n = 15;
    println!("   Input: n = {}", n);
    let result = fizz_buzz(n);
    println!("   Output: {:?}", result);
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

pub fn leetcode_two_sum() {
    let data = vec![3,2,4];
    let target = 6;
    let index = two_sum(data, target);
    println!("The indexes of both numbers are {:?}",index);
}

pub fn leetcode_longest_substring(){
    let string = "arunjot";
    let total = longest_substring(string.to_string());
    println!("The total of String is {:?}",total);
}