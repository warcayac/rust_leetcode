struct Solution;

use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();  // stack: FIFO
        
        if let Some(node) = root {
            stack.push(node);
        }

        while let Some(node) = stack.pop() {
            let mut node_ref = node.borrow_mut();

            if let Some(left) = node_ref.left.take() {
                stack.push(Rc::clone(&node));
                stack.push(left);
            } else {
                result.push(node_ref.val);

                if let Some(right) = node_ref.right.take() {
                    stack.push(right);
                }
            }
        }
        
        result
    }

    // pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut res = vec![];
        
    //     fn traversal(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    //         if let Some(n) = node {
    //             traversal(n.borrow().left.clone(), res);
    //             res.push(n.borrow().val);
    //             traversal(n.borrow().right.clone(), res);
    //         }
    //     }
        
    //     traversal(root, &mut res);
        
    //     res
    // }
}

fn main() {
    let tests = [
        vec![Some(1),None,Some(2),Some(3)], // [1,3,2]
        vec![], // []
        vec![Some(1)], // [1]
        vec![Some(1),Some(2),Some(7),Some(3),Some(4),Some(8),Some(9),None,Some(5),Some(6),None,Some(10),None,Some(11),Some(12)], // [3, 5, 2, 6, 4, 1, 10, 8, 7, 11, 9, 12]
    ].map(TreeNode::from_level_order);

    for t in tests.into_iter() {
        let result = Solution::inorder_traversal(t.clone());
        println!("{:?}: {:?}", t, result);
    }

    println!("\nJob done!");
}
