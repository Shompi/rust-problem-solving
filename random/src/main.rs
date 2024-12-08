fn main() {
    println!(
        "{}",
        owoify("I'm gonna ride 'til I can't no more".to_string())
    );
    println!(
        "{}",
        owoify("Do you ever feel like a plastic bag".to_string())
    );
    println!("{}", owoify("Cause baby you're a firework".to_string()));
}

fn owoify(phrase: String) -> String {
    let mut owoified_string = String::new();

    for p in phrase.chars() {
        match p {
            'i' => owoified_string.push_str("wi"),
            'e' => owoified_string.push_str("we"),
            _ => owoified_string.push(p),
        }
    }

    owoified_string.push_str(" owo");
    return owoified_string;
}
