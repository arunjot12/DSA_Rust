fn main() {
   let arr = [1,2,3,4,5,6];
   let mut i = 0;
   let mut number = 3;
   let mut sum = 0;
   
   while i < number {
     sum = arr[i] + arr[i+1] + arr[i+2];
     println!("The sum of the array in consecutive numbers are {:?}", sum);
     i+=1;
   }
}

// Sliding Window

// Online Rust compiler to run Rust program online
// Print "Start small. Ship something." message

fn main() {
   let arr = [1,2,3,4,5,6];
   let mut i = 1;
   let mut sum = arr[0] + arr[1] + arr[2];
   
   while i <= 3 {
       sum = sum - arr[i-1] + arr[2+i];
       i+=1;
   }
   
}