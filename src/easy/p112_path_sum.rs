use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() { return false; }
        Self::aggregator(root, target_sum)
    }

    fn aggregator(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let mut node_ref = node.borrow_mut();
                let cur_sum = target_sum - node_ref.val;

                if node_ref.left.is_none() && node_ref.right.is_none() {
                    return cur_sum == 0;
                }

                Self::aggregator(node_ref.left.take(), cur_sum) || Self::aggregator(node_ref.right.take(), cur_sum)
            },
            None => false,
        }
    }
}

fn main() {
    let tests = [
        ("5,4,8,11,null,13,4,7,2,null,null,null,1", 22), // true
        ("1,2,3", 5), // false
        ("", 0), // false
        ("-2,null,-3",-5), // true
        ("1,-2,-3,1,3,-2,null,-1", -1), // true
        ("1,2", 1) // false
    ].map(|t| (TreeNode::from_string(t.0), t.1));

    for t in tests.into_iter() {
        let result = Solution::has_path_sum(t.0, t.1);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
