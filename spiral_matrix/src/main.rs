// https://leetcode.com/problems/spiral-matrix/description/
// MEDIUM

pub enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {

    let (
        mut lim_inf, 
        mut lim_sup, 
        mut lim_left, 
        mut lim_right
    ) = (
        -1 as i32,
        matrix.len() as i32,
        -1 as i32,
        matrix[0].len() as i32
    );

    let values = matrix.len() * matrix[0].len();
    let mut direction: Direction = Direction::RIGHT;
    let mut result_vec: Vec<i32> = Vec::with_capacity(values);

    let (
        mut v_pos,
        mut h_pos
    ) = (0, 0);


    while lim_inf < lim_sup && lim_left < lim_right {

        if result_vec.len() == values {
            break;
        }
        
        match direction {
            Direction::RIGHT => {
                if h_pos < lim_right {
                    result_vec.push(matrix[v_pos as usize][h_pos as usize]);
                    h_pos += 1;
                } else {
                    h_pos -= 1;
                    v_pos += 1;
                    lim_inf += 1; // We traversed the first row so we ignore it next time
                    direction = Direction::DOWN;
                    continue;
                }
            },
            Direction::DOWN => {
                if v_pos < lim_sup {
                    result_vec.push(matrix[v_pos as usize][h_pos as usize]);
                    v_pos += 1;
                } else {
                    v_pos -= 1;
                    h_pos -= 1;
                    lim_right -= 1;
                    direction = Direction::LEFT;
                    continue;
                }
            },
            Direction::LEFT => {
                if h_pos > lim_left {
                    result_vec.push(matrix[v_pos as usize][h_pos as usize]);
                    h_pos -= 1;
                } else {
                    h_pos += 1;
                    v_pos -= 1;
                    lim_sup -= 1;
                    direction = Direction::UP;
                }
            },

            Direction::UP => {
                if v_pos > lim_inf {
                    result_vec.push(matrix[v_pos as usize][h_pos as usize]);
                    v_pos -= 1;
                } else {
                    v_pos += 1;
                    h_pos += 1;
                    lim_left += 1;
                    direction = Direction::RIGHT;
                }
            },
        }
    }
    result_vec
}

fn main() {
    println!("Hello, world!");
    spiral_order(vec![vec![1,2,3,4], vec![5,6,7,8], vec![9,10,11,12]]);
}
