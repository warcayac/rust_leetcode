struct Solution;

use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::divide_and_conquer(nums.as_slice())
    }

    fn divide_and_conquer(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() { return None; }

        let (value, left, right) = Solution::split_by_middle(values);
        let node = Rc::new(RefCell::new(TreeNode::new(value.unwrap())));

        node.borrow_mut().left = Solution::divide_and_conquer(left);
        node.borrow_mut().right = Solution::divide_and_conquer(right);

        Some(node)
    }

    fn split_by_middle(vec: &[i32]) -> (Option<i32>,&[i32], &[i32]) {
        match vec.len() {
            0 => (None, &[], &[]),
            1 => (Some(vec[0]), &[], &[]),
            _ => {
                let midx = vec.len() / 2;
                (Some(vec[midx]), &vec[..midx], &vec[midx+1..])
            }
        }
    }
}

fn main() {
    let tests = [
        vec![-10,-3,0,5,9],
        vec![1,3],
    ];

    for t in tests.into_iter() {
        let result = Solution::sorted_array_to_bst(t);
        println!("==> {:?}", TreeNode::get_level_order(result));
    }

    println!("\nJob done!");
}
