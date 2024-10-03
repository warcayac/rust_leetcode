struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        const BASE: u8 = 64; // A:65 , Z:90, 26 chars in ASCII
        let divisor = (b'Z' - b'A' + 1) as u32;
        let mut dividend = column_number as u32;
        let mut remainders: Vec<u8> = Vec::new();

        while dividend > divisor {
            let mut remainder = dividend % divisor;
            let mut quotient = dividend / divisor;
            if remainder == 0 {
                remainder = divisor;
                quotient -= 1;
            }
            remainders.push(remainder as u8 + BASE);
            dividend = quotient;
        }
        remainders.push(dividend as u8 + BASE);
        remainders.into_iter().rev().map(|n| n as char).collect::<String>()
    }

}

fn main() {
    let tests = [
        1, // A
        28, // AB
        701, // ZY
        52, // AZ
    ];

    for t in tests.into_iter() {
        let result = Solution::convert_to_title(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
