//! Array algorithms module

pub mod count_even;
pub mod linear_search;
pub mod reverse;
pub mod richest_wealth_customer;
pub mod running_sum_of_1d_array;
pub mod sorted;
pub mod sum_consecutive_number;

// Re-export functions for easy access
pub use count_even::Solution as CountEvenSolution;
pub use linear_search::linear_search;
pub use reverse::Solution as ReverseSolution;
pub use richest_wealth_customer::richest_wealth_customer;
pub use running_sum_of_1d_array::running_sum;
pub use sorted::sorted_array;
pub use sum_consecutive_number::{sum_consecutive_brute_force, sum_consecutive_sliding_window};

/// Demo: Running Sum of 1D Array
pub fn run_running_sum_demo() {
    println!("📌 Running Sum of 1D Array");
    println!("───────────────────────────");
    let nums = vec![1, 2, 3, 4];
    println!("   Input:  {:?}", nums);
    let result = running_sum(nums);
    println!("   Output: {:?}", result);
}

/// Demo: Richest Wealth Customer
pub fn run_richest_wealth_demo() {
    println!("📌 Richest Wealth Customer");
    println!("──────────────────────────");
    let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
    println!("   Accounts: {:?}", accounts);
    let result = richest_wealth_customer(accounts);
    println!("   Richest Wealth: {}", result);
}

/// Demo: Count Even Numbers
pub fn run_count_even_demo() {
    println!("📌 Count Even Numbers");
    println!("─────────────────────");
    let nums = [1, 2, 3, 4, 5, 6];
    println!("   Input: {:?}", nums);
    let result = CountEvenSolution::count_even(&nums);
    println!("   Even Count: {}", result);
}

/// Demo: Linear Search
pub fn run_linear_search_demo() {
    println!("📌 Linear Search");
    println!("────────────────");
    let arr = [10, 23, 45, 70, 11, 15];
    let target = 70;
    println!("   Array:  {:?}", arr);
    println!("   Target: {}", target);
    let result = linear_search(&arr, target);
    if result >= 0 {
        println!("   Found at index: {}", result);
    } else {
        println!("   Not found!");
    }
}

/// Demo: Reverse Array
pub fn run_reverse_demo() {
    println!("📌 Reverse Array");
    println!("────────────────");
    let nums = [1, 2, 3, 4, 5];
    println!("   Input:  {:?}", nums);
    let result = ReverseSolution::reverse(&nums);
    println!("   Output: {:?}", result);
}

/// Demo: Check if Sorted
pub fn run_sorted_demo() {
    println!("📌 Check if Array is Sorted");
    println!("───────────────────────────");
    let arr1 = [1, 2, 7, 8, 10, 12];
    let arr2 = [1, 2, 111, 7, 8];
    println!("   Input 1: {:?}", arr1);
    println!("   Is sorted? {}", sorted_array(&arr1));
    println!("   Input 2: {:?}", arr2);
    println!("   Is sorted? {}", sorted_array(&arr2));
}

/// Demo: Sum Consecutive Numbers
pub fn run_sum_consecutive_demo() {
    println!("📌 Sum Consecutive Numbers");
    println!("──────────────────────────");
    let arr = [1, 2, 3, 4, 5, 6];
    let window = 3;
    println!("   Input: {:?}, window = {}", arr, window);
    sum_consecutive_brute_force(&arr, window);
    sum_consecutive_sliding_window(&arr, window);
}
