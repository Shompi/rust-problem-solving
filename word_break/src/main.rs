/// https://leetcode.com/problems/word-break/
/// MEDIUM

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {

    // Check if the phrase on s can be segmented in to words
    // by using the words in word_dict
    
    // Dynamic Programming

    let mut dp: Vec<bool> = Vec::with_capacity(s.len());
    dp.push(true);

    for _ in 1..dp.capacity() {
        dp.push(false);
    }

    for i in 1..s.len() as i32 {
        for w in word_dict.clone() {
            let start = i - w.chars().count() as i32;
            // let word_vec: Vec<char> = w.chars().into_iter().collect();



            if start >= 0 && *dp.get(start as usize).unwrap() && w.eq(s.get(start as usize..i as usize).unwrap()) {
                dp[i as usize] = true;
                break;
            }
        }
    }

    dp[dp.len() - 1]
}

fn main() {
    println!("Hello, world!");
    println!("{}", word_break("leetcode".to_string(), vec!["leet".to_string(),"code".to_string()]));
    println!("{}", word_break("applepenapple".to_string(), vec!["apple".to_string(),"pen".to_string()]));
    println!("{}", word_break("catsandog".to_string(), vec!["cats".to_string(),"dog".to_string(),"sand".to_string(),"and".to_string(),"cat".to_string()]));
}
