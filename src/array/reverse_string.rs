// Online Rust compiler to run Rust program online
// Print "Start small. Ship something." message

fn main() {
    let a = "Arunjot";
    let mut reverse_string = String::new();
    let mut vec_string :Vec<char> =a.chars().collect();
    let mut vec_len = a.len();
    println!("The vec string is {:?}, {:?}",vec_string, vec_len);
    let mut i = 0;
    
    while i < a.len(){
        println!("Data is {:?}",(vec_string[vec_len-1]));
        reverse_string.push(vec_string[vec_len-1]);
        vec_len-=1;
        i+=1;
    }
    println!("Reverse string is {:?}",reverse_string);
}