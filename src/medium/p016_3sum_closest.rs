struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        // Sort the input array
        nums.sort_unstable();
        // Initialize closest sum
        let mut closest_sum = nums[0] + nums[1] + nums[2];
        // println!("closest_sum: {:?}", closest_sum);
        // Iterate through the array
        for i in 0..nums.len() - 2 {
            // Initialize two pointers
            let (mut left, mut right) = (i+1, nums.len()-1);

            while left < right {
                // Calculate the sum of three elements
                let sum = nums[i] + nums[left] + nums[right];
                // println!("n[{i}]: {}, n[{left}]: {}, n[{right}]: {}, sum: {}", nums[i], nums[left], nums[right], sum);
                // Update closest sum if necessary
                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                    // println!("closest_sum: {:?}", closest_sum);
                }
                // Move pointers based on sum
                match sum.cmp(&target) {
                    std::cmp::Ordering::Greater => right -= 1,
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Equal => {
                        // Exact match found, return target
                        return target;
                    }
                }
            }
        }

        // Return closest sum
        closest_sum
    }
}

fn main() {
    let tests = [
        ((vec![-1,2,1,-4], 1), 2),
        ((vec![0,0,0], 1), 0),
        ((vec![-1, 2, 1, -4, 5, 3], 1), 1),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::three_sum_closest(t.0.0.to_vec(), t.0.1);
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
