// https://leetcode.com/problems/single-number/description/
// Easy

use std::collections::{HashSet};


pub fn single_number(nums: Vec<i32>) -> i32 {


    let mut map: HashSet<i32> = HashSet::new();

    for num in nums {

        if map.contains(&num) {
            map.remove(&num);
        } else {
            map.insert(num);
        }
    }

    let mut value = 0;
    for v in map.drain() {
        value = v;
    }

    value
}


fn main() {
    println!("Hello, world!");
}
