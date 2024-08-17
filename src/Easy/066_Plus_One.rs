struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = digits.clone();
        for i in (0..result.len()).rev() {
            if result[i] < 9 {
                result[i] += 1;
                return result;
            }
            result[i] = 0;
        }
        result.insert(0, 1);
        result
    }
}

fn main() {
    let tests = [
        vec![1,2,3],
        vec![4,3,2,1],
        vec![9],
    ];

    for t in tests.iter() {
        let result = Solution::plus_one(t.clone());
        println!("{:?}: {:?}", t, result);
    }

    println!("\nJob done!");  
}
