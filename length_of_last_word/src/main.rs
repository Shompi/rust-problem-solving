pub fn length_of_last_word(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut cursor = chars.len();
    let mut word_length = 0;

    while cursor > 0 {
        while cursor > 0 && chars[cursor - 1].is_whitespace() {
            cursor -= 1;
        }

        while cursor > 0 && !chars[cursor - 1].is_whitespace() {
            word_length += 1;
            cursor -= 1;
        }
        break;
    }

    word_length
}
fn main() {
    println!("Hello, world!");
    println!(
        "Length of last word: {}",
        length_of_last_word(String::from("   fly me   to   the moon  "))
    );
    println!(
        "Length of last word: {}",
        length_of_last_word(String::from("luffy is still joyboy"))
    );
    println!(
        "Length of last word: {}",
        length_of_last_word(String::from(""))
    );
    println!(
        "Length of last word: {}",
        length_of_last_word(String::from(" "))
    );
    println!(
        "Length of last word: {}",
        length_of_last_word(String::from("  a  "))
    );
}
