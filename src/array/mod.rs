//! Array algorithms module

pub mod count_even;
pub mod reverse;
pub mod running_sum_of_1d_array;
pub mod sorted;

// Re-export functions for easy access
pub use count_even::count_even_numbers;
pub use reverse::reverse_array;
pub use running_sum_of_1d_array::running_sum;
pub use sorted::sorted_array;
