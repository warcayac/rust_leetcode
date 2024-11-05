use leetcode::utils::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let nums = ListNode::to_vec(head)
            .chunks(2)
            .flat_map(|v| v.iter().rev())
            .copied()
            .collect::<Vec<i32>>()
        ;
        ListNode::from_vec(nums)
    }
}

fn main() {
    let tests = [
        // (vec![1,2,3,4], vec![2,1,4,3]),
        // (vec![], vec![]),
        // (vec![1], vec![1]),
        // (vec![1,2,3], vec![2,1,3]),
        (vec![1,2,3,4,5], vec![2,1,4,3,5]),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::swap_pairs(ListNode::from_vec(t.0));
        let result = ListNode::to_vec(result);
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
