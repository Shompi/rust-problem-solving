/*
    Example 1:

    Input: s = "(()"
    Output: 2
    Explanation: The longest valid parentheses substring is "()".

    Example 2:

    Input: s = ")()())"
    Output: 4
    Explanation: The longest valid parentheses substring is "()()".
*/

pub fn longest_valid_parentheses(s: String) -> i32 {

    /* 
        turns out ()(()) is valid too so this code doesn't work
        yippeee
    */

    if s.chars().count() == 0 {
        return 0;
    }

    let mut longest = 0;
    let mut valid_vector: Vec<bool> = Vec::with_capacity(s.chars().count());
    let string_as_vec: Vec<char> = s.chars().collect();
    let mut cursor: usize = 0;

    loop {
        if (cursor as i32) + 1 >= string_as_vec.len() as i32 {
            break;
        }

        let mut valid = false;

        if string_as_vec[cursor] == '(' && string_as_vec[cursor + 1] == ')' {
            valid = true;
        }

        if valid {
            // valid_vector[cursor] = true;
            // valid_vector[cursor + 1] = true;

            valid_vector.push(true);
            valid_vector.push(true);

            cursor += 2;
            continue;
        } else {
            valid_vector.push(false);
        }

        cursor += 1;
    }

    cursor = 0;

    let mut acc = 0;
    while cursor < valid_vector.len() {
        if valid_vector[cursor] {
            acc += 1
        } else {
            if acc > longest {
                longest = acc;
            }
            acc = 0;
        }

        cursor += 1;
    }

    // If we ended looping in a non-zero value we have to check the acc

    if acc > longest {
        longest = acc;
    }

    longest
}

fn main() {
    println!("Hello, world!");
    // println!(
    //     "Longest: {}",
    //     longest_valid_parentheses(String::from("()"))
    // );
    println!(
        "Longest: {}",
        longest_valid_parentheses(String::from("()())()()()()(((((())()"))
    );
    // println!(
    //     "Longest: {}",
    //     longest_valid_parentheses(String::from("(()()"))
    // );
}
