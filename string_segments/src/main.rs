// https://leetcode.com/problems/number-of-segments-in-a-string/
// EASY

pub fn count_segments(s: String) -> i32 {

    let mut is_segment = false;
    let mut segments = 0;
    for ch in s.chars() {

        if ch.is_whitespace() {
            if is_segment {
                is_segment = false;
            }

        } else {
            if is_segment {
                continue;
            } else {
                segments += 1;
                is_segment = true;
            }
        }
    }

    segments
}

fn main() {
    println!("Hello, world!");
    println!("Segments: {}", count_segments(String::from("Hello, World!")));
    println!("Segments: {}", count_segments(String::from("asdasd a      asdasd   sadasd")));
}
