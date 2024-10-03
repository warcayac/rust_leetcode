struct Solution;

use leetcode::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let mut node_ref = node.as_ref().borrow_mut();
                let right = node_ref.right.take();
                node_ref.right = Self::invert_tree(node_ref.left.take());
                node_ref.left = Self::invert_tree(right);
                Some(node.clone())
            },
            None => None
        }
    }
}

fn main() {
    let tests = [
        "4,2,7,1,3,6,9", // [4,7,2,9,6,3,1]
        "2,1,3",         // [2,3,1]
        "",              // []
    ]
    .map(TreeNode::from_string);

    for t in tests.into_iter() {
        let result = Solution::invert_tree(t);
        println!("=> {:?}", TreeNode::get_level_order(result));
    }

    println!("\nJob done!");
}
