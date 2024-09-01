// use std::collections::HashMap;

struct Solution;

impl Solution {
    // pub fn single_number(nums: Vec<i32>) -> i32 {
    //     let mut freqs: HashMap<i32,u16> = HashMap::new();
        
    //     nums.into_iter().for_each(|n| *freqs.entry(n).or_insert(0) += 1);
    //     let (k,_) = freqs.iter().min_by_key(|(_,v)| **v).unwrap();

    //     *k
    // }
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res : i32 = 0 ;
        for n in nums{
            res ^= n ;
        }
        res
    }
/*
Let's go through the iterations with the input vector nums = [2, 3, 7, 3, 2]:
    1) Initially, res = 0.
    2) First iteration: res ^= 2 => res = 0 ^ 2 = 2.
    3) Second iteration: res ^= 3 => res = 2 ^ 3 = 1 (because 2 and 3 have different bits, the result is 1).
    4) Third iteration: res ^= 7 => res = 1 ^ 7 = 6 (because 1 and 7 have different bits, the result is 6).
    5) Fourth iteration: res ^= 3 => res = 6 ^ 3 = 5 (because 6 and 3 have different bits, the result is 5).
    6) Fifth iteration: res ^= 2 => res = 5 ^ 2 = 7 (because 5 and 2 have different bits, the result is 7).

So, the final result is res = 7, which is the single number that appears only once in the input vector.

Here's a summary of the XOR operations:
    2 and 2 cancel out
    3 and 3 cancel out
    7 is left alone, because it appears only once
Therefore, the output is 7, which is the single number in the vector.
*/
}

fn main() {
    let tests = [
        vec![2,2,1], // 1
        vec![4,1,2,1,2], // 4
        vec![1], // 1
    ];

    for t in tests.into_iter() {
        let result = Solution::single_number(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
