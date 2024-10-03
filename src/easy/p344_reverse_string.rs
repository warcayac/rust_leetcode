struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn reverse_string(s: &mut Vec<char>) {
        let size = s.len();
        for i in 0..size/2 {
            s.swap(i, size - i - 1);
        }
    }
}

fn main() {
    let tests = [
        vec!['h','e','l','l','o'], // ['o','l','l','e','h']
        vec!['H','a','n','n','a','h'] // ['h','a','n','n','a','H']
    ];

    for t in tests.into_iter() {
        let mut t = t.clone();
        Solution::reverse_string(&mut t);
        println!("=> {:?}", t);
    }

    println!("\nJob done!");
}
