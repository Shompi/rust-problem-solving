pub fn is_vowel(v: char) -> bool {
    match v {
        'a' |
        'e' |
        'i' |
        'o' |
        'u' => true,
        _ => false
    }
}

pub fn reverse_vowels(s: String) -> String {
    let mut str_vec: Vec<char> = s.chars().collect();

    // let mut found_vowel = false;
    let mut swapped = false;
    let str_len = s.len();
    // let half = s.len() / 2;

    let mut cursor = 0;
    let mut front_cursor = str_len - 1;

    while cursor < front_cursor {

        if is_vowel(str_vec[cursor].to_ascii_lowercase()) {

            while front_cursor > cursor && !swapped {

                if is_vowel(str_vec[front_cursor].to_ascii_lowercase()) {
                    // Swap
                    let aux = str_vec[front_cursor];
                    str_vec[front_cursor] = str_vec[cursor];
                    str_vec[cursor] = aux;
                    swapped = true;
                    // found_vowel = false;
                }

                front_cursor -= 1;
            }
            swapped = false;
        }

        cursor += 1;
    }

    String::from(str_vec.into_iter().collect::<String>())
}
fn main() {
    println!("Hello, world!");
    println!("{}", reverse_vowels(String::from("abdefghijk")));
    println!("{}", reverse_vowels(String::from("IceCream")));
}
