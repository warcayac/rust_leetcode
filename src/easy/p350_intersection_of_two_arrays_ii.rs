struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut v1, mut v2) = if nums1.len() < nums2.len() {(nums1, nums2)} else {(nums2, nums1)};
        let mut matches = vec![];
        let mut i = 0;

        v1.sort();
        v2.sort();
        
        for n in v1 {
            while i < v2.len() {
                if v2[i] >= n {
                    if v2[i] == n { 
                        matches.push(n); 
                        i += 1;
                    }
                    break;
                }
                i += 1;
            }
        }

        matches
    }
}

fn main() {
    let tests = [
        (vec![1, 2, 2, 1], vec![2, 2]), // [2, 2]
        (vec![4, 9, 5], vec![9, 4, 9, 8, 4]), // [4, 9]
        (vec![1, 2], vec![1, 1]), // [1]
    ];

    for t in tests.into_iter() {
        let result = Solution::intersect(t.0, t.1);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
