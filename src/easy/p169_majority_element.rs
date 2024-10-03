// use std::collections::HashMap;

struct Solution;

impl Solution {
    // pub fn majority_element(nums: Vec<i32>) -> i32 {
    //     let mut freqs: HashMap<i32,u16> = HashMap::new();
        
    //     nums.into_iter().for_each(|n| *freqs.entry(n).or_insert(0) += 1);
    //     let (k,_) = freqs.iter().max_by_key(|(_,v)| **v).unwrap();

    //     *k
    // }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // Boyer-Moore majority vote algorithm
        let mut candidate = 0;
        let mut counter = 0;
        for num in nums {
            if counter == 0 {
                candidate = num;
                counter = 1;
            } else if candidate == num {
                counter += 1;
            } else {
                counter -= 1;
            }
        }
        candidate
    }
}

fn main() {
    let tests = [
        vec![3,2,3], // 3
        vec![2,2,1,1,1,2,2], // 2
        vec![1], // 1
    ];

    for t in tests.into_iter() {
        let result = Solution::majority_element(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
