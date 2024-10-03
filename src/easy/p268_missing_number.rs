struct Solution;

impl Solution {
    // Given an array nums containing n distinct numbers in the range [0, n], 
    // return the only number in the range that is missing from the array.
    // pub fn missing_number(nums: Vec<i32>) -> i32 {
    //     let mut nums = nums;
    //     nums.sort_unstable();
    //     if nums[0] != 0 {
    //         return 0;
    //     }
    //     if nums[nums.len()-1] != nums.len() as i32 {
    //         return nums.len() as i32;
    //     }
    //     Self::binary_search(&nums)
    // }

    // fn binary_search(nums: &[i32]) -> i32 {
    //     let mut left = 0;
    //     let mut right = nums.len();
        
    //     while left < right {
    //         let middle = (left + right) / 2;

    //         if nums[middle] == middle as i32 {
    //             left = middle + 1;
    //         } else {
    //             right = middle;
    //         }
    //     }

    //     left as i32
    // }

    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let target = n * (n + 1) / 2;
        target - nums.iter().sum::<i32>()
    }

}

fn main() {
    let tests = [
        vec![3,0,1], // 2
        vec![0,1], // 2
        vec![9,6,4,2,3,5,7,0,1], // 8 , 0 1 2 3 4 5 6 7 9
        vec![8,6,4,2,5,7,0,1,9], // 8 , 0 1 2 4 5 6 7 8 9
        vec![1], // 0
    ];

    for t in tests.into_iter() {
        let result = Solution::missing_number(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
