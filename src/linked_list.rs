pub struct LinkedList {
    data: i32,
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    fn new(data: i32) -> LinkedList {
        LinkedList {
            data: data,
            next: None,
        }
    }
}

fn main() {
    fn add_two_numbers(l1: Option<Box<LinkedList>>, l2: Option<Box<LinkedList>>) {
        
    }
}
