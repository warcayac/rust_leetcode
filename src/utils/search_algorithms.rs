use std::cmp::Ordering::{self, Equal, Greater, Less};

/// this function returns the **index** of the element of the array that satisfies the condition
/// or None if no such element exists
pub fn binary_search_on_array<T, F>(arr: &[T], compare: F) -> Option<usize>
where
    F: Fn(&T) -> Ordering,
{
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match compare(&arr[mid]) {
            Equal => return Some(mid),
            Less => left = mid + 1,
            Greater => right = mid,
        }
    }

    None
}

/*
fn main() {
    let numbers = vec![1, 3, 4, 6, 8, 9, 11];
    
    // Search for the value 6
    let result = binary_search(&numbers, |&x| x.cmp(&6));
    match result {
        Some(index) => println!("Found 6 at index {}", index),
        None => println!("6 not found in the array"),
    }

    // Search for the first value greater than or equal to 7
    let result = binary_search(&numbers, |&x| {
        if x >= 7 { std::cmp::Ordering::Equal }
        else { std::cmp::Ordering::Less }
    });
    match result {
        Some(index) => println!("First value >= 7 is at index {}", index),
        None => println!("No value >= 7 found in the array"),
    }
}
*/

/// this function returns the **value** of the element in the range that satisfies the condition
/// or None if no such element exists
pub fn binary_search_on_range<F>(range: (i32, i32), compare: F) -> Option<i32>
where
    F: Fn(i32) -> Ordering,
{
    let (mut left, mut right) = range;

    while left <= right {
        let mid = left + (right - left) / 2;
        match compare(mid) {
            Equal => return Some(mid),
            Less => left = mid + 1,
            Greater => right = mid - 1,
        }
    }

    None
}
