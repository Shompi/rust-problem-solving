// https://leetcode.com/problems/merge-intervals/description/
// MEDIUM
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    println!("Intervals: {:?}", intervals);

    let mut intervals = intervals;
    intervals.sort_by(|x, y| x[0].cmp(&(y[0])));
    println!("Sorted Int: {:?}", intervals);


    let mut merged: Vec<Vec<i32>> = vec![];


    for interval in intervals {


        if merged.is_empty() {
            merged.push(interval);
            continue;
        }

        match merged.last() {
            Some(v) => {

                let (x1, x2) = (v[0], v[1]);
                let (x3,x4) = (interval[0], interval[1]);

                if (x3 >= x1 && x3 <= x2) || (x4 <= x2 && x4 >= x1) {
                    // this is overlapping on one or both ends

                    // We need to extend the current overlap

                    merged.pop();
                    merged.push(vec![
                        std::cmp::min(x1, x3),
                        std::cmp::max(x2, x4),
                    ]);

                } else {
                    merged.push(interval);
                }
            },
            None => ()
        }
    }
    println!("Merged: {:?}", merged);
    merged
}

fn main() {
    println!("Hello, world!");
    merge(vec![
        vec![4,5],
        vec![1,3],
        vec![1,4],
        vec![8,10],
        vec![2,6],
        vec![2,6],
        vec![15,18],
    ]);
}
