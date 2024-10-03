use leetcode::utils::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let size = { // encontrar el tamaño de la lista enlazada
            let mut counter = 0;
            let mut child = head.as_ref();
            while let Some(node) = child {
                child = node.next.as_ref();
                counter += 1;
            }
            counter
        };
        // dividir la lista original, devolviendo la nueva lista creada (segunda mitad) 
        let mut head2 = { 
            let mut node = head.as_mut();
            for _ in 0..size/2-1 {
                node = node.unwrap().next.as_mut();
            }
            node.unwrap().next.take()
        };
        
        // si el tamaño de la lista original fue impar, el primer nodo de head2 debe ser omitido
        if size > 1 && size % 2 != 0 { 
            head2 = head2.unwrap().as_mut().next.take();
        }
        
        // invertir el order de enlazamiento de nodo. Ex: 3>2>1 => 3<2<1
        head2 = {
            let mut prev = None;
            let mut curr = head2;
            while let Some(mut node) = curr {
                let next = node.next.take();
                node.next = prev.take();
                prev = Some(node);
                curr = next;
            }
            prev
        };
        
        let result = {
            let mut left = head.as_ref();
            let mut right = head2.as_ref();
            while let (Some(l), Some(r)) = (left, right) {
                if l.val != r.val { return false; }
                left = l.next.as_ref();
                right = r.next.as_ref();
            }
            true
        };

        result
    }

    // pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    //     let mut fast = &head;
    //     let mut slow = &head;
    //     let mut stack = Vec::new(); // a vector that will store the first half of the list

    //     // Traverse the list and fill the stack. This loop moves fast two steps and slow one step at a time.
    //     while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
    //         stack.push(slow.as_ref().unwrap().val);
    //         slow = &slow.as_ref().unwrap().next;
    //         fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    //     }

    //     // Handle odd-length lists. If fast is not None, the list has an odd number of elements.
    //     let mut current = if fast.is_some() {
    //         &slow.as_ref().unwrap().next
    //     } else {
    //         slow
    //     };

    //     // Compare second half with stack
    //     while let Some(node) = current {
    //         if Some(node.val) != stack.pop() {
    //             return false;
    //         }
    //         current = &node.next;
    //     }

    //     true
    // }
}

fn main() {
    let tests = [
        vec![1,2,3,4,3,2,1], // T
        vec![1,2,2,1], // T
        vec![1,2], // F
        vec![1], // T
    ].map(ListNode::from_vec);

    for t in tests.into_iter() {
        let result = Solution::is_palindrome(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
