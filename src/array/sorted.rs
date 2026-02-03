/// Checks if an array is sorted in ascending order
#[allow(dead_code)]
pub fn sorted_array() {
    let a = [1, 2, 111, 7, 8, 10, 12];
    let mut first_array = a[0];
    let mut condition = true;

    // Iterate through array and compare each element with previous
    for i in a {
        condition = if i >= first_array {
            first_array = i;
            true
        } else {
            false
        };
    }

    println!("Array is sorted {:?}", condition);
}
