// https://leetcode.com/problems/maximum-distance-in-arrays/description/?envType=daily-question&envId=2024-12-16

struct Pair {
    /* The value that holds this pair */
    value: i32,
    /* In which array was this value found */
    found_at: usize
}

pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {


    let mut lowest = Pair {
        value: arrays[0][0],
        found_at: 0,
    };

    let mut highest = Pair {
        value: arrays[0][0],
        found_at: 0,
    };
    
    let mut cursor: usize = 0;

    // let max_distance = 0;

    // Note each array inside arrays is sorted in ASCENDING order.
    while cursor < arrays.len() {
        
        let inner_array = &arrays[cursor];

        if inner_array[0] < lowest.value {
            lowest.value = inner_array[0];
            // lowest.index = cursor;
        }

        if inner_array[inner_array.len() - 1] > highest.value {
            highest.value = inner_array[inner_array.len() - 1];
        }

        cursor += 1;
    }

    (lowest.value - highest.value).abs()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_one() {
        assert_eq!(
            max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]),
            4
        );
    }
    #[test]
    fn case_two() {
        assert_eq!(
            max_distance(vec![vec![1], vec![1]]),
            0
        );
    }
}
