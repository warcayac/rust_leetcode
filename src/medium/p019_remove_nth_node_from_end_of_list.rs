use leetcode::utils::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let nums = ListNode::to_vec(head)
            .into_iter()
            .rev()
            .enumerate()
            .filter(|(i, _)| *i != (n-1) as usize)
            .map(|(_, v)| v)
            .rev()
            .collect::<Vec<i32>>()
        ;

        ListNode::from_vec(nums)
    }
}

fn main() {
    let tests = [
        ((vec![1,2,3,4,5], 2), vec![1,2,3,5]),
        ((vec![1], 1), vec![]),
        ((vec![1,2], 1), vec![1]),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::remove_nth_from_end(ListNode::from_vec(t.0.0), t.0.1);
        let result = ListNode::to_vec(result);
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
