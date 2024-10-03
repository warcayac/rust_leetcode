
struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() { return vec![]; }

        let (mut start, mut end) = (nums[0], nums[0]);
        let mut result: Vec<String> = vec![];

        for current in nums.into_iter().skip(1) {
            if end+1 == current {
                end = current;
            } else {
                result.push( if end != start { format!("{}->{}",start,end) } else { start.to_string() } );
                (start, end) = (current, current);
            }
        }
        result.push( if end != start { format!("{}->{}",start,end) } else { start.to_string() } );

        result
    }
}

fn main() {
    let tests = [
        vec![0,1,2,4,5,7], // ["0->2","4->5","7"]
        vec![0,2,3,4,6,8,9], // ["0","2->4","6","8->9"]
    ];

    for t in tests.into_iter() {
        let result = Solution::summary_ranges(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
