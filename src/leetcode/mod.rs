//! LeetCode problems module

pub mod fizzbuzz;
pub mod number_of_steps_to_zero;

pub use fizzbuzz::fizz_buzz;
pub use number_of_steps_to_zero::number_of_steps;

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
