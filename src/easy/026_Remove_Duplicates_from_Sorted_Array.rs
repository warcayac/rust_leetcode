struct Solution;

// impl Solution {
//     pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//         let mut uniques = vec![];
//         let mut num = nums[0];
//         uniques.push(num);
        
//         for n in nums.iter() {
//             if *n != num {
//                 num = *n;
//                 uniques.push(num);
//             }
//         }
//         std::mem::swap(nums, &mut uniques);
//         nums.len() as i32
//     }
// }

/*
In this version, we use two pointers, read_index and write_index, to iterate through the vector. 
We only write to nums[write_index] when we encounter a unique element, which reduces the number of 
unnecessary writes. This approach also avoids the need for an additional vector uniques and the use 
of mem::swap.
This solution has a time complexity of O(n) and a space complexity of O(1), making it more efficient 
than the original code.
*/

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write_index = 1;

        for read_index in 1..nums.len() {
            if nums[read_index] != nums[read_index - 1] {
                nums[write_index] = nums[read_index];
                write_index += 1;
            }
        }

        write_index as i32
    }
}

fn main() {
    let mut tests = [
        vec![1,1,2],
        vec![0,0,1,1,1,2,2,3,3,4],
    ];
    for t in tests.iter_mut() {
        let result = Solution::remove_duplicates(t);
        println!("{:?}: {}", t, result);
    }
    println!("\nJob done!"); 
}
