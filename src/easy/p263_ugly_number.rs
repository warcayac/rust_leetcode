struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 { return false; }
        if n == 1 { return true; }
        let mut dividend = n;

        for divisor in [2,3,5].into_iter() {
            loop {
                if dividend % divisor == 0 {
                    dividend /= divisor;
                } else {
                    break;
                }
            }
        }

        dividend == 1
    }
}

fn main() {
    let tests = [
        6, // T
        1, // T
        14, // F
    ];

    for t in tests.into_iter() {
        let result = Solution::is_ugly(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
