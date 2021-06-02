fn main() {
    let balanced = String::from("{}(()[])");
//    let balanced = String::from("()");
    let un_balanced = String::from("(()[)]");
    let empty_string = String::from("");
    let string_without_brackets = String::from("HI! My name is. My name is.");
    let s1 = String::from("HI! My)( name is. My name is.");
    
    println!("is {} balanced? {}", balanced, check_balance(&balanced));
    println!("is {} balanced? {}", un_balanced, check_balance(&un_balanced));
    println!("is {} balanced? {}", empty_string, check_balance(&empty_string));
    println!("is {} balanced? {}", string_without_brackets, check_balance(&string_without_brackets));
    println!("is {} balanced? {}", s1, check_balance(&s1));

}

fn check_balance(input_string: &String) -> bool {
    let opening_brack_def = "({[";
    let closing_brack_def = ")}]";
    let mut brack_stack = Vec::new();
    for c in input_string.chars() {
        if opening_brack_def.contains(c) {
            brack_stack.push(c);
        } else if closing_brack_def.contains(c) {
            let cur_closing = c;

            match brack_stack.last() {
                Some(cur_opening) => {
                    if opening_brack_def.chars().position(|c| c == *cur_opening).unwrap() == closing_brack_def.chars().position(|c| c == cur_closing).unwrap() {
                        brack_stack.pop();
                    }
                },
                _ => return false
            }
        }
    }
    if brack_stack.len() == 0 {
        return true
    } else {
        return false;
    }
}
