struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        nums.iter().position(|x| *x >= target).map(|y| y as i32).unwrap_or(nums.len() as i32)
    }
}

fn main() {
    let tests = [
        (vec![1,3,5,6], 5),
        (vec![1,3,5,6], 2),
        (vec![1,3,5,6], 7),
    ];

    for t in tests.iter() {
        let result = Solution::search_insert(t.0.clone(), t.1);
        println!("{:?}: {}", t, result);
    }

    println!("\nJob done!");  
}
