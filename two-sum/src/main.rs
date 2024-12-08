// https://leetcode.com/problems/two-sum/description/
// Runtime 12ms.
// This solution beats 37.71%

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // let mut sum = 0;
    let mut index_i = 0;
    let mut _index_j = 0;

    while index_i < nums.len() {
        _index_j = index_i + 1;

        loop {
            
            if _index_j == nums.len() {
                break;
            }

            if nums[index_i] + nums[_index_j] == target {
                return vec![
                    index_i as i32,
                    _index_j as i32,
                ];
            }
            _index_j += 1;
        }
        index_i += 1;
    }

    panic!("You shouldn't have reached this point.");
}

fn main() {
    println!("Hello, world!");
}
