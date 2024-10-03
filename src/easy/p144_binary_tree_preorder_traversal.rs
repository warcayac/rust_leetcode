use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        Solution::get_preorder(root, &mut result);
        result
    }

    fn get_preorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();
            result.push(node_ref.val);
            Solution::get_preorder(node_ref.left.take(), result);
            Solution::get_preorder(node_ref.right.take(), result);
        }
    }
}

fn main() {
    let tests = [
        "1,null,2,3", // [1,2,3]
        "1,2,3,4,5,null,8,null,null,6,7,9", // [1,2,4,5,6,7,3,8,9]
        "", // []
        "1", // [1]
    ].map(TreeNode::from_string);

    for t in tests.into_iter() {
        let result = Solution::preorder_traversal(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
