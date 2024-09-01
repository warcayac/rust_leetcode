struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s
            .split_whitespace()
            .last()
            .map(|s| s.len() as i32)
            .unwrap_or(0)
    }
}

fn main() {
    let tests = [
        "Hello World",
        "   fly me   to   the moon  ",
        "luffy is still joyboy",
    ].iter().map(|s| s.to_string()).collect::<Vec<String>>();

    for t in tests.iter() {
        let result = Solution::length_of_last_word(t.clone());
        println!("{:?}: {}", t, result);
    }

    println!("\nJob done!");  
}
