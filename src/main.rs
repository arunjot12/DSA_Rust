//! DSA - Data Structures and Algorithms in Rust
//!
//! A collection of common data structures and algorithms implemented in Rust
//! for learning purposes.

pub mod array;
pub mod linked_list;

fn main() {
    println!("=== DSA in Rust ===\n");

    // Array examples
    println!("📦 Array Operations:");
    
    let arr = [1, 2, 3, 4, 5];
    println!("  Original array: {:?}", arr);
    println!("  Is sorted: {}", array::is_sorted(&arr));
    println!("  Even count: {}", array::count_even_numbers(&arr));
    println!("  Reversed: {:?}", array::reverse_array(&arr));

    let unsorted = [5, 2, 8, 1, 9];
    println!("\n  Unsorted array: {:?}", unsorted);
    println!("  Is sorted: {}", array::is_sorted(&unsorted));

    // Linked list example
    println!("\n🔗 Linked List:");
    let mut list = linked_list::LinkedList::new(1);
    list.append(2);
    list.append(3);
    println!("  Created linked list: 1 -> 2 -> 3");
    println!("  Head data: {}", list.data());

    println!("\n✅ All demos completed!");
}
