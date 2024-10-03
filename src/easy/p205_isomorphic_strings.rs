use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut isomorphics = HashMap::new();
        let mut values = HashSet::new();
    
        s.iter()
            .zip(t.iter())
            .all(|(c1, c2)| match isomorphics.get(c1) {
                Some(v) => *v == *c2,
                None => values.insert(*c2) && isomorphics.insert(*c1, *c2).is_none(),
            })
    }
}

fn main() {
    let tests = [
        ("egg", "add"), // true
        ("foo", "bar"), // false
        ("paper", "title"), // true
        ("badc", "baba"), // F
    ];

    for t in tests.into_iter() {
        let result = Solution::is_isomorphic(t.0.to_owned(), t.1.to_owned());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
