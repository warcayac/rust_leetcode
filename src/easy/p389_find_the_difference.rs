struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        if s.is_empty() { return t.chars().next().unwrap(); }
        let mut chars = [0; 26]; // lowercase English letters: a-z, a=97
        let mut s = s.as_bytes().iter();
        let mut t = t.as_bytes().iter();

        for &x in s.by_ref() {
            let y = *t.next().unwrap();
            chars[(x - 97) as usize] += 1;
            chars[(y - 97) as usize] -= 1;
        }
        chars[(t.next().unwrap() - 97) as usize] -= 1;

        (chars.iter().position(|&x| x != 0).unwrap() as u8 + 97) as char
    }
}

fn main() {
    let tests = [
        ("abcd", "abcde"), // e
        ("", "y"), // y
        ("a", "aa"), // a
        ("ae", "aea"), // a
    ];

    for t in tests.into_iter() {
        let result = Solution::find_the_difference(t.0.to_owned(), t.1.to_owned());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
