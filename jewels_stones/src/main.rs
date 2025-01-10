use std::collections::HashMap;

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {

    let mut jewels_map: HashMap<char, i32> = HashMap::new();

    for jewel in jewels.chars() {
        jewels_map.insert(jewel, 0);
    }

    for stone in stones.chars() {

        match jewels_map.get(&stone) {
            Some(v) => {
                jewels_map.insert(stone, *v + 1);
            },
            None => ()
        }
    }

    jewels_map.values().sum()
}

fn main() {
    println!("Hello, world!");
}
