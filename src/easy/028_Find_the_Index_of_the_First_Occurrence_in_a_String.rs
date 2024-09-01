struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map(|x| x as i32).unwrap_or(-1)
        /*
        .map(|x| x as i32):
            - This part uses the map method on the Option<usize>.
            - map takes a closure (an anonymous function) as an argument. The closure is executed 
              only if the Option is Some.
            - The closure |x| x as i32 converts the usize index (returned by find) to an i32. This 
              is necessary because the function is expected to return an i32.
        */
    }
}

fn main() {
    let tests = [
        vec!["sadbutsad", "sad"],
        vec!["leetcode", "leeto"],
    ].iter().map(|v| v.iter().map(|s| s.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

    for t in tests.iter() {
        let result = Solution::str_str(t[0].clone(), t[1].clone());
        println!("{:?}: {}", t, result);
    }

    println!("\nJob done!"); 
  
}
