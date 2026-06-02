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