struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut pascal_triangle: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
        
        for size in 1..=num_rows {
            let mut cur_row: Vec<i32> = Vec::with_capacity(size as usize);
            
            for i in 0..size {
                if [0, size-1].contains(&i) {
                    cur_row.push(1);
                } else {
                    let previous_row = pascal_triangle.last().unwrap();
                    cur_row.push(previous_row[(i-1) as usize] + previous_row[i as usize]);
                }
            }

            pascal_triangle.push(cur_row);
        }
        
        pascal_triangle
    }
}

fn main() {
    let tests = [
        5, // [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
        1, // [[1]]
        6, // [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1],[1,5,10,10,5,1]]
    ];

    for t in tests.into_iter() {
        let result = Solution::generate(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
