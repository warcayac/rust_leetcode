struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_signed = x.is_negative();
        let mut x = x.abs().to_string().chars().rev().collect::<String>();
        if is_signed { x.insert(0, '-'); }
        x.parse::<i32>().unwrap_or(0)
    }
}

fn main() {
    let tests = [
        (123, 321),
        (-123, -321),
        (120, 21),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::reverse(t.0);
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
