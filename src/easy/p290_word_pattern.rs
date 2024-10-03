use std::collections::{HashMap, HashSet};


struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.split_whitespace().count() {
            return false;
        }
        
        let mut dic: HashMap<String, char> = HashMap::new();
        let mut uniques = HashSet::new();

        pattern
            .chars()
            .zip(s.split_whitespace())
            .all(|(c, w)| match dic.get(w) {
                Some(&value) => value == c,
                None => uniques.insert(c) && dic.insert(w.to_owned(), c).is_none(),
            })
        /*
        uniques.insert(c): true if c is not in uniques
        dic.insert(w, c): None if w is not in dic otherwise returns Some(old_value)
        */
    }
}

fn main() {
    let tests = [
        ("abba", "dog cat cat dog"), // true
        ("abba", "dog cat cat fish"), // false
        ("aaaa", "dog cat cat dog"), // false
    ];

    for t in tests.into_iter() {
        let result = Solution::word_pattern(t.0.to_owned(), t.1.to_owned());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
