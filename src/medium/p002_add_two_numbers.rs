use leetcode::utils::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head1 = l1.as_ref();
        let mut head2 = l2.as_ref();
        let mut carryover = 0;
        let mut result: Option<Box<ListNode>> = None;
        let mut tail = &mut result;
        
        while head1.is_some() || head2.is_some() || carryover != 0 {
            let mut sum = [head1,head2].iter().map(|h| h.map_or(0, |n| n.val)).sum::<i32>() + carryover;
            carryover = sum / 10;
            sum %= 10;

            *tail = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;
            
            head1 = head1.and_then(|node| node.next.as_ref());
            head2 = head2.and_then(|node| node.next.as_ref());
        }

        result
    }
}

fn main() {
    let tests = [
        (vec![2,4,3], vec![5,6,4]), // [7,0,8]
        (vec![0], vec![0]), // [0]
        (vec![9,9,9,9,9,9,9], vec![9,9,9,9]), // [8,9,9,9,0,0,0,1]
        (vec![3,2,1], vec![5,4,3,2,1]), // [8,6,4,2,1]
    ].map(|(x,y)| (ListNode::from_vec(x), ListNode::from_vec(y)));

    for t in tests.into_iter() {
        let result = Solution::add_two_numbers(t.0, t.1);
        println!("=> {:?}", ListNode::to_string(&result));
    }

    println!("\nJob done!");
}
