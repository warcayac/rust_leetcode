use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut uniques = HashSet::new();
        !nums.into_iter().all(|n| uniques.insert(n))
    }
}

fn main() {
    let tests = [
        vec![1,2,3,1], // T
        vec![1,2,3,4], // F
        vec![1,1,1,3,3,4,3,2,4,2], // T
    ];

    for t in tests.into_iter() {
        let result = Solution::contains_duplicate(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
