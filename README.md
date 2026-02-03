# 🦀 DSA - Data Structures and Algorithms in Rust

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> A curated collection of data structures and algorithms implemented in Rust for learning and practice.

---

## 📁 Project Structure

```
src/
├── main.rs                        # Entry point
├── array/                         # Array algorithms
│   ├── mod.rs                     # Module exports
│   ├── running_sum_of_1d_array.rs # Running sum algorithm
│   ├── sorted.rs                  # Check if array is sorted
│   ├── count_even.rs              # Count even numbers
│   └── reverse.rs                 # Reverse array
└── linked_list/                   # Linked list implementations
    └── mod.rs                     # LinkedList struct and methods
```

---

## 🚀 Getting Started

```bash
# Clone the repository
git clone https://github.com/arunjot12/DSA_Rust.git
cd DSA_Rust

# Run the program
cargo run

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

---

## 📚 Implemented Algorithms

### Array Algorithms

| Algorithm | File | Description |
|-----------|------|-------------|
| **Running Sum** | `running_sum_of_1d_array.rs` | Calculate running sum of 1D array |
| **Sorted Check** | `sorted.rs` | Check if array is sorted in ascending order |
| **Count Even** | `count_even.rs` | Count even numbers in array |
| **Reverse** | `reverse.rs` | Reverse an array |

### Linked List

| Method | Description |
|--------|-------------|
| `LinkedList::new(data)` | Create a new linked list node |
| `add_two_numbers(l1, l2)` | Add two numbers represented as linked lists |

---

## 💡 Example

```rust
use dsa::array;

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let result = array::running_sum(nums);
    
    println!("{:?}", result);
    // Output: [1, 3, 6, 10, 15]
}
```

---

## 🎯 LeetCode Problems

| # | Problem | Solution |
|---|---------|----------|
| 1480 | [Running Sum of 1D Array](https://leetcode.com/problems/running-sum-of-1d-array/) | [`running_sum_of_1d_array.rs`](src/array/running_sum_of_1d_array.rs) |

---

## 📝 License

This project is licensed under the MIT License.

---

<p align="center">
  Made with ❤️ and 🦀
</p>
