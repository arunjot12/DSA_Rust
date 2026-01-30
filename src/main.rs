//! DSA - Data Structures and Algorithms in Rust

use std::io::{self, Write};

mod array;
mod linked_list;

fn main() {
    println!("=== DSA in Rust ===\n");
    println!("Choose a program to run:");
    println!("  1. Check if array is sorted");
    println!("  2. Count even numbers");
    println!("  3. Reverse array");
    println!("  4. Linked list demo");
    println!("  0. Exit\n");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => array::sorted_array(),
        "2" => array::count_even_numbers(),
        "3" => array::reverse_array(),
        "4" => {
            println!("Linked list - add_two_numbers (TODO)");
            linked_list::add_two_numbers(None, None);
        }
        "0" => println!("Goodbye!"),
        _ => println!("Invalid choice!"),
    }
}
