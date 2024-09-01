use leetcode::utils::combinatorics;

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let size = row_index as u32 + 1;
        let mut row: Vec<i32> = Vec::with_capacity(size as usize);
        let midx = (size -1) / 2;
        let mut dec_idx = if size % 2 == 0 { 1 } else { 2 };
        let n = size - 1;
        

        for i in 0..size {
            if [0, size-1].contains(&i) {
                row.push(1);
            } else if i > midx {
                row.push(row[(i - dec_idx) as usize]);
                dec_idx += 2;
            } else {
                row.push(combinatorics::combinatorial_number(n, i) as i32);
            }
        }
        
        row
    }
}

fn main() {
    let tests = [
        3, // [1,3,3,1]
        0, // [1]
        1, // [1,1]
        4, // [1,4,6,4,1]
        5, // [1,5,10,10,5,1]
        6, // [1,6,15,20,15,6,1]
    ];

    for t in tests.into_iter() {
        let result = Solution::get_row(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
