use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let s1 : HashSet<i32> = HashSet::from_iter(nums1);
        let s2 : HashSet<i32> = HashSet::from_iter(nums2);
        s1.intersection(&s2).copied().collect::<Vec<_>>()
    }
}

fn main() {
    let tests = [
        (vec![1, 2, 2, 1], vec![2, 2]), // [2]
        (vec![4, 9, 5], vec![9, 4, 9, 8, 4]), // [9, 4]
    ];

    for t in tests.into_iter() {
        let result = Solution::intersection(t.0, t.1);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
