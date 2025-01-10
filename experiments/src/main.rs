pub fn merge_alternately(word1: String, word2: String) -> String {
    // https://leetcode.com/problems/merge-strings-alternately/
    // EASY

    // Merge 2 strings in alternating order of characters.

    let mut w1_chars = word1.chars().into_iter();
    let mut w2_chars = word2.chars().into_iter();
    let mut final_string: Vec<char> =
        Vec::with_capacity(word1.chars().count() + word2.chars().count());
    let mut w1_finished = false;
    let mut w2_finished = false;

    loop {
        if !w1_finished {
            match w1_chars.next() {
                Some(c) => final_string.push(c),
                None => w1_finished = true,
            }
        }

        if !w2_finished {
            match w2_chars.next() {
                Some(c) => final_string.push(c),
                None => w2_finished = true,
            }
        }

        if w1_finished && w2_finished {
            break;
        };
    }

    final_string.into_iter().collect::<String>()
}

pub fn is_subsequence(s: String, t: String) -> bool {
    // https://leetcode.com/problems/is-subsequence/
    // EASY

    // Every empty string is a substring
    if s.is_empty() && !t.is_empty() {
        return true;
    }

    if s.is_empty() && t.is_empty() {
        return true;
    }

    if !s.is_empty() && t.is_empty() {
        return false;
    }

    let mut pattern_cursor: usize = 0;
    let w1 = t.chars();
    let w2 = s.chars().collect::<Vec<char>>();
    let w2_length = w2.len();

    for c in w1 {
        if pattern_cursor >= w2_length {
            break;
        }

        if c == w2[pattern_cursor] {
            pattern_cursor += 1;
        }
    }

    if pattern_cursor < w2_length {
        return false;
    };

    true
}

fn main() {
    println!(
        "{}",
        is_subsequence(String::from(""), String::from("ahbgdc"))
    );
}
