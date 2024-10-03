use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::counter(root.as_ref()) as i32
    }

    fn counter(head: Option<&Rc<RefCell<TreeNode>>>) -> u16 {
        match head {
            Some(node) => {
                let node_ref = node.borrow();
                1 + Self::counter(node_ref.left.as_ref()) + Self::counter(node_ref.right.as_ref())
            },
            None => 0,
        }
    }
}

fn main() {
    let tests = [
        "1,2,3,4,5,6", // 6
        "", // 0
        "1", // 1
    ].map(TreeNode::from_string);

    for t in tests.into_iter() {
        let result = Solution::count_nodes(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
