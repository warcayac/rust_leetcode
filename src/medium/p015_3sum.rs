struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        nums.sort_unstable();
        let n = nums.len();
    
        for i in 0..n.saturating_sub(2) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
    
            let (mut left, mut right) = (i + 1, n - 1);
    
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
    
                match sum.cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        results.push(vec![nums[i], nums[left], nums[right]]);
                        while left < right && nums[left] == nums[left + 1] { left += 1; }
                        while left < right && nums[right] == nums[right - 1] { right -= 1; }
                        left += 1;
                        right -= 1;
                    }
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                }
            }
        }
    
        results
    }
}

fn main() {
    let tests = leetcode::data::data_015::TESTS_015.to_vec();

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::three_sum(t.0.to_vec());
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
