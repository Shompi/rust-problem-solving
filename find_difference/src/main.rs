// https://leetcode.com/problems/find-the-difference/description/
// EASY

pub fn find_the_difference(s: String, t: String) -> char {


    if s.len() == 0 {
        return t.chars().next().unwrap();
    }

    let mut sum: i32 = 0;    

    for c in t.chars() {
        // println!("char: {}", c);
        sum += c as i32;
    }

    for c in s.chars() {
        sum -= c as i32;
    }

    char::from(sum as u8)
}


fn main() {
    println!("Hello, world!");
    println!("char: {:?}", find_the_difference(String::from("abcd"), String::from("abcde")));
    println!("char: {:?}", find_the_difference(String::from("abcd"), String::from("abgcd")));
    println!("char: {:?}", find_the_difference(String::from("abcd"), String::from("abcdx")));
}
