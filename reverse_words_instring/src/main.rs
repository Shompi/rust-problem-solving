/// https://leetcode.com/problems/reverse-words-in-a-string/
/// MEDIUM

pub fn reverse_words(s: String) -> String {


    // Reverse each word of the string, not the entire string

    let mut p = s.trim()
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    p.reverse();

    p.join(" ")
}

fn main() {
    println!("Hello, world!");
    println!("{}", reverse_words(String::from("the sky is blue")));
    println!("{}", reverse_words(String::from("  hello world  ")));
    println!("{}", reverse_words(String::from("a good   example")));
}
