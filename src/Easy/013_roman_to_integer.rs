use std::collections::HashMap;

struct Solution;

struct RomanNumerals {
    map: HashMap<char, u32>,
}

impl RomanNumerals {
    fn new() -> RomanNumerals {
        let mut map = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);
        RomanNumerals { map }
    }

    fn get(&self, ch: char) -> Option<&u32> {
        self.map.get(&ch)
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_numerals = RomanNumerals::new();
        let values: Vec<u32> = s
            .chars()
            .map(|ch| *roman_numerals.get(ch).unwrap())
            .collect();
        let len = values.len();
        let mut sum: u32 = 0;

        for i in 0..len {
            if i < len - 1 && values[i] < values[i + 1] {
                sum -= values[i];
            } else {
                sum += values[i];
            }
        }

        sum as i32
    }
}

fn main() {
    for n in ["III","LVIII","MCMXCIV"].iter() {
        let number = Solution::roman_to_int(n.to_string());
        println!("{}: {}", n, number);
    }
    println!("\nJob done!");
}
