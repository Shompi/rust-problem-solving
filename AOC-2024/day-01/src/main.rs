use std::{collections::HashMap, fs::File, io::Read};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    println!("Opening file...");

    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let _replace = contents.replace("   ", " ");
    let _split: Vec<&str> = _replace.split("\n").collect();

    let mut difference = 0;
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut map: HashMap<i32, i32> = HashMap::new();

    for split in _split {
        let ends: Vec<&str> = split.split(" ").collect();

        if ends[0] == "" {
            continue;
        }

        // println!("ENDS: {:?}", ends);

        let val_left = ends[0].parse().unwrap();
        let val_right = ends[1].parse().unwrap();

        left.push(val_left);
        right.push(val_right);

        // Insert the seen value into the hashmap
        map.insert(val_left, 0);
    }

    left.sort();
    right.sort();

    let mut cursor = 0;
    while cursor < left.len() {
        if left[cursor] < right[cursor] {
            difference += right[cursor] - left[cursor];
        } else {
            difference += left[cursor] - right[cursor];
        }
        cursor += 1;
    }

    let mut similarity_score: i32 = 0;

    for value in left {
        let mut occurrences = 0;

        for r_value in &right {
            if *r_value == value {
                occurrences += 1;
            }
        }

        similarity_score += value * occurrences;

        println!("value {} appears {} times in map", value, occurrences);
    }

    println!("The calculated difference is: {}", difference);
    println!("The similarity score is {}", similarity_score);

    Ok(())
}
