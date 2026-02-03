//! # DSA - Data Structures and Algorithms in Rust 🦀
//!
//! A collection of common data structures and algorithms
//! implemented in Rust for learning and practice.

mod array;
mod linked_list;

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   🦀 DSA - Data Structures & Algorithms   ║");
    println!("╚══════════════════════════════════════════╝");
    println!();

    // Running Sum of 1D Array
    println!("📌 Running Sum of 1D Array");
    println!("───────────────────────────");

    let nums = vec![1, 2, 3, 4, 5];
    let nums_2 = vec![9, 3, 5, 642, 12];
    println!("   Input:       {:?}", nums);

    let result = array::richest_wealth_customer(vec![nums, nums_2]);
    println!("   Running Sum: {:?}", result);

    println!();
    println!("✅ Done!");
}
