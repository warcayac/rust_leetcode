struct Solution;

use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node_ref = root.as_ref().unwrap().borrow();
        Solution::compare_nodes(node_ref.left.as_ref(), node_ref.right.as_ref())
    }

    fn compare_nodes(left: Option<&Rc<RefCell<TreeNode>>>, right: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if left.is_none() || right.is_none() {
            return false;
        }

        let left = left.unwrap().borrow();
        let right = right.unwrap().borrow();

        if left.val != right.val {
            return false;
        }

        let outer_nodes_are_equal = Solution::compare_nodes(left.left.as_ref(), right.right.as_ref());
        let inner_nodes_are_equal = Solution::compare_nodes(left.right.as_ref(), right.left.as_ref());

        outer_nodes_are_equal && inner_nodes_are_equal
    }
}

fn main() {
    let tests = [
        "1,2,2,3,4,4,3",
        "1,2,2,null,3,null,3",
    ].map(TreeNode::from_string);

    for t in tests.into_iter() {
        let result = Solution::is_symmetric(t);
        println!("{:?}", result);
    }

    println!("\nJob done!");
}
