use leetcode::utils::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut result: Option<Box<ListNode>> = None;
        
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = result;
            result = Some(node);
        }

        result
    }
}

fn main() {
    let tests = [
        vec![1,2,3,4,5], // [5,4,3,2,1]
        vec![1,2], // [2,1]
        vec![], // []
        vec![1], // [1]
    ].map(ListNode::from_vec);

    for t in tests.into_iter() {
        let result = Solution::reverse_list(t);
        println!("=> {:?}", ListNode::to_string(&result));
    }

    println!("\nJob done!");
}
