/// Counts even numbers in an array
pub fn count_even_numbers() {
    let a = [1, 2, 3, 7, 8, 10, 12];
    let mut b = 0;

    // Check each element if divisible by 2
    for i in a {
        if i % 2 == 0 {
            b = b + 1;
        }
    }

    println!("the b is {:?}", b);
}
