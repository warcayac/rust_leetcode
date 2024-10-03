struct Solution;

impl Solution {
    // pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    //     use std::collections::HashSet;

    //     let mut uniques = HashSet::new();
    //     let k = k as usize;

    //     for i in 0..nums.len() {
    //         if uniques.insert(nums[i]) { continue; }
    //         let lower_idx = if i < k { 0 } else { i - k };
    //         for j in lower_idx..i {
    //             if nums[j] == nums[i] { return true; }
    //         }
    //     }

    //     false
    // }

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut last_seen:HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        
        nums
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| 
                last_seen
                    .insert(v, i) // retorna el valor previo valor si existía, caso contrario es None
                    .map(|d| (i - d) as i32)  // si el previo valor no es None entonces...
            )
            .any(|x| x <= k)
    }
}

fn main() {
    let tests = [
        (vec![1,2,3,1], 3), // T
        (vec![1,0,1,1], 1), // T
        (vec![1,2,3,1,2,3], 2), // F
        (vec![1,0,1,1], 2), // T
    ];

    for t in tests.into_iter() {
        let result = Solution::contains_nearby_duplicate(t.0, t.1);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
