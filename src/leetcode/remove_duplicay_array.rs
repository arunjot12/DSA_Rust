impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // let mut first_value = nums[0];
        let mut new_vec = vec![nums[0]];
        // let mut i = 1;

        println!("Initial Vec: {:?}", nums);
        println!("Initial first_value: {}", nums[0]);
        // println!("Initial i: {}", i);

        for i in 1..nums.len(){
            
            if new_vec.contains(&nums[i]) {
                continue
            }
            else{
                new_vec.push(nums[i]);
            }

        }
          *nums = new_vec.clone();

        println!("\nExited While Loop");
        println!("Final Vec Inside Function: {:?}", nums);

        nums.len() as i32
}
}