
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
      let mut i = 0;
       while i < nums.len(){
         println!("The i is {:?}",i);
         println!("The i is {:?}",nums[i] );
                if nums[i] as i32 == val {
                  println!("Removed");
                   nums.remove(i);
                  
                }
                else{
                  i+=1;
                }     
        }
        nums.len() as i32
    }