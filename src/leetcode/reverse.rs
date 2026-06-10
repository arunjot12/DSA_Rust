pub fn reverse(x: i32) -> i32 {
    let mut condition = x;
    let mut rev = 0;

    while condition != 0 {
         let remainder = condition % 10;
          rev = rev * 10 +  remainder;
         condition = condition / 10;
    }

rev
}
