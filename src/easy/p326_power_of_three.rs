struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        const MAX_POWER_OF_THREE: i32 = 1162261467; // 3^19
        n > 0 && MAX_POWER_OF_THREE % n == 0
    }
    
}

fn main() {
    let tests = [
        27, // true
        0,  // false
        -1, // false
        1,  // true
        25, // false
        243, // true
        225, // false
        441, // false
        1594322, // false
    ];

    for t in tests.into_iter() {
        let result = Solution::is_power_of_three(t);
        println!("=> {} : {:?}", t, result);
    }

    println!("\nJob done!");
}
