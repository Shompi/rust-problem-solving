/* https://leetcode.com/problems/check-if-n-and-its-double-exist/description */

pub fn check_if_exist(arr: Vec<i32>) -> bool {
    // let arr_length = arr.len();
    // let mut comparator = 0;
    // let mut index_i = 0;
    // let mut index_j = 0;

    for (index_i, value_i) in arr.iter().enumerate() {
        for (index_j, value_j) in arr.iter().enumerate() {
            if index_i == index_j {
                continue;
            }

            if *value_i == *value_j * 2 {
                return true
            }
        }
    }

    false
}

fn main() {
    println!("Hello, world!");
}
