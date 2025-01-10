// https://leetcode.com/problems/rotate-image/
// MEDIUM

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {

    // Must modify the matrix in place

    // Traverse the matrix through its diagonal and swap the top elements with the bottom ones

    let length = matrix.len(); // We only need this one since its a square matrix everytime
    let mut swapper: i32;

    for i in 0..length {

        //[i][i] == Matrix Diagonal

        for j in i..length {


            if i == j {
                continue; // skip this iteration
            }

            swapper = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = swapper;
        }
    }

    // then reverse each vector
    matrix.into_iter().map(|vector| vector.reverse()).collect()
}

fn main() {
    println!("Hello, world!");
    let mut matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    rotate(&mut matrix);
    println!("{:?}", matrix);
}
