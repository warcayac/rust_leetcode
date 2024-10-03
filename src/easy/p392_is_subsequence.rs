struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let mut t = t.as_bytes().iter();
    
        for &s_b in s {
            if !t.any(|&t_b| t_b == s_b) {
                return false;
            }
        }
    
        true
    }

    /*
    An iterator maintains its own internal state about its current position. 
    When we call methods on the iterator (like any()), it advances its position automatically. 
    This means:

    1. We don't need to create new objects.
    2. We don't need to manually update any pointers or lengths.
    3. The iterator efficiently keeps track of where we are in the sequence.
    */
}

fn main() {
    let tests = [
        ("abc", "ahbgdc"), // true
        ("axc", "ahbgdc"), // false
    ];

    for t in tests.into_iter() {
        let result = Solution::is_subsequence(t.0.to_owned(), t.1.to_owned());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
