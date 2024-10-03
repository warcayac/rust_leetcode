use std::{cell::RefCell, rc::Rc};
use leetcode::utils::tree_node::TreeNode;

struct Solution;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut paths: Vec<String> = vec![];
        Self::search_leaf(&root, &mut paths, "".to_string());
        paths
    }

    // Depth-first search algorithm
    fn search_leaf(head: &Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<String>, curr_path: String) {
        if let Some(node) = head {
            let node_ref = node.borrow();
            let new_path = if curr_path.is_empty() { 
                node_ref.val.to_string() 
            } else {
                format!("{}->{}", curr_path, node_ref.val)
            };
            if node_ref.left.is_none() && node_ref.right.is_none() {
                paths.push(new_path);
            } else {
                Self::search_leaf(&node_ref.right, paths, new_path.clone());
                Self::search_leaf(&node_ref.left, paths, new_path);
            }
        }
    }
}

fn main() {
    let tests = [
        "1,2,3,null,5", // ["1->2->5","1->3"]
        "1", // ["1"]
    ].map(TreeNode::from_string);

    for t in tests.into_iter() {
        let result = Solution::binary_tree_paths(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
