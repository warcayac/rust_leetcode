struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut max_len, mut start) = (0, 0);

        for (i, e) in s.iter().enumerate() {
            let slice = &s[start..i];
            if slice.contains(e) {
                max_len = max_len.max(slice.len());
                start += slice.iter().position(|x| x == e).unwrap() + 1;
            }
        }
        max_len = max_len.max(s[start..].len());

        max_len as i32
    }
}

fn main() {
    let tests = [
        ("abcabcbb", 3),
        ("bbbbb", 1),
        ("pwwkew", 3),
        (" ", 1),
        ("aab", 2),
        ("dvdf", 3),
        ("abcdefghijklmnopqrstuvwxyzabc", 26),
        ("abcabcabcabcabcabcabcab", 3),
        ("bbbbbbbbbbbbbbbbbcccccc", 2),
        ("pwwkewpwwkewpwwkewpwwke", 4),
    ];
    
    for t in tests.into_iter() {
        let result = Solution::length_of_longest_substring(t.0.to_owned());
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("[{}] {:?} {}", eval.0, result, expected);
    }

    println!("\nJob done!");
}
