use leetcode::utils::list_node::ListNode;

struct Solution;

impl Solution {
    // pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    //     Solution::remover(head, val)
    // }

    // fn remover(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    //     match head {
    //         Some(mut node) => {
    //             let next_node = node.next.take();
    //             if node.val.eq(&val) {
    //                 Solution::remover(next_node, val)
    //             } else {
    //                 node.next = Solution::remover(next_node, val);
    //                 Some(node)
    //             }
    //         },
    //         None => None,
    //     }
    // }

    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut prev: &mut ListNode = &mut dummy;
        let mut head = head;
        while let Some(mut hd) = head {
            head = hd.next.take();
            if hd.val != val {
                prev.next = Some(hd);
                prev = prev.next.as_deref_mut().unwrap();
            }
        }
        dummy.next.take()
    }
}

fn main() {
    let tests = [
        (vec![1,2,6,3,4,5,6], 6), // [1,2,3,4,5]
        (vec![], 1), // []
        (vec![7,7,7,7], 7), // []
    ].map(|t| (ListNode::from_vec(t.0), t.1));

    for t in tests.into_iter() {
        let result = Solution::remove_elements(t.0, t.1);
        println!("=> {:?}", ListNode::to_string(&result));
    }

    println!("\nJob done!");
}
