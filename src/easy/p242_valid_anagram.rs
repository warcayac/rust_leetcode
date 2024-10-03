use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.chars().count() != t.chars().count() {
            return false;
        }
        
        let mut counter = HashMap::new();
        let (mut s, mut t) = (s.chars(), t.chars());
        
        while let (Some(x), Some(y)) = (s.next(), t.next()) {
            *counter.entry(x).or_insert(0) += 1;
            *counter.entry(y).or_insert(0) -= 1;
        }
        
        return counter.values().all(|&x| x == 0)
    }
}

fn main() {
    let tests = [
        ("anagram", "nagaram"), // T
        ("rat", "car"), // F
        ("a", "ab"), // F
    ];

    for t in tests.into_iter() {
        let result = Solution::is_anagram(t.0.to_owned(), t.1.to_owned());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}

/*
Dr Rojas, traumatologo, 9 de sept, emergencia
*/