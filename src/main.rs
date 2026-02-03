//! # DSA - Data Structures and Algorithms in Rust 🦀
//!
//! A collection of common data structures and algorithms
//! implemented in Rust for learning and practice.

mod array;
mod leetcode;
mod linked_list;

use std::io::{self, Write};

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   🦀 DSA - Data Structures & Algorithms   ║");
    println!("╚══════════════════════════════════════════╝");
    println!();

    loop {
        print_menu();

        let choice = read_input();

        println!();
        match choice.as_str() {
            "1" => array::run_running_sum_demo(),
            "2" => array::run_richest_wealth_demo(),
            "3" => array::run_count_even_demo(),
            "4" => array::run_linear_search_demo(),
            "5" => array::run_reverse_demo(),
            "6" => array::run_sorted_demo(),
            "7" => leetcode::run_fizzbuzz_demo(),
            "8" => leetcode::run_number_of_steps_demo(),
            "9" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option. Please choose 1-9."),
        }
        println!();
    }
}

fn print_menu() {
    println!("┌─────────────────────────────────┐");
    println!("│        Select a Program         │");
    println!("├─────────────────────────────────┤");
    println!("│  1. Running Sum of 1D Array     │");
    println!("│  2. Richest Wealth Customer     │");
    println!("│  3. Count Even Numbers          │");
    println!("│  4. Linear Search               │");
    println!("│  5. Reverse Array               │");
    println!("│  6. Check if Sorted             │");
    println!("│  7. FizzBuzz                    │");
    println!("│  8. Number of Steps to Zero     │");
    println!("│  9. Exit                        │");
    println!("└─────────────────────────────────┘");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
