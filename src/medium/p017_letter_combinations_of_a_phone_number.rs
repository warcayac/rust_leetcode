struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let idxs = digits
            .chars()
            .filter_map(|c| {
                match c.to_digit(10) {
                    Some(d) => match d {
                        0|1 => None,
                        _ => Some((d-2) as usize),
                    },
                    None => None,
                }
            })
            .collect::<Vec<_>>()
        ;
        
        if idxs.is_empty() { return vec![]; }
        
        let charsets = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut result = charsets[idxs[0]].chars().map(|c| c.to_string()).collect::<Vec<_>>();

        for i in idxs.into_iter().skip(1) {
            result = charsets[i]
                .chars()
                .flat_map(|c| 
                    result
                        .iter()
                        .map(|s| format!("{}{}", s, c))
                        .collect::<Vec<_>>()
                )
                .collect()
            ;
        }

        result
    }
}

fn main() {
    let tests = [
        ("23", vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]),
        ("0", vec![]),
        ("2", vec!["a","b","c"]),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let mut result = Solution::letter_combinations(t.0.to_string());
        result.sort();
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
