
struct Solution;

impl Solution {
    // pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // use std::cmp::Ordering::{Equal, Greater, Less};
    //     if ransom_note.len() > magazine.len() { return false; }

    //     let mut ma_chars = magazine.into_bytes();
    //     let mut rn_chars = ransom_note.into_bytes();
    //     ma_chars.sort_unstable();
    //     rn_chars.sort_unstable();
    //     let mut ma_chars = ma_chars.into_iter();

    //     for rn in rn_chars.into_iter() {
    //         while let Some(ma) = ma_chars.next() {
    //             match ma.cmp(&rn) {
    //                 Equal => break,
    //                 Less => continue,
    //                 Greater => return false,
    //             }
    //         }
    //     }

    //     true
    // }
    
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() { return false; }

        let mut available_chars = std::collections::HashMap::new();
        magazine.bytes().for_each(|c| *available_chars.entry(c).or_insert(0) += 1);

        for c in ransom_note.bytes() {
            if let Some(n) = available_chars.get_mut(&c) {
                *n -= 1;
                if *n < 0 { return false; }
            } else {
                return false;
            }
        }
        
        true
    }
}

fn main() {
    let tests = [
        ("a", "b"), // false
        ("aa", "ab"), // false
        ("aa", "aab"), // true
    ];

    for t in tests.into_iter() {
        let result = Solution::can_construct(t.0.to_string(), t.1.to_string());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
