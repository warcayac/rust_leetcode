struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // TODO: Whitespace: Ignore any leading whitespace (" ").
        let s = s.trim_start();
        // TODO: Signedness: Determine the sign by checking if the next character is '-' or '+', assuming positivity is neither present.
        let is_negative = s.starts_with('-');
        // TODO: Conversion: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
        let skip_value = if ['-','+'].into_iter().any(|sign| s.starts_with(sign)) { 1 } else { 0 };
        let result = s
            .chars()
            .skip(skip_value)
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<i32>()
        ;
        // TODO: Rounding: If the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then round the integer to remain in the range. Specifically, integers less than -2^31 should be rounded to -2^31, and integers greater than 2^31 - 1 should be rounded to 2^31 - 1.        
        match result {
            Ok(result) => if !is_negative { result  } else { result.saturating_mul(-1) },
            Err(e) => match e.kind() {
                std::num::IntErrorKind::PosOverflow => if is_negative { i32::MIN } else { i32::MAX },
                _ => 0
            }
        }
    }
}

fn main() {
    let tests = [
        ("42", 42),
        ("   -042", -42),
        ("1337c0d3", 1337),
        ("0-1", 0),
        ("words and 987", 0),
        ("+234", 234),
        ("3147483647", 2_147_483_647),
        ("-3147483647", -2_147_483_648),
        ("+-12", 0),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::my_atoi(t.0.to_owned());
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
