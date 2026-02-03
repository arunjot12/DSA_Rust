# 🦀 DSA - Data Structures and Algorithms in Rust

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> A curated collection of data structures and algorithms implemented in Rust for learning and practice.

---

## 📁 Project Structure

```
src/
├── main.rs                        # Entry point with interactive menu
├── array/                         # Array algorithms
│   ├── mod.rs                     # Module exports
│   ├── running_sum_of_1d_array.rs # Running sum algorithm
│   ├── richest_wealth_customer.rs # Find richest customer
│   ├── sorted.rs                  # Check if array is sorted
│   ├── count_even.rs              # Count even numbers
│   ├── linear_search.rs           # Linear search algorithm
│   └── reverse.rs                 # Reverse array
├── leetcode/                      # LeetCode solutions
│   ├── mod.rs                     # Module exports
│   ├── fizzbuzz.rs                # FizzBuzz problem
│   └── number_of_steps_to_zero.rs # Number of steps to reduce to zero
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
```

---

## 🎮 Interactive Menu

Run `cargo run` to launch the interactive menu:

```
┌─────────────────────────────────┐
│        Select a Program         │
├─────────────────────────────────┤
│  1. Running Sum of 1D Array     │
│  2. Richest Wealth Customer     │
│  3. Count Even Numbers          │
│  4. Linear Search               │
│  5. Reverse Array               │
│  6. Check if Sorted             │
│  7. FizzBuzz                    │
│  8. Number of Steps to Zero     │
│  9. Exit                        │
└─────────────────────────────────┘
```

---

## 📚 Implemented Algorithms

### Array Algorithms

| Algorithm | File | Description |
|-----------|------|-------------|
| **Running Sum** | `running_sum_of_1d_array.rs` | Calculate running sum of 1D array |
| **Richest Wealth** | `richest_wealth_customer.rs` | Find customer with maximum wealth |
| **Sorted Check** | `sorted.rs` | Check if array is sorted in ascending order |
| **Count Even** | `count_even.rs` | Count even numbers in array |
| **Linear Search** | `linear_search.rs` | Search for element in array |
| **Reverse** | `reverse.rs` | Reverse an array |

### LeetCode Solutions

| # | Problem | Solution |
|---|---------|----------|
| 412 | [FizzBuzz](https://leetcode.com/problems/fizz-buzz/) | [`fizzbuzz.rs`](src/leetcode/fizzbuzz.rs) |
| 1342 | [Number of Steps to Reduce to Zero](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/) | [`number_of_steps_to_zero.rs`](src/leetcode/number_of_steps_to_zero.rs) |
| 1480 | [Running Sum of 1D Array](https://leetcode.com/problems/running-sum-of-1d-array/) | [`running_sum_of_1d_array.rs`](src/array/running_sum_of_1d_array.rs) |
| 1672 | [Richest Customer Wealth](https://leetcode.com/problems/richest-customer-wealth/) | [`richest_wealth_customer.rs`](src/array/richest_wealth_customer.rs) |

### Linked List

| Method | Description |
|--------|-------------|
| `LinkedList::new(data)` | Create a new linked list node |
| `add_two_numbers(l1, l2)` | Add two numbers represented as linked lists |

---

## 📝 License

This project is licensed under the MIT License.

---

<p align="center">
  Made with ❤️ and 🦀
</p>
