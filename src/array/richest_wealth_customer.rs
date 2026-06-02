pub fn richest_wealth_customer(accounts: Vec<Vec<i32>>) -> i32 {
    let mut new_vec: Vec<i32> = Vec::new();
    let mut new_number = 0;

    for (_index, value) in accounts.iter().enumerate() {
        for i in value {
            new_number = new_number + i;
        }
        new_vec.push(new_number);
        new_number = 0;
    }

    let greatest = new_vec.iter().max().unwrap();

    println!("New Number {:?}", new_vec);
    *greatest
}
