struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() || s.chars().count() % 2 != 0 {
            return false;
        }

        let chars = s.chars().map(Self::to_number).collect::<Vec<i8>>();
        let mut stack: Vec<i8> = vec![];

        for y in chars {
            if y > 0 {
                stack.push(y);
            } else {
                let x = stack.pop();
                if x.is_none() || x.unwrap() != y.abs() {
                    return false;
                }
            }
        }

        stack.is_empty()
    }

    fn to_number(c: char) -> i8 {
        match c {
            '(' => 1,
            ')' => -1,
            '[' => 2,
            ']' => -2,
            '{' => 3,
            '}' => -3,
            _ => 0,
        }
    }
}

       
fn main() {
    for e in ["()","()[]{}","(]","(){}}{","(((([[{{}}]])))){}","(([]){})"].iter() {
        let response = Solution::is_valid(e.to_string());
        println!("{}: {}", e, response);
    }
    println!("\nJob done!");  
}
