pub fn longest_substring(s: String) -> i32 {
    let mut prefix = String::from("");
    let mut first = s.chars().nth(0).unwrap();

    for (outer_index, main_char) in s.chars().enumerate() {
        for inner_char in s.chars().skip(outer_index + 1) {
            if inner_char != main_char {
                let combine = first.to_string() + &main_char.to_string();
                let a = &combine[0..combine.len()];
                if prefix.contains(a) {
                    continue;
                } else {
                    prefix = prefix + &combine;
                    first = inner_char;
                }
            } else {
                println!("Characters are same");
                break;
            }

            println!("--------------------------");
        }
    }

    prefix.len() as i32
}
