/* MEDIUM */
// https://leetcode.com/problems/adding-spaces-to-a-string/?envType=daily-question&envId=2024-12-18

pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {

    let mut s_vec: Vec<char> = Vec::with_capacity(s.chars().count() + spaces.len());
    let string_vec: Vec<char> = s.chars().collect();
    // let mut spaces = spaces;
    let mut string_cursor = 0;
    let mut spaces_cursor = 0;
    let mut cursor: usize = 0;
    let mut shift_by: usize = 0;

    loop {

        if cursor >= s_vec.capacity() {
            break;
        }

        if spaces_cursor >= spaces.len() {
            while cursor < s_vec.capacity() {
                s_vec.insert(cursor, string_vec[string_cursor]);
                // string_cursor += 1;
                cursor += 1;
                string_cursor += 1;
            }
            break;
        }

        if (spaces[spaces_cursor] + shift_by as i32) as usize != cursor {
            
            s_vec.push(string_vec[string_cursor]);

            // s_vec[cursor + shift_by] = string_vec[string_cursor];
            string_cursor += 1;
        } else {
            s_vec.insert(cursor, ' ');
            // s_vec[string_cursor] = ' ';
            spaces_cursor += 1;
            shift_by += 1;
        }

        cursor += 1;
    
    }

    String::from_iter(s_vec.into_iter())
}

fn main() {
    println!("Hello, world!");
    add_spaces(String::from("LeetcodeHelpsMeLearn"), vec![8,13,15]);
}
