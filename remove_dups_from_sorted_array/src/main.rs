pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut cursor_index = 1;
    let mut to_replace_index = 0;
    // let mut next_value = nums[0];
    // let mut last_value = nums[0];
    // let mut length = nums.len();
    let mut found_dup: bool = false;
    let mut new_length = 0;

    match nums.len() {
        0 => return 0,
        1 => return 1,
        2 => {
            if nums[0] == nums[1] {
                return 1;
            } else {
                return 2;
            }
        }
        _ => {
            while cursor_index < nums.len() {
                if nums[to_replace_index] != nums[cursor_index] {
                    if nums[to_replace_index] > nums[cursor_index] {
                        cursor_index += 1;
                        continue;
                    }

                    if found_dup {
                        to_replace_index += 1;
                        nums[to_replace_index] = nums[cursor_index];
                        // cursor_index = to_replace_index + 1;
                        new_length += 1;
                        found_dup = false;
                        continue;
                    }

                    to_replace_index += 1;
                    cursor_index += 1;
                    new_length += 1;
                    continue;
                }

                if nums[to_replace_index] == nums[cursor_index] {
                    found_dup = true;
                    cursor_index += 1;

                    if cursor_index >= nums.len() {
                        break;
                    }
                }
            }

            return new_length + 1;
        }
    }
}

fn main() {
    println!("Hello, world!");

    let k = remove_duplicates(&mut vec![1, 2, 2]);
    println!("K = {}", k);
    let k = remove_duplicates(&mut vec![1, 2, 3]);
    println!("K = {}", k);
    let k = remove_duplicates(&mut vec![1, 2, 4, 4, 4, 6, 6, 10]);
    println!("K = {}", k);
    let k = remove_duplicates(&mut vec![4, 5, 6, 6, 6, 6, 7, 8, 9, 9, 9, 9]);
    println!("K = {}", k);
}
