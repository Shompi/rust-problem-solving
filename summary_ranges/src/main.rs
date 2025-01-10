// https://leetcode.com/problems/summary-ranges/description/
// EASY
// Arrays

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }

    if nums.len() == 1 {
        return vec![nums[0].to_string()];
    }

    // The array is sorted and its values are unique

    let mut cursor: usize = 0;
    let mut start: usize = 0;
    let mut end: usize = cursor;
    let mut ranges: Vec<String> = vec![];

    loop {
        if cursor + 1 == nums.len() {
            if end == start {
                ranges.push(format!("{}", nums[end]));
            } else {
                ranges.push(format!("{}->{}", nums[start], nums[end]));
            }

            break;
        }

        if nums[cursor] + 1 == nums[cursor + 1] {

            end = cursor + 1;
            cursor += 1;
            continue;

        } else {
            end = cursor;

            if end == start {
                ranges.push(format!("{}", nums[end]));
            } else {
                ranges.push(format!("{}->{}", nums[start], nums[end]));
            }
        }
        cursor += 1;
        start = cursor;
        end = start;
    }

    ranges
}

fn main() {
    println!("Hello, world!");
    println!(
        "Ranges: {:?}",
        summary_ranges(vec![1, 2, 3, 5, 6, 7, 10, 12, 14])
    );
}
