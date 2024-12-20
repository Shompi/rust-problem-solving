pub fn is_palindrome(s: String) -> bool {
    let string_vec: Vec<char> = s.chars().collect();
    let mut clean_string: Vec<char> = vec![];

    for ch in &string_vec {

        let c = ch.to_ascii_lowercase();

        if c.is_alphanumeric() {
            clean_string.push(c);
        }
    }

    // println!("PROCESED STRING: {:?}", clean_string);

    let mut cursor = 0;
    let string_length = clean_string.len();
    while cursor < (string_length / 2) {
        if clean_string[cursor] != clean_string[string_length - cursor - 1] {
            return false;
        }

        cursor += 1;
    }
    true
}

fn main() {

    let tests: Vec<String> = vec![
        String::from("A man, a plan, a canal: Panama"),
        String::from("race a car"),
        String::from(" "),
    ];

    for test in &tests {
        println!("INPUT: {}\n PALINDROME: {}", test, is_palindrome(String::from(test)));
    }
}
