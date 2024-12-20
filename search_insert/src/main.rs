pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    

    let mut pivot = nums.len() / 2;
    let mut left_side: i32 = 0;
    let mut right_side: i32 = nums.len() as i32;

    if nums.len() == 0 {
        return 0;
    }

    if nums.len() == 1 {
        if target > nums[0] {
            return 1;
        } else {
            return 0;
        }
    }

    while nums[pivot] != target {

        if target > nums[pivot] {
            left_side = pivot as i32;

        } else if target < nums[pivot] {
            right_side = pivot as i32;
        }

        pivot = ((left_side + right_side) / 2) as usize;

        if nums[pivot] == target {
            // Instert it to the left
            if (pivot) == 0 {
                return 0;
            } else {
                return pivot as i32;
            }
        }

        if right_side - left_side == 1 {
            if target < nums[pivot] {
                break;
            } else {
                pivot += 1;
            }
            break;
        }
    }

    pivot as i32

}

fn main() {
    println!("Hello, world!");
    println!("result: {}", search_insert(vec![1,3,5,6], 5));
    println!("result: {}", search_insert(vec![1,3,5,6], 2));
    println!("result: {}", search_insert(vec![1,3,5,6], 7));
    println!("result: {}", search_insert(vec![1,3], 1));
}