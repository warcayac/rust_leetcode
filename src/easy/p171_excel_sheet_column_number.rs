struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        const BASE: i32 = 26;
        let offset = 'A' as i32 - 1;
        
        column_title
            .chars()
            .map(|c| c as i32 - offset)
            .rev()
            .enumerate()
            .map(|(i,n)| n * BASE.pow(i as u32))
            .sum()
    }
}

fn main() {
    let tests = [
        "A", // 1
        "AB", // 28
        "ZY", // 701
        "AZ", // 52
    ];

    for t in tests.into_iter() {
        let result = Solution::title_to_number(t.to_owned());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
