pub fn longest_common_prefix(strs: Vec<String>) -> String {

    let mut  _longest: Vec<char> = strs[0].chars().collect();

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

fn main() {
    println!("Hello, world!");
}