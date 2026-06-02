pub fn longest_substring(s: String) -> i32 {
    let mut temp = 1;
    let mut earlier_temp = 0;

    println!("Input String: {}\n", s);

    for (outer_index, main_char) in s.chars().enumerate() {
        println!("==========================");
        println!("Outer Loop: main_char = {}", main_char);

        for other_char in s.chars().skip(outer_index + 1) {
            println!(
                "Comparing main_char '{}' with other_char '{}'",
                main_char, other_char

            );

            if other_char != main_char {
                temp += 1;
                println!("Characters are different");
                println!("temp increased to {}", temp);
            } else {
                println!("Characters are same");
                break;
            }

            println!("--------------------------");
        }

          if temp >= earlier_temp {
                println!(
                    "temp ({}) >= earlier_temp ({})",
                    temp, earlier_temp
                );
                earlier_temp = temp;

                println!("earlier_temp updated to {}", earlier_temp);
            }
               temp = 0;

    }

    println!("\nFinal earlier_temp = {}", earlier_temp);

    earlier_temp
}