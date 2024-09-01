use leetcode::utils::list_node::*;

struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut list = ListNode::to_vec(list1);
        list.extend(ListNode::to_vec(list2));
        list.sort();

        ListNode::from_vec(list)
    }
}

fn main() {
    let tests = [
        (vec![1, 2, 4], vec![1, 3, 4]),
        (vec![], vec![]),
        (vec![], vec![0]),
    ].map(|t| (ListNode::from_vec(t.0), ListNode::from_vec(t.1)) );
    for t in tests.iter() {
        let result = Solution::merge_two_lists(t.0.clone(), t.1.clone());
        println!("( {} , {} ) : {}", ListNode::to_string(&t.0), ListNode::to_string(&t.1), ListNode::to_string(&result));
    }
    println!("\nJob done!");
}
