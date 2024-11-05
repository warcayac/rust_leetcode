struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        todo!()
    }
}

fn main() {
    let tests = [
        (vec![1,2,3], vec![1,3,2]),
        (vec![3,2,1], vec![1,2,3]),
        (vec![1,1,5], vec![1,5,1]),
        (vec![1], vec![1]),
    ];

    for (i,mut t) in tests.into_iter().enumerate() {
        Solution::next_permutation(&mut t.0);
        let result = t.0;
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
