// https://leetcode.com/problems/house-robber/
// Medium
pub fn rob(nums: Vec<i32>) -> i32 {

    let n = nums.len();
    let mut dp: Vec<i32> = vec![0; n];


    if n == 1 { return nums[0] }

    dp[0] = nums[0];
    dp[1] = nums[0].max(nums[1]);

    let mut cursor = 2;

    while cursor < n {

        dp[cursor] = dp[cursor - 1].max(nums[cursor] + dp[cursor - 2]);
        cursor += 1;
    }

    dp[n - 1]
}

fn main() {
    println!("Hello, world!");
    println!("Robbed: {}", rob(vec![1,2,3,4,5]));
    // println!("Robbed: {}", rob(vec![]));
    println!("Robbed: {}", rob(vec![5,10,3,5,6,10]));
}
