/// Reverses an array by copying elements in reverse order
#[allow(unused)]
pub fn reverse_array() {
    let a = [1, 2, 3, 4, 5];
    let size_of_a = a.len();
    let mut last_index = size_of_a;

    println!("size of a {}", size_of_a);

    // Create new array and fill from end to start
    let mut b = [0; 5];
    for i in a {
        last_index = last_index - 1;
        b[last_index] = i;
    }

    println!("B is {:?}", b);
}
