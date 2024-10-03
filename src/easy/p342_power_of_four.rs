struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // const MAX_POWER_OF_FOUR: i32 = 1073741824; // 4^15
        let size = format!("{:b}", n).len() as u32;
        n > 0 && size % 2 != 0 && n % 2_i32.pow(size - 1) == 0
    }
}

fn main() {
    let tests = [
        16, // true
        5, // false
        2, // false
        1, // true
        8, // false
    ];

    for t in tests.into_iter() {
        let result = Solution::is_power_of_four(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
