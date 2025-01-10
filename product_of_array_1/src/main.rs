// https://leetcode.com/problems/product-of-array-except-self/description/
// MEDIUM

/* pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // This solution runs into a TLE with a test case of about a million numbers

    let mut products: Vec<i32> = vec![0; nums.len()];

    for to_ignore in 0..nums.len() {

        let mut cursor = 0;
        let mut result = 1;

        while cursor < nums.len() {

            if cursor == to_ignore {
                cursor += 1;
                continue;
            }

            result *= nums[cursor];

            if result == 0 {
                break;
            }

            cursor += 1;
        }

        products[to_ignore] = result;
    }

    products
} */

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let size = nums.len();
    let mut products = vec![0; size];

    products[0] = 1;

    let mut prefix = 1;
    let mut suffix = 1;

    for idx in 0..size - 1 {
        prefix = prefix * nums[idx];
        products[idx + 1] = prefix;
    }

    for idx in (0..size).rev() {
        products[idx] = products[idx] * suffix;
        suffix = suffix * nums[idx];
    }

    products
}

fn main() {
    println!("Hello, world!");
}
