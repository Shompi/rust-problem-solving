/// https://leetcode.com/problems/word-search/
/// MEDIUM

struct Point {
    x: usize,
    y: usize
}

pub fn check_neighbors(position: Point, board: Vec<Vec<char>>) {





}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {


    let mut board = board.clone();
    let word: Vec<char> = word.chars().collect();

    let mut row: usize = 0;
    let mut col: usize = 0;

    while row < board.len() {
        
        while col < board[row].len() {


            // Search for first occurence of first letter
            
            if board[row][col] == word[0] {
                // if the letter at [row][col] == first letter of the target word we have a coincidence

                // Check if we have a neighbor that matches the next character



            }


            col += 1;
        }

        row += 1;
    }

    todo!()
}

fn main() {
    println!("Hello, world!");
}
