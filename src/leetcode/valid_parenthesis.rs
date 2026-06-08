pub fn valid_parenthesis(s: String) -> bool{
    let closer = vec![']','}',')'];
    let opener = vec!['[','{','('];
    let mut continously_vec : Vec<(i32,char)> = vec![];
    let vec_string : Vec<char> = s.chars().collect();
    println!("The vec string is {:?}",vec_string);
    let mut iteration = 0;
    let mut push_tuple = 0;
   

   for i in &vec_string{
        if closer.contains(&i){
            push_tuple +=1;
            continue
        }
        else{
            continously_vec.push((push_tuple,*i));
            push_tuple +=1;
        }
    }
    println!("The continously_vec string is {:?}",continously_vec);

    for (i,v) in continously_vec.clone().into_iter().enumerate().rev(){
        let particular_index = v.0 as i32 + iteration + 1;
        let opener_bracket_index = opener.iter().position(|&ch | ch == v.1);

        if vec_string[particular_index as usize ] == closer[opener_bracket_index.unwrap()]{
            continously_vec.remove(i);
            iteration +=1;
        }
    }


    continously_vec.is_empty()
}