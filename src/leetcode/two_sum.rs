pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut return_index: Vec<i32> = vec![];
    let mut first_value: (i32, i32) = (0, nums[0]);

    println!("Input nums   : {:?}", nums);
    println!("Target       : {}", target);
    println!("Initial pair : {:?}", first_value);

     for index in 1..nums.len(){
    // for (index, value) in nums.iter().enumerate() {
        println!("\n--- New Iteration ---");
        println!("Current index : {}", index);
        println!("Current value : {}", nums[index]);
        println!("Stored value  : {:?}", first_value);

        if first_value.1 + nums[index] == target {
            println!(
                "MATCH FOUND -> {} + {} = {}",
                first_value.1,
                nums[index],
                target
            );

            return_index.push(first_value.0 as i32);
            return_index.push(index as i32);

            println!("return_index = {:?}", return_index);
        } else {
            println!(
                "No match -> {} + {} = {}",
                first_value.1,
                nums[index],
                first_value.1 + nums[index]
            );

            first_value = (index as i32, nums[index]);

            println!("Updated first_value = {:?}", first_value);
        }
    }

    println!("\nFinal return_index = {:?}", return_index);

    return_index
}