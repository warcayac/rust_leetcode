use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

pub enum Direction { Left, Right }

struct TreeInfo {
    max_depth: i32,
    is_balanced: bool,
    min_depth: i32,
}

impl TreeInfo {
    fn new(max_depth: i32, is_balanced: bool, min_depth: i32) -> Self {
        Self { max_depth, is_balanced, min_depth }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    // you visit all of the nodes at an upper level before visiting any of the nodes at the next level down.
    pub fn from_level_order(values: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        let mut values = VecDeque::from(values);
        let root = values.pop_front()?.map(|val| Rc::new(RefCell::new(Self::new(val))))?;
        let mut queue = VecDeque::new();
        queue.push_back(Some(Rc::clone(&root)));
    
        while !values.is_empty() {
            let level_size = queue.len();
            for _ in 0..level_size {
                if let Some(node) = queue.pop_front().flatten() {
                    for direction in &[Direction::Left, Direction::Right] {
                        if let Some(value) = values.pop_front() {
                            match value {
                                Some(val) => {
                                    let new_node = Rc::new(RefCell::new(Self::new(val)));
                                    queue.push_back(Some(Rc::clone(&new_node)));
                                    match direction {
                                        Direction::Left => node.borrow_mut().left = Some(new_node),
                                        Direction::Right => node.borrow_mut().right = Some(new_node),
                                    }
                                },
                                None => {
                                    queue.push_back(None);
                                },
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    
        Some(root)
    }

    pub fn get_level_order(root: Option<Rc<RefCell<Self>>>) -> Vec<Option<i32>> {
        let mut queue: VecDeque<Option<Rc<RefCell<Self>>>> = VecDeque::new();  // queue: FIFO
        let mut result: Vec<Option<i32>> = Vec::new();
        let mut end_reached = false;

        if root.is_some() {
            queue.push_back(root);
        }

        while !queue.is_empty() && !end_reached {
            let level_size = queue.len();
            end_reached = true;
            
            for _ in 0..level_size {
                match queue.pop_front().flatten() {
                    Some(node) => {
                        let mut node_ref = node.borrow_mut();
                        result.push(Some(node_ref.val));
                        let item = node_ref.left.take();
                        end_reached = end_reached && item.is_none();
                        queue.push_back(item);
                        let item = node_ref.right.take();
                        end_reached = end_reached && item.is_none();
                        queue.push_back(item);
                    },
                    None => {
                        result.push(None);
                        queue.push_back(None);
                        queue.push_back(None);
                    }
                }
            }
        }

        result
    }

    pub fn from_string(s: &str) -> Option<Rc<RefCell<Self>>> {
        let values = s
            .split_terminator(|c| [',',' '].contains(&c))
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>()
        ;
        TreeNode::from_level_order(values)
    }

    pub fn max_depth(root: Option<Rc<RefCell<Self>>>) -> i32 {
        let info = TreeNode::level_counter(root.as_ref(), 0, 0);
        info.max_depth
    }

    pub fn is_balanced(root: Option<Rc<RefCell<Self>>>) -> bool {
        let info = TreeNode::level_counter(root.as_ref(), 0, 0);
        info.is_balanced
    }

    pub fn min_depth(root: Option<Rc<RefCell<Self>>>) -> i32 {
        const MAX_NUM_NODES: i32 = 100_001;
        let info = TreeNode::level_counter(root.as_ref(), 0, MAX_NUM_NODES);
        info.min_depth
    }

    /// this function returns these values:
    /// - **i32**: is the max level reached by some breach from current node: 0...
    /// - **bool**: is the tree under current node balanced?
    /// - **i32**: is the minimum length of a path for the tree under current node: 0...
    fn level_counter(node: Option<&Rc<RefCell<Self>>>, prev_level: i32, min_path_level: i32) -> TreeInfo {
        match node {
            Some(parent) => {
                let cur_level = prev_level + 1;
                if min_path_level > 0 && cur_level >= min_path_level {
                    return TreeInfo::new(prev_level, false, min_path_level);
                }
                let node_ref = parent.borrow();
                let left = Self::level_counter(node_ref.left.as_ref(), cur_level, min_path_level);
                let right = Self::level_counter(node_ref.right.as_ref(), cur_level, min_path_level);
                
                TreeInfo::new(
                    left.max_depth.max(right.max_depth),
                    left.is_balanced && right.is_balanced && (left.max_depth - right.max_depth).abs() <= 1,
                    if min_path_level > 0 {
                        if left.max_depth == cur_level && right.max_depth == cur_level {
                            cur_level.min(min_path_level)
                        } else {
                            left.min_depth.min(right.min_depth)
                        }
                    } else {
                        0
                    }
                )
            },
            None => TreeInfo::new(prev_level, true, if prev_level == 0 { 0 } else { min_path_level }),
        }
    }
}
