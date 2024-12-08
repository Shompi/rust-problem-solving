pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {

    let mut cursor = 0;
    // let mut removed = 0; // Initialize it to the last on the array

    if nums.len() == 0 {
        return 0;
    }

    while cursor < nums.len() {

        if nums[cursor] == val {
            nums.remove(cursor);
            // removed += 1;
            continue;
        }

        cursor +=1;
    }

    nums.len() as i32
} // pito


fn main() {
    println!("Hello, world!");
}
