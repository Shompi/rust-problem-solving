pub fn closes_parenthesis(open: char, closes: char) -> bool {
    if open == '(' && closes == ')' {
        return true;
    } else if open == '{' && closes == '}' {
        return true;
    } else if open == '[' && closes == ']' {
        return true;
    }

    false
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    let string_chars: Vec<char> = s.chars().collect();

    let mut cursor: usize = 0;

    while cursor < string_chars.len() {
        let paren = string_chars[cursor];

        if paren == '(' || paren == '[' || paren == '{' {
            stack.push(paren);
        } else {
            let last_paren = stack.pop();

            match last_paren {
                None => return false, // We simply cannot close a parenthesis if there are none pending
                Some(open_paren) => {
                    if !closes_parenthesis(open_paren, paren) {
                        return false;
                    }
                }
            }
        }
        cursor += 1;
    }

    true
}

fn main() {
    println!("Hello, world!");

    println!("VALID: {}",is_valid(String::from("()")).to_string());
    println!("VALID: {}",is_valid(String::from("()[]{}")).to_string());
    println!("VALID: {}",is_valid(String::from("(]")).to_string());
    println!("VALID: {}",is_valid(String::from("([])")).to_string());
}
