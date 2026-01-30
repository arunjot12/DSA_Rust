//! Array algorithms and utilities

pub mod count_even;
pub mod reverse;
pub mod sorted;

// Re-export commonly used functions
pub use count_even::count_even_numbers;
pub use reverse::{reverse_array, reverse_in_place};
pub use sorted::is_sorted;
