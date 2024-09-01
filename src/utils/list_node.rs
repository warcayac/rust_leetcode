// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl ListNode { 
    // ------ associated functions a.k.a. "static functions"
    pub fn to_vec(head: Option<Box<Self>>) -> Vec<i32> {
        let mut result = Vec::new(); // mismo que: vec![]
        let mut current = head;
        
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }

    pub fn from_vec(values: Vec<i32>) -> Option<Box<Self>> {
        values
            .into_iter()  // Creates a consuming iterator, otherwise use: iter()
            .rev()  // Reverses an iterator's direction
            .fold(  // Folds every element into an accumulator by applying an operation, returning the final result
                None,  // valor inicial del acumulador "next": next = None
                |next, val| {  // next (Option<Box<ListNode>>): acumulador,  val: actual elemento del iterador (i32)
                    Some(Box::new(ListNode { val, next }))  // next = Some(...)
                }
            )
    }

    pub fn push(head: &mut Option<Box<Self>>, val: i32) {
        match head {
            None => *head = Some(Box::new(Self { val, next: None })),
            Some(node) => {
                let mut current = node;
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(Box::new(Self { val, next: None }));
            }
        }
    }

    pub fn to_string(head: &Option<Box<Self>>) -> String {
        match head {
            None => "[]".to_string(),
            Some(node) => {
                let mut current = node;
                let mut result = format!("[{:?}]", node.val);
                while current.next.is_some() {
                    current = current.next.as_ref().unwrap();
                    result.push_str(&format!("->[{:?}]", current.val));
                }
                result
            }
        }
    }    
}
