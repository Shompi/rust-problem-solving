
pub fn reverse_string(s: &mut Vec<char>) {
    
    let mut cursor = 0;
    let len = s.len() - 1;
    let half = s.len() / 2;
    
    loop {
        if cursor >= half{
            break;
        }

        let aux = s[len - cursor];

        s[len - cursor] = s[cursor]; 
        s[cursor] = aux;
        cursor +=1;
    }
}
fn main() {
    println!("Hello, world!");
    let mut st: Vec<char> = vec!['a','b','c','d','e',];
    reverse_string(&mut st);
    println!("{:?}", st);
}
