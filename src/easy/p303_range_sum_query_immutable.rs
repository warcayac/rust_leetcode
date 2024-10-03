// struct NumArray {  // 2nd implementation
//     nums: Vec<i32>,
// }

// impl NumArray {
//     fn new(nums: Vec<i32>) -> Self {
//         Self { nums }
//     }
    
//     fn sum_range(&self, left: i32, right: i32) -> i32 {
//         let arr = &self.nums[left as usize..=right as usize];
//         arr.iter().sum()
//     }
// }

struct NumArray {  // 1st implementation
    summed: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut summed = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            summed[i + 1] = summed[i] + nums[i];
        }
        Self { summed }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (left, right) = (left as usize, right as usize);
        self.summed[right + 1] - self.summed[left]
    }
}

/*
The first implementation is more efficient due to its use of a precomputed sum array, 
which allows for faster range sum queries. Let's break down the differences:

1. Data Structure:

    - First implementation: Uses two vectors - nums (original array) and summed (precomputed sum array).
    - Second implementation: Uses only one vector - nums (original array).

2. Initialization (new function):

    - First implementation: Creates a summed array where each element is the sum of all previous elements. 
      This is done using fold operation, which has O(n) time complexity.
    - Second implementation: Simply stores the input array. O(1) time complexity.

3. Sum Range Query (sum_range function):

    - First implementation: Calculates the range sum using two lookups in the summed array. 
      This operation has O(1) time complexity.
    - Second implementation: Slices the array and sums all elements in the range. This operation 
      has O(k) time complexity, where k is the size of the range (right - left + 1).

The key efficiency gain comes from the sum_range function:

    - In the first implementation, regardless of the range size, it always performs two array lookups 
      and one subtraction. This is an O(1) operation.
    - In the second implementation, it needs to sum all elements in the given range, which takes time 
      proportional to the range size. This is an O(k) operation, where k can be as large as the entire 
      array size.

The trade-off is that the first implementation uses more memory (O(n) additional space for the summed 
array) and has a slightly more expensive initialization. However, this pays off significantly for multiple 
range sum queries, especially for large arrays or large ranges.

This technique is known as a "prefix sum" or "cumulative sum" array. It's particularly useful when you 
need to perform many range sum queries on a static array, as it allows for constant-time queries at the 
cost of linear extra space and initialization time.
*/

fn main() {
    let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    println!("{}", num_array.sum_range(0, 2)); // 1
    println!("{}", num_array.sum_range(2, 5)); // -1
    println!("{}", num_array.sum_range(0, 5)); // -3

    println!("\nJob done!");
}
