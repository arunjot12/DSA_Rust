pub fn number_of_steps(num: i32) -> i32 {
    let mut data = 0;
    let mut input = num;

    while input >= 0 {
        if input == 0 {
            break;
        } else if input % 2 == 0 {
            input = input / 2;
        } else {
            input = input - 1;
        }
        data += 1;
    }
    data
}
