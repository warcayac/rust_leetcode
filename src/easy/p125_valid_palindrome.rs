struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.bytes()
            .filter(|b| b.is_ascii_alphanumeric())
            .map(|b| b.to_ascii_lowercase())
            .collect::<Vec<u8>>();
        // let size = bytes.len();
        // let midx = size / 2;

        // for i in 0..midx {
        //     if bytes[i] != bytes[size - i - 1] {
        //         return false;
        //     }
        // }

        // true
        bytes == bytes.iter().copied().rev().collect::<Vec<_>>()
    }
}

fn main() {
    let tests = [
        "A man, a plan, a canal: Panama", // T
        "race a car", // F
        " ", // T
    ];

    for t in tests.into_iter() {
        let result = Solution::is_palindrome(t.to_string());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
