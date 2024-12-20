pub fn permute(chars: &mut Vec<char>, l: usize, result: &mut Vec<String>) {

    // Base case when the l (counter) reaches the end of the string

    if l == chars.len() {
        result.push(chars.iter().collect());
    } else {
        for i in l..chars.len() {
            chars.swap(l, i);
            permute(chars, l+1, result);
            chars.swap(l, i);
        }
    }

}

fn main() {
    let input = "abcd";
    let mut chars: Vec<char> = input.chars().collect();
    let mut result = Vec::new();

    permute(&mut chars, 0, &mut result);
    for permutation in result {
        println!("{}", permutation);
    }
}
