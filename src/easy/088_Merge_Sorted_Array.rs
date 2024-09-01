struct Solution;

impl Solution {
    #[allow(unused_variables)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // if nums2.is_empty() {
        //     return;
        // }
        nums1.splice(m as usize.., nums2.drain(..));
        nums1.sort();
    }
}

fn main() {
    let mut tests = [
        (vec![1,2,3,0,0,0], 3, vec![2,5,6], 3),  // [1,2,2,3,5,6]
        (vec![1], 1, vec![], 0),  // [1]
        (vec![0], 0, vec![1], 1),  // [1]
    ];

    for t in tests.iter_mut() {
        print!("{:?} -> ", t.0);
        Solution::merge(&mut t.0, t.1, &mut t.2, t.3);
        println!("{:?}", t.0);
    }

    println!("\nJob done!");
}
