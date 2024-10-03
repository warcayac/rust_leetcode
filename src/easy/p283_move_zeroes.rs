struct Solution;

impl Solution {
    // pub fn move_zeroes(nums: &mut Vec<i32>) {
    //     let initial_size = nums.len();
    //     nums.retain(|&e| e != 0);
    //     nums.extend(vec![0; initial_size - nums.len()]);
    // }

    #[allow(clippy::ptr_arg)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut p = 0;
        for idx in 0..len {
            if nums[idx] != 0 {
                nums.swap(p, idx);
                p += 1;
            }
        }
    }

}

fn main() {
    let tests = [
        vec![0,1,0,3,12], // [1,3,12,0,0]
        vec![0], // [0]
    ];

    for t in tests.into_iter().as_mut_slice() {
        Solution::move_zeroes(t);
        println!("=> {:?}", t);
    }

    println!("\nJob done!");
}
