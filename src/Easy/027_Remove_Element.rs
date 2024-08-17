struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut write_index = 0;

        for read_index in 0..nums.len() {
            if nums[read_index] != val {
                if write_index != read_index {
                    nums[write_index] = nums[read_index];
                }
                write_index += 1;
            }
        }

        write_index as i32
    }
}

fn main() {
    let mut tests = [
        (vec![3,2,2,3], 3),
        (vec![0,1,2,2,3,0,4,2], 2),
    ];
    for t in tests.iter_mut() {
        let result = Solution::remove_element(&mut t.0, t.1);
        println!("{:?}: {}", t, result);
    }
    println!("\nJob done!"); 
}
