pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    for i in 1..n + 1 {
        let convert = i.to_string();
        if i % 3 == 0 && i % 5 == 0 {
            data.push("FizzBuzz".to_string())
        } else if i % 3 == 0 {
            data.push("Fizz".to_string())
        } else if i % 5 == 0 {
            data.push("Buzz".to_string())
        } else {
            data.push(convert)
        }
    }
    data
}
