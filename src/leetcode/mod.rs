//! LeetCode problems module

pub mod fizzbuzz;

pub use fizzbuzz::fizz_buzz;

/// Demo: FizzBuzz
pub fn run_fizzbuzz_demo() {
    println!("📌 FizzBuzz");
    println!("───────────");
    let n = 15;
    println!("   Input: n = {}", n);
    let result = fizz_buzz(n);
    println!("   Output: {:?}", result);
}
