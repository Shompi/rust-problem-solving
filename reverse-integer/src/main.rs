pub fn reverse(x: i32) -> i32 {

    /* Attempt 1 */
    /* Constraints: You cannot store anything bigger than an i32 */
    /* Lets try converting the integer to a string first. */

    let num_string = x.to_string();
    let mut num_vec: Vec<char> = num_string.chars().collect();

    let mut is_negative = false;

    if num_vec[0] == '-' {
        is_negative = true;
        num_vec.remove(0);
    }

    num_vec.reverse();

    let number_string: String = num_vec.into_iter().collect();

    match number_string.parse::<i32>() {
        Err(_) => return 0,
        Ok(num) => {
            match is_negative {
                true => return num * -1,
                false => return num
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("num: {}, reversed: {}", 10, reverse(10));
    println!("num: {}, reversed: {}", 69, reverse(69));
    println!("num: {}, reversed: {}", 123, reverse(123));
    println!("num: {}, reversed: {}", -41, reverse(-41));
    println!("num: {}, reversed: {}", 123456789, reverse(123456789));
    println!("num: {}, reversed: {}", 1534236469, reverse(1534236469));
}
