struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        const ROMAN_SYMBOLS: [char; 9] = ['I','V','X','L','C','D','M','?','?'];
        let mut num = num;
        let mut idx = 0;
        let mut result = String::new();

        while num > 0 {
            result.insert_str(0, &Self::roman_from_digit(num % 10, &ROMAN_SYMBOLS[idx..idx+3]));
            num /= 10;
            idx += 2;
        }

        result
    }

    fn roman_from_digit(digit: i32, symbols: &[char]) -> String {
        let (one, five, ten) = (symbols[0], symbols[1], symbols[2]);
        let mut digit = digit;
        let mut result = String::new();
        
        loop {
            match digit {
                0 => break,
                4 => result = format!("{}{}", one, five),
                9 => result = format!("{}{}", one, ten),
                a if a >= 5 => {
                    result.push(five);
                    digit -= 5;
                    continue;
                },
                a => result = format!("{}{}", result, one.to_string().repeat(a as usize)),
            }
            break;
        }

        result
    }
}

fn main() {
    let tests = [
        (3749, "MMMDCCXLIX"),
        (58, "LVIII"),
        (1994, "MCMXCIV"),
        (3999, "MMMCMXCIX"),
        (23, "XXIII"),
        (3, "III"),
        (30, "XXX"),
        (3000, "MMM"),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::int_to_roman(t.0);
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
