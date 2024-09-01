use leetcode::utils::list_node::*;

struct Solution;

impl Solution {
    // pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     // as_ref(): It essentially creates a reference to the value inside the Option without taking ownership.
    //     //> ?: This is the "try" operator. If the as_ref() method returns None, the code execution will 
    //     // immediately stop and return None.
    //     head.as_ref()?;

    //     const MAX_NODES: u16 = 300;
    //     let mut current = head.as_ref();
    //     let mut pivot = -200;
    //     let mut uniques: Vec<i32> = Vec::with_capacity(MAX_NODES as usize);
    
    //     while let Some(node) = current {
    //         if pivot != node.val {
    //             uniques.push(node.val);
    //             pivot = node.val;
    //         }
    //         current = node.next.as_ref();
    //     }

    //     ListNode::from_vec(uniques)
    // }

    // pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     // take ownership of the head, and we will manipulate this directly.
    //     let mut current = head;
    //     // Mutable reference to 'current' node. We use 'as_mut' and 'and_then' to traverse and modify the list in place
    //     let mut current_ref = current.as_mut();

    //     while let Some(curr_node) = current_ref {
    //         // check if the next node is a duplicate
    //         while curr_node.next.as_ref().map_or(false, |next_node| next_node.val == curr_node.val) {
    //             //skip the duplicate
    //             curr_node.next = curr_node.next.as_mut().unwrap().next.take();
    //         }
    //         // Move to the next node
    //         current_ref = curr_node.next.as_mut();
    //     }
    //     current
    // }

    // https://stackoverflow.com/a/78886459/955594
    #[allow(dead_code)]
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // The head of the list, if there aren't a list, return None
        let mut head: Box<ListNode> = head?;
        let mut current: Option<Box<ListNode>> = head.next.take();
        let mut tail: &mut Box<ListNode> = &mut head;
    
        // we TAKE the second element as the current node
        //
        // you can think of it as
        //
        //               |- tail
        //               v
        // head : () -> [first val]
        //
        // current: () -> [second val] -> [thrid val] -> ...
        //
    
        // Then we keep dequeuing the current
        // when current is None, we know the list ended
        while let Some(mut c) = current {
            //
            //                                     | tail
            //                                     v
            //  head : () -> [1] -> [2] -> ... -> [4]
            //  c    : () -> [5] -> [6]-> ...
            //
            // TAKE out the next node as the next current
            current = c.next.take();
            //
            //                                        | tail
            //                                        v
            //  head    : () -> [1] -> [2] -> ... -> [4]
            //  c       : () -> [5]
            //  current : () -> [6] -> [7]-> ...
            //
            // if the c.val is not equal to the last list, we push it to the end of the list
            if c.val != tail.val {
                tail.next = Some(c);
                tail = tail.next.as_mut().unwrap();
                //
                //                                               | tail
                //                                               v
                //  head    : () -> [1] -> [2] -> ... -> [4] -> [5]
            }
        }
    
        Some(head)
    }

    #[allow(dead_code)]
    pub fn delete_duplicates_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // take ownership of the head, and we will manipulate this directly.
        let mut current = head;
        // Mutable reference to 'current' node. We use 'as_mut' and 'and_then' to traverse and modify the list in place
        let mut current_ref = current.as_mut();
    
        while let Some(curr_node) = current_ref {
            // check if the next node is a duplicate
            while curr_node
                .next
                .as_ref()
                .map_or(false, |next_node| next_node.val == curr_node.val)
            {
                //skip the duplicate
                curr_node.next = curr_node.next.as_mut().unwrap().next.take();
            }
            // Move to the next node
            current_ref = curr_node.next.as_mut();
        }
        current
    }
    
    #[allow(dead_code)]
    pub fn delete_duplicates_3(head: &mut Option<Box<ListNode>>) {
        // Make sure that `node` is `Option<&mut Box<ListNode>>`, to
        // avoid taking ownership of nodes after the head (which would
        // not be allowed by Rust)
        let mut node = head.as_mut();
    
        while let Some(this) = node {
            // Loop through every node that shares the same value as `this`
            loop {
                if let Some(ref mut next) = this.next {
                    if this.val == next.val {
                        // `this` and `next` have the same value.
                        // Connect `this` to `next.next`, so that the
                        // list changes from:
                        //
                        //   ... -> this -> next -> next.next -> ...
                        //
                        // to:
                        //
                        //   ... -> this -> next.next -> ...
                        //
                        // The use of `take()` ensures that `next.next`
                        // is replaced with `None` (this avoids taking
                        // multiple ownerships).
                        this.next = next.next.take();
                        continue;
                    }// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

                }
                // Either there's no next node, or the next node has a
                // different value than `this`
                break;
            }
            node = this.next.as_mut();
        }
    }
    
    #[allow(dead_code)]
    fn delete_duplicates_4(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        let mut head = head.as_mut().unwrap();
    
        while let Some(ref n) = head.next {
            if head.val == n.val {
                head.next = head.next.take().unwrap().next;
            } else {
                head = head.next.as_mut().unwrap();
            }
        }
    }
}

fn main() {
    let tests = [
        vec![],
        vec![1,1,2],
        vec![1,1,2,3,3],
        vec![1,2,2,3,3,3,4],
    ].map(|v| ListNode::from_vec(v.to_vec()));

    for t in tests.iter() {
        let result = Solution::delete_duplicates(t.clone());
        println!("> {} : {}", ListNode::to_string(t), ListNode::to_string(&result));
    }

    println!("\nJob done!");
}
