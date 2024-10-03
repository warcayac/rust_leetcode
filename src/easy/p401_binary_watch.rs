
struct Solution;

impl Solution {
    // pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    //     use std::collections::HashMap;
    //     let ones_h: HashMap<u8, Vec<u8>> = HashMap::from([
    //         (0, vec![0]), //1
    //         (1, vec![1, 2, 4, 8]), //4
    //         (2, vec![3, 5, 6, 9, 10]), //5
    //         (3, vec![7, 11]) //2
    //     ]);
    //     let ones_m: HashMap<u8, Vec<u8>> = HashMap::from([
    //         (0, vec![0]), //1
    //         (1, vec![1, 2, 4, 8, 16, 32]), //6
    //         (2, vec![3, 5, 6, 9, 10, 12, 17, 18, 20, 24, 33, 34, 36, 40, 48]), //15
    //         (3, vec![7, 11, 13, 14, 19, 21, 22, 25, 26, 28, 35, 37, 38, 41, 42, 44, 49, 50, 52, 56]), //20
    //         (4, vec![15, 23, 27, 29, 30, 39, 43, 45, 46, 51, 53, 54, 57, 58]), //14
    //         (5, vec![31, 47, 55, 59]) //4
    //     ]);
    //     let (max_1s_h, max_1s_m) = (3, 5);

    //     if turned_on > max_1s_h + max_1s_m {
    //         return vec![];
    //     }

    //     let mut result = vec![];
    //     (0.max(turned_on-5)..=max_1s_h.min(turned_on)).for_each(|h| result.push((h, turned_on-h)));

    //     result
    //         .into_iter()
    //         .flat_map(|(key_h, key_m)| 
    //             ones_h
    //                 .get(&(key_h as u8))
    //                 .unwrap()
    //                 .iter()
    //                 .flat_map(|hour| 
    //                     ones_m
    //                         .get(&(key_m as u8))
    //                         .unwrap()
    //                         .iter()
    //                         .map(move |minute| format!("{:?}:{:02}", hour, minute))
    //                 )
    //                 .collect::<Vec<_>>()
    //         )
    //         .collect::<Vec<_>>()
    // }
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        const MAX_TIME: u32 = 12 * 60;
        let mut result = Vec::new();
        
        for time in 0..MAX_TIME {
            let hours = time / 60;
            let minutes = time % 60;
            
            if hours >= 12 {
                continue;
            }
            
            if (hours.count_ones() + minutes.count_ones()) as i32 == turned_on {
                result.push(format!("{}:{:02}", hours, minutes));
            }
        }
        
        result
    }
}

fn main() {
    let tests = [
        1, // ["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
        9, // []
    ];

    for t in tests.into_iter() {
        let result = Solution::read_binary_watch(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
