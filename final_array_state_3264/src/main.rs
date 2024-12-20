// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/description/?envType=daily-question&envId=2024-12-16

pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut k = k;
    let mut nums = nums;

    while k > 0 {
        let mut cursor: usize = 0;
        let mut smaller = nums[0];
        let mut smaller_at: usize = 0;

        while cursor < nums.len() {
            if nums[cursor] < smaller {
                smaller = nums[cursor];
                smaller_at = cursor;
            }
            cursor += 1;
        }

        nums[smaller_at] *= multiplier;

        k -= 1;
    }

    nums
}

fn main() {
    println!("Hello, world!");
}
