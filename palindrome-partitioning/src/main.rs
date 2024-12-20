pub fn is_palindrome(s: String) -> bool {
    let string_as_vec: Vec<char> = s.chars().collect();

    let mut cursor = 0;
    let string_length = string_as_vec.len();
    while cursor < (string_length / 2) {
        if string_as_vec[cursor] != string_as_vec[string_length - cursor - 1] {
            return false;
        }

        cursor += 1;
    }
    true
}


pub fn partition(s: String) -> Vec<Vec<String>> {
    if is_palindrome(s) {}

    todo!("I'll come back to this later.");
}

fn main() {
    println!("Hello, world!");
    println!("PALINDROME: {}", is_palindrome(String::from("aabbaa")));
    println!("PALINDROME: {}", is_palindrome(String::from("aabbab")));
}
