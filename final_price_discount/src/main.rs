pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let mut cursor = 0;
    let mut final_vector: Vec<i32> = vec![];

    while cursor < prices.len() {
        let mut next = cursor + 1;
        let i_value = prices[cursor];
        let mut found = false;

        loop {
            if next >= prices.len() {
                break;
            }

            if prices[next] <= i_value {
                found = true;
                break;
            }
            next += 1;
        }
        if found {
            final_vector.push(i_value - prices[next]);
        } else {
            final_vector.push(prices[cursor]);
        }

        cursor += 1;
    }

    final_vector
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", final_prices(vec![8, 4, 6, 2, 3]));
    println!("{:?}", final_prices(vec![1, 2, 3, 4, 5]));
    println!("{:?}", final_prices(vec![10, 1, 1, 6]));
}
