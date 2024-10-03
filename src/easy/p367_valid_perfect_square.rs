use leetcode::utils::search_algorithms::binary_search_on_range;

struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num < 1 { return false; }
        if num == 1 { return true; }

        const MAX_RANGE: i32 = 46340; // 46_340^2 = 2_147_395_600
        binary_search_on_range(
            (1, MAX_RANGE.min(num/2)),
            |x| (x * x).cmp(&num),
        ).is_some()
    }
}

fn main() {
    let tests = [
        16, // true
        14, // false
        1, // true
        2000105819, // false
    ];

    for t in tests.into_iter() {
        let result = Solution::is_perfect_square(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
