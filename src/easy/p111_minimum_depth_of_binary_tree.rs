use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        TreeNode::min_depth(root)
    }
}

fn main() {
    let tests = [
        "",  // 0
        "3",  // 1
        "3,9,20,null,null,15,7", // 2
        "2,null,3,null,4,null,5,null,6", // 5
        "3,4,null,5,null,6", // 4
        "2,3,4,null,null,5,null,6", // 2
        "2,null,3,4,5,null,null,6,null,7", // 3
    ].map(TreeNode::from_string);

    for t in tests.into_iter() {
        let result = Solution::min_depth(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
