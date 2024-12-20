/**
You are given a large integer represented as an integer array digits, where each digits[i]
is the ith digit of the integer.
The digits are ordered from most significant to least significant in left-to-right order.
The large integer does not contain any leading 0's.
Increment the large integer by one and return the resulting array of digits.

 */

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {

    let mut cursor:i32 = digits.len() as i32 - 1; // Start from the end
    let mut carry_over: i32 = 0;
    let plus_one = 1;
    let mut result: Vec<i32> = vec![];

    if digits[cursor as usize] + plus_one >= 10 {
        carry_over = 1;
        result.push(digits[cursor as usize] + 1 - 10);

        cursor -= 1;
        while carry_over >= 1 && cursor >= 0 {
            let sum = digits[cursor as usize] + carry_over;
            
            carry_over = 0;

            if sum >= 10 {
                carry_over = 1;
                result.push(sum - 10);
            }

            if carry_over == 0 {
                result.push(sum);
            }
            cursor -= 1;

        }

        if carry_over == 1 {
            result.push(carry_over);
        }

    } else {
        result = digits.clone();

        result[digits.len() - 1 ] += 1;

        return result;
    }

    while cursor >= 0 {
        result.push(digits[cursor as usize]);
        cursor -= 1;
    }

    result.reverse();

    result
}

fn main() {
    println!("Hello, world!");
}
