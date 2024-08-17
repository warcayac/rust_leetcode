use leetcode::utils::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        None
    }
}

fn main() {
    let tests = [
        vec![1,1,2],
        vec![1,1,2,3,3],
    ];

    for t in tests.iter() {
        let result = Solution::delete_duplicates();
        println!("{:?}: {}", t, result);
    }

    println!("\nJob done!");
}
