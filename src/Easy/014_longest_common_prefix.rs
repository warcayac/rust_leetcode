struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        if strs.len() == 1 {
            return strs[0].clone();
        }
        
        let mut strs = strs;
        strs.sort_by_key(|f| f.len());
        
        if strs[0].is_empty() {
            return "".to_string();
        }

        let mut prefix = strs[0].as_bytes();
        for s in strs.iter().skip(1) {
            let sb = s.as_bytes();
            
            for i in 0..prefix.len() {
                if prefix[i] != sb[i] {
                    prefix = &prefix[..i];
                    break;
                }
            }

            if prefix.is_empty() {
                break;
            }
        }

        std::str::from_utf8(prefix).unwrap().to_string()
    }
}

fn vec_string(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(|s| s.to_string()).collect::<Vec<String>>()
}

fn main() {
    let strs = [
        vec_string(vec!["flower","flow","flight"]),
        vec_string(vec!["dog","racecar","car"])
    ];
    for e in strs.iter() {
        let prefix = Solution::longest_common_prefix(e.to_vec());
        println!("{:?}: {}", e, prefix);
    }
    println!("\nJob done!");  
}
