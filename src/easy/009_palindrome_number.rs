struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let s = x.to_string();
        
        s.as_bytes() == s.chars().rev().collect::<String>().as_bytes()
    }
}
  
fn main() {
    for i in [121,-121,10].iter() {
        let response = Solution::is_palindrome(*i);
        println!("{}: {}", i, response);
    }
    println!("\nJob done!");
}