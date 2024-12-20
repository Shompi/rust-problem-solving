pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    let string_vec = sentence.split_ascii_whitespace();
    let search_word: Vec<char> = search_word.chars().collect();

    let mut found_match = false;

    for (index, string) in string_vec.into_iter().enumerate() {
        let string_vec: Vec<char> = string.chars().collect();

        if string_vec[0] == search_word[0] {
            let mut inner_cursor: usize = 0;

            loop {
                
                if search_word.len() > string_vec.len() {
                    break;
                }
                
                if inner_cursor >= search_word.len() {
                    found_match = true;
                    break;
                }

                if string_vec[inner_cursor] != search_word[inner_cursor] {
                    break;
                }

                inner_cursor += 1;
            }

            if found_match {
                return (index + 1) as i32;
            }
        }
    }

    -1
}

fn main() {
    println!("Hello, world!");

    println!("{}", is_prefix_of_word(String::from("i love eating burger"), String::from("burg")))
}
