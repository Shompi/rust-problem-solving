pub fn str_str(haystack: String, needle: String) -> i32 {
    let mut cursor = 0;
    let mut n_iterator = 0;
    let mut found_at: i32 = -1;
    let haystack_chars: Vec<char> = haystack.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();
    let haystack_count: i32 = haystack_chars.len() as i32;

    while cursor < haystack_count {

        if cursor as usize + needle_chars.len() > haystack_chars.len() {
            return found_at;
        }

        if haystack_chars[cursor as usize] != needle_chars[0] {
            cursor += 1;
            continue;
        } else {

            found_at = cursor;
            loop {

                if (cursor as usize) + n_iterator >= haystack_chars.len() {
                    return found_at;
                }

                if n_iterator >= needle_chars.len() {
                    found_at = cursor;

                    return found_at;
                }

                if haystack_chars[cursor as usize + n_iterator] != needle_chars[n_iterator] {
                    break;

                } else {
                    n_iterator += 1;
                }
            }
        }
        cursor += 1;
        n_iterator = 0;
        found_at = -1;
    }

    found_at
}

fn main() {
    println!("Hello, world!");
    let result = str_str( String::from("a"), String::from("a"));

    println!("Result: {}", result);
}
