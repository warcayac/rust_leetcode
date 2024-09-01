struct Solution;

use leetcode::utils::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let p = TreeNode::get_level_order(p);
        // println!("> {:?}", p);
        let q = TreeNode::get_level_order(q);
        // println!("> {:?}", q);

        p == q
    }
}

fn main() {
    let tests = [
        ("1,2,3", "1,2,3"),
        ("1,2", "1,*,2"),
        ("1,2,1", "1,1,2"),
        ("1,2,7,*,4,8,9,*,*,6,*,10,*,*,12", "1,2,7,*,4,8,9,*,*,6,*,10,*,*,12"),
        // ("1,2,3,4,5,*,*,*,*,6,7", ""),
        // ("50,35,57,30,40,52,58,11", ""),
    ]
    .map(|e| (
        e.0.split(',').map(|s| s.parse::<i32>().ok()).collect::<Vec<Option<i32>>>(),
        e.1.split(',').map(|s| s.parse::<i32>().ok()).collect::<Vec<Option<i32>>>()
    ))
    .map(|t| (TreeNode::from_level_order(t.0), TreeNode::from_level_order(t.1)));

    for t in tests.into_iter() {
        let result = Solution::is_same_tree(t.0.to_owned(), t.1.to_owned());
        println!("{:?}", result);
    }

    println!("\nJob done!");
}
