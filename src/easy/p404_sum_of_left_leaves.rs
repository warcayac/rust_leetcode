use std::{cell::RefCell, rc::Rc};
use leetcode::utils::tree_node::TreeNode;

struct Solution;

impl Solution {
    // pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     match root {
    //         Some(node) => {
    //             let node_ref = node.borrow();
    //             let left = match node_ref.left.clone() {
    //                 Some(node_left) => {
    //                     let node_left_ref = node_left.borrow();
    //                     if node_left_ref.left.is_none() && node_left_ref.right.is_none() {
    //                         node_left_ref.val
    //                     } else {
    //                         Self::sum_of_left_leaves(node_ref.left.clone())
    //                     }    
    //                 },
    //                 None => 0
    //             };

    //             let right = Self::sum_of_left_leaves(node_ref.right.clone());
    //             left + right
    //         },
    //         None => 0
    //     }
    // }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_helper(&root, false)
    }

    fn sum_helper(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match node {
            Some(n) => {
                let node_ref = n.borrow();
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    if is_left { node_ref.val } else { 0 }
                } else {
                    Self::sum_helper(&node_ref.left, true) + Self::sum_helper(&node_ref.right, false)
                }
            },
            None => 0
        }
    }
}

fn main() {
    let tests = [
        "3,9,20,null,null,15,7", // 24
        "1", // 0
        "3,null,20,15,7", // 15
    ].map(TreeNode::from_string);

    for t in tests.into_iter() {
        let result = Solution::sum_of_left_leaves(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
