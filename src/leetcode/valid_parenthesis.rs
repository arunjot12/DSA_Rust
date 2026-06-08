pub fn valid_parenthesis(s: String) -> bool {
    let mut stack = Vec::new();

    for i in s.chars(){
        match i {
            '{' | '(' | '['  =>  stack.push(i), 
            '}'=> if stack.pop() !=  Some('{') { return false; },
            ')'=> if stack.pop() !=  Some('(') { return false; },
            ']'=> if stack.pop() !=  Some('[') { return false; },
             _ => {},
        
         }
    }
    stack.is_empty()
}
