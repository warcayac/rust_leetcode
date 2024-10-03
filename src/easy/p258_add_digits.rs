struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        const DIVISOR : i32 = 10;
        let mut sum = num;

        while sum >= DIVISOR {
            let mut digits = vec![];
            let mut dividend = sum;
            loop {
                digits.push(dividend % DIVISOR);
                dividend /= DIVISOR;
                if dividend < DIVISOR { 
                    digits.push(dividend);
                    break; 
                }
            }
            sum = digits.into_iter().sum();
        }
        
        sum
    }
}

fn main() {
    let tests = [
        38, // 2
        0, // 0
    ];

    for t in tests.into_iter() {
        let result = Solution::add_digits(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
