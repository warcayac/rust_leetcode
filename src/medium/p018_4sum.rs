struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let n = nums.len();
        if n < 4 { return result; }
        let mut nums = nums;
        nums.sort_unstable();

        for i in 0..n-3 {
            if i > 0 && nums[i] == nums[i-1] { continue; }

            Self::find_three_sum(&nums[i+1..], target - nums[i])
                .into_iter()
                .for_each(|mut v| {
                    v.insert(0, nums[i]);
                    result.push(v);
                })
            ;
        }
        
        result
    }

    fn find_three_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i-1] { continue; }
            let (mut left, mut right) = (i+1, nums.len()-1);

            while left < right {
                let sum = nums[i].saturating_add(nums[left]).saturating_add(nums[right]);
                match sum.cmp(&target) {
                    std::cmp::Ordering::Greater => right -= 1,
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Equal => {
                        let left_val = nums[left];
                        let right_val = nums[right];
                        result.push(vec![nums[i], left_val, right_val]);
                        while left_val == nums[left] && left < right { left += 1; }
                        while right_val == nums[right] && left < right { right -= 1; }
                    },
                }
            }
        }
        result
    }
}

fn main() {
    let tests = [
        ((vec![1,0,-1,0,-2,2], 0), vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]]),
        ((vec![2,2,2,2,2], 8), vec![vec![2,2,2,2]]),
        ((vec![-2,-1,-1,1,1,2,2], 0), vec![vec![-2,-1,1,2],vec![-1,-1,1,1]]),
        ((vec![1000000000,1000000000,1000000000,1000000000], -294967296), vec![]),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::four_sum(t.0.0, t.0.1);
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
