pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut _longest: Vec<char> = strs[0].chars().collect();

    let mut max = _longest.len();

    for r#string in strs {
        let str_vec: Vec<char> = r#string.chars().collect();
        let str_vec_len = str_vec.len();

        for (index, r#char) in str_vec.into_iter().enumerate() {
            if max == 0 {
                return String::from("");
            }

            if index >= max {
                break;
            }

            if r#char != _longest[index] {
                max = index;
            }
        }

        if max > str_vec_len {
            max = str_vec_len;
        }
    }

    let mut cursor = 0;
    let mut answer: String = String::new();

    while cursor < max {
        answer.push(_longest[cursor]);
        cursor += 1;
    }

    answer
}

pub fn longest_common_prefix_v2(strs: Vec<String>) -> String {
    if strs.first().unwrap().is_empty() {
        return String::from("");
    }

    let mut longest_prefix = strs.first().unwrap().chars().collect::<Vec<char>>();

    for i in 1..strs.len() {
        let mut max_checks = 0;
        let word_to_compare = strs[i].chars().collect::<Vec<char>>();

        for i in 0..word_to_compare.len() {
            if i < longest_prefix.len() && word_to_compare[i] == longest_prefix[i] {
                max_checks = i + 1;
                continue;
            }

            break;
        }

        longest_prefix.truncate(max_checks);

        if longest_prefix.is_empty() {
            return String::from("");
        }
    }

    longest_prefix.into_iter().collect::<String>()
}

fn main() {
    println!("Hello, world!");
    println!("{}", longest_common_prefix_v2(vec!["ab".to_string(), "a".to_string()]));
}
