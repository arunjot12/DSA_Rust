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
    println!("   Input:       {:?}", nums);

    let result = array::running_sum(nums);
    println!("   Running Sum: {:?}", result);

    println!();
    println!("✅ Done!");
}
