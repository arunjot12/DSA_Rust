//! Linked List data structure implementations

/// A singly linked list node
pub struct LinkedList {
    data: i32,
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    /// Creates a new LinkedList node with the given data
    pub fn new(data: i32) -> LinkedList {
        LinkedList {
            data,
            next: None,
        }
    }

    /// Returns the data stored in this node
    pub fn data(&self) -> i32 {
        self.data
    }

    /// Appends a new node to the end of the list
    pub fn append(&mut self, data: i32) {
        match self.next {
            Some(ref mut next) => next.append(data),
            None => self.next = Some(Box::new(LinkedList::new(data))),
        }
    }
}

/// Adds two numbers represented as linked lists
pub fn add_two_numbers(
    _l1: Option<Box<LinkedList>>,
    _l2: Option<Box<LinkedList>>,
) -> Option<Box<LinkedList>> {
    // TODO: Implement add two numbers algorithm
    None
}
