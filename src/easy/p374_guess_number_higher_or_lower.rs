use leetcode::utils::search_algorithms::binary_search_on_range;

struct Solution;

#[allow(unused_variables)]
unsafe fn guess(num: i32) -> i32 { todo!() }

impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        binary_search_on_range((1,n), |mid| 0.cmp(&guess(mid))).unwrap()
    }
}

fn main() {
    let tests = [
        10, // 6
        1, // 1
        2, // 1
    ];

    for t in tests.into_iter() {
        let result = unsafe { Solution::guessNumber(t) };
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
