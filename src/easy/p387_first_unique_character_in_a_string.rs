
struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut ascii_chars: [(usize,i32); 256] = [(0,0); 256]; // (first_index, count) for each byte

        s.as_bytes().iter().enumerate().for_each(|(idx, chr)| {
            let entry = &mut ascii_chars[*chr as usize];
            if entry.1 == 0 {
                entry.0 = idx;
            }
            entry.1 += 1;
        });

        ascii_chars
            .iter()
            .filter(|&&(_, counter)| counter == 1)
            .map(|&(idx, _)| idx as i32)
            .min()
            .unwrap_or(-1)
    }
}

fn main() {
    let tests = [
        "leetcode", // 0
        "loveleetcode", // 2
        "aabb", // -1
    ];

    for t in tests.into_iter() {
        let result = Solution::first_uniq_char(t.to_owned());
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
