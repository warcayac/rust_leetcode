struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = if a.len() >= b.len() { (a, b) } else { (b, a) };
        let mut result = Vec::with_capacity(a.chars().count() + 1);
        let [mut a, mut b] = [&a, &b].map(|s| s.chars().rev());
        let mut carry: u8 = 0;

        for x in a.by_ref() {
            // print!("carry: {}, x: {}, ", carry, x);
            let mut digit = x.to_digit(2).unwrap() as u8 + carry;
            if let Some(y) = b.next() {
                // print!("y: {}, ", y);
                digit += y.to_digit(2).unwrap() as u8;
            } else if carry == 0 {
                // println!("d: {} ... break", digit % 2);
                result.push(x);
                break;
            }
            // println!("d: {}", digit % 2);
            result.push(char::from(b'0' + (digit % 2)));
            carry = digit / 2;
        }

        if carry > 0 {
            result.push('1');
        }
        result.extend(a);
        result.iter().rev().collect::<String>()
    }
}

fn main() {
    let tests = [
        ("11", "1"),    // 100
        ("1010", "1011"),   // 10101
        ("1010", "10111100"),   // 11000110
        ("1010", "10010111100"),   // 10011000110
        ("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101",
        "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011")
    ].iter().map(|s| (s.0.to_string(), s.1.to_string())).collect::<Vec<(String,String)>>();

    for t in tests.iter() {
        let result = Solution::add_binary(t.0.clone(), t.1.clone());
        println!("{:?}: {}", t, result);
    }

    println!("\nJob done!");
}
