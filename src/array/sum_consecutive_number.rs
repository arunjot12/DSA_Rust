pub fn sum_consecutive_brute_force(arr: &[i32], window: usize) {
    if arr.len() < window { return; }
    println!("   Brute Force:");
    for i in 0..=arr.len() - window {
        let sum: i32 = arr[i..i + window].iter().sum();
        println!("     Window {:?}, sum = {}", &arr[i..i + window], sum);
    }
}

pub fn sum_consecutive_sliding_window(arr: &[i32], window: usize) {
    if arr.len() < window { return; }
    println!("   Sliding Window:");
    let mut sum: i32 = arr[0..window].iter().sum();
    println!("     Window {:?}, sum = {}", &arr[0..window], sum);
    for i in 1..=arr.len() - window {
        sum = sum - arr[i - 1] + arr[i + window - 1];
        println!("     Window {:?}, sum = {}", &arr[i..i + window], sum);
    }
}
