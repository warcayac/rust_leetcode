struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n).map(|e| {
            let fizz = e % 3 == 0;
            let buzz = e % 5 == 0;
            
            match (fizz, buzz) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                (false, false) => e.to_string(),
            }
        }).collect::<Vec<_>>()
    }
}

fn main() {
    let tests = [
        3, // ["1", "2", "Fizz"]
        5, // ["1", "2", "Fizz", "4", "Buzz"]
        15, // ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]
    ];

    for t in tests.into_iter() {
        let result = Solution::fizz_buzz(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
