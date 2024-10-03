struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut subs = s.chars().filter(|c| vowels.contains(c)).rev();
        s.chars().map(|c| if vowels.contains(&c) { subs.next().unwrap() } else { c } ).collect()
    }
}

fn main() {
    let tests = [
        "IceCreAm", // "AceCreIm"
        "leetcode" // "leotcede"
    ];

    for t in tests.into_iter() {
        let result = Solution::reverse_vowels(t.to_owned());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
