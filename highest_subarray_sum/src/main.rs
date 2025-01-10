pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_ending = nums[0];
    let mut max_sum = nums[0];

    for i in 1..nums.len() {
        max_ending = nums[i].max(max_ending + nums[i]);

        max_sum = max_sum.max(max_ending);
    }

    max_sum
}

fn main() {
    println!("Hello, world!");
}
