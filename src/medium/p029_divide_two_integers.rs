struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        match (dividend, divisor) { 
            (_, 0) => panic!("Division by zero"),
            (0, _) => 0,
            (_, 1) => dividend,
            (dd, -1) => match dd == i32::MIN {
                true => i32::MAX,
                false => -dividend,
            },
            (dd, dr) if dd != i32::MIN && dd.abs() == dr.abs() => match (dd.is_positive(), dr.is_positive()) {
                (true, true) | (false, false) => 1,
                _ => -1,
            },
            _ => {
                let is_positive = (dividend.is_positive() && divisor.is_positive()) || 
                    (dividend.is_negative() && divisor.is_negative());
                let dividend = match dividend == i32::MIN {
                    true => i32::MAX as u32 + 1,
                    false => dividend.unsigned_abs(),
                };
                let divisor = divisor.unsigned_abs();

                if dividend < divisor { return 0; }

                let mut quotient = 0;
                let mut remainder = 0;
                // Process 32 bits
                for i in (0..32).rev() {
                    remainder = (remainder << 1) | ((dividend >> i) & 1);
                    
                    if remainder >= divisor {
                        remainder = remainder.wrapping_sub(divisor);
                        quotient |= 1 << i;
                    }
                }

                if is_positive { quotient } else { -quotient }
            },
        }
    }
}

fn main() {
    let tests = [
        ((10,3), 3),
        ((7,-3), -2),
        ((-2147483648, -1), 2147483647),
        ((-2147483648, 1), -2147483648),
        ((2_147_483_647, 2), 1_073_741_823),
        ((-2_147_483_648, 2), -1_073_741_824),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::divide(t.0.0, t.0.1);
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
