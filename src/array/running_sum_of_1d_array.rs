pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut running_sum = 0;
    let mut new_vec = Vec::new();
    for num in nums {
        running_sum += num;
        new_vec.push(running_sum)
    }
    new_vec
}
