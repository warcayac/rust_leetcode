
struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        // let mut chars = std::collections::HashMap::new();
        // s.as_bytes().iter().for_each(|c| { chars.entry(c).and_modify(|e| *e += 1 ).or_insert(1); });
        
        // un array de tamaño fijo es más rápido que un HashMap, ya que su overhead es menor.
        // 128 es el número de caracteres en la tabla ASCII.
        // a = 97, z = 122, A = 65, Z = 90
        let total_chars = s.chars().count() as u16;
        let mut chars = [0u16; 128];
        let mut result = 0;

        s.as_bytes().iter().for_each(|c| { chars[*c as usize] += 1; });
        
        chars.into_iter().filter(|e| *e > 1).for_each(|e| result += e / 2 * 2);

        if total_chars > result { result += 1; }

        result as i32
    }
}

fn main() {
    let tests = [
        "abccccdd", // 7
        "a", // 1
        "ccc", // 3
        "ababababa", // 9
    ];

    for t in tests.into_iter() {
        let result = Solution::longest_palindrome(t.to_string());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
