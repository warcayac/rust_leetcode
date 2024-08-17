use leetcode::utils::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut list = list_to_vec(list1);
        list.extend(list_to_vec(list2));
        list.sort();

        vec_to_list(list)
    }
}

fn vec_to_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    values
        .into_iter()    // Creates a consuming iterator, otherwise use: iter()
        .rev()          // Reverses an iterator's direction
        .fold(      // Folds every element into an accumulator by applying an operation, returning the final result
            None,           // valor inicial del acumulador "next": next = None
            |next, val| {   // next (Option<Box<ListNode>>): acumulador,  val: actual elemento del iterador (i32)
                Some(Box::new(ListNode { val, next }))  // next = Some(...)
            }
        )
}

fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new(); // mismo que: vec![]
    let mut current = head;
    
    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }
    result
}

fn main() {
    let tests = [
        (vec![1, 2, 4], vec![1, 3, 4]),
        (vec![], vec![]),
        (vec![], vec![0]),
    ];
    for t in tests.iter() {
        let result = Solution::merge_two_lists(vec_to_list(t.0.clone()), vec_to_list(t.1.clone()));
        println!("{:?}: {:?}", t, result);
    }
    println!("\nJob done!");
}
