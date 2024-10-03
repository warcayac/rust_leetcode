use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        Solution::get_postorder(root, &mut result);
        result
    }

    fn get_postorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();
            Solution::get_postorder(node_ref.left.take(), result);
            Solution::get_postorder(node_ref.right.take(), result);
            result.push(node_ref.val);
        }
    }
}

fn main() {
    let tests = [
        "1,null,2,3", // [3,2,1]
        "1,2,3,4,5,null,8,null,null,6,7,9", // [4,6,7,5,2,9,8,3,1]
        "", // []
        "1", // [1]
    ].map(TreeNode::from_string);

    for t in tests.into_iter() {
        let result = Solution::postorder_traversal(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
