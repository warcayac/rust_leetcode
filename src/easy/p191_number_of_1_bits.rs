struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n as u32;
        let mut counter = 0;
        // println!("> {:b}", n);
        
        while n > 0 {
            counter += n & 1;
            n >>= 1;
        }

        counter as i32
    }
}

fn main() {
    let tests = [
        11, // 3
        128, // 1
        2147483645, // 30
    ];

    for t in tests.into_iter() {
        let result = Solution::hamming_weight(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
