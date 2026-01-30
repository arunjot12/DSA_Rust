# DSA - Data Structures and Algorithms in Rust 🦀

A collection of common data structures and algorithms implemented in Rust for learning purposes.

## Project Structure

```
src/
├── main.rs              # Entry point with demos
├── array/               # Array algorithms
│   ├── mod.rs           # Module exports
│   ├── sorted.rs        # Check if array is sorted
│   ├── count_even.rs    # Count even numbers
│   └── reverse.rs       # Reverse array
└── linked_list/         # Linked list implementations
    └── mod.rs           # LinkedList struct and methods
```

## Getting Started

```bash
# Run the demo
cargo run

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## Modules

### Array (`array::`)

| Function | Description |
|----------|-------------|
| `is_sorted(&[i32])` | Check if array is sorted in ascending order |
| `count_even_numbers(&[i32])` | Count even numbers in array |
| `reverse_array(&[i32])` | Return a reversed copy of the array |
| `reverse_in_place(&mut [i32])` | Reverse array in place |

### Linked List (`linked_list::`)

| Method | Description |
|--------|-------------|
| `LinkedList::new(data)` | Create a new linked list node |
| `append(data)` | Add a node to the end of the list |
| `data()` | Get the data from the current node |

## Example Usage

```rust
use dsa::array;
use dsa::linked_list::LinkedList;

// Array operations
let arr = [1, 2, 3, 4, 5];
assert!(array::is_sorted(&arr));
assert_eq!(array::count_even_numbers(&arr), 2);
assert_eq!(array::reverse_array(&arr), vec![5, 4, 3, 2, 1]);

// Linked list
let mut list = LinkedList::new(10);
list.append(20);
list.append(30);
```

## License

MIT
