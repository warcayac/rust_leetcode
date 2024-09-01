use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        TreeNode::is_balanced(root)
    }
}

fn main() {
    let tests = [
        "3,9,20,null,null,15,7",
        "1,2,2,3,3,null,null,4,4",
        "",
        "1,2,2,3,null,null,3,4,null,null,4",  // false
    ].map(TreeNode::from_string);

    for t in tests.into_iter() {
        let result = Solution::is_balanced(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
