/// A singly linked list node
#[allow(dead_code)]
pub struct LinkedList {
    data: i32,
    next: Option<Box<LinkedList>>,
}

#[allow(dead_code)]
impl LinkedList {
    /// Creates a new LinkedList node
    pub fn new(data: i32) -> LinkedList {
        LinkedList {
            data: data,
            next: None,
        }
    }
}

/// Adds two numbers represented as linked lists
/// TODO: implement the logic
#[allow(unused)]
pub fn add_two_numbers(l1: Option<Box<LinkedList>>, l2: Option<Box<LinkedList>>) {
    // Your implementation here
}
