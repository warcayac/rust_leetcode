struct Solution;

impl Solution {
    // pub fn is_happy(n: i32) -> bool {
    //     let mut n = n as u32;
    //     const DIVISOR : u32 = 10;

    //     // if you keep squaring 7, it would end with 1
    //     while n != 1 && n != 7 {
    //         if n < 10 {
    //             return false;
    //         }
    //         let mut sum = 0;
    //         let mut dividend = n;

    //         while dividend >= DIVISOR {
    //             sum += (dividend % DIVISOR).pow(2);
    //             dividend /= DIVISOR;
    //         }
            
    //         n = sum + dividend.pow(2);
    //     }

    //     true
    // }

    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;

        let mut n = n;
        let mut set = HashSet::from([n]);
        
        while n != 1 {
            n = n.to_string()
                  .chars()
                  .fold(0, |acc, cur| { 
                    let num = cur.to_digit(10).unwrap() as i32;
                    acc + num * num
                  });

            if set.contains(&n) {
                return false;
            } else {
                set.insert(n);
            }
        }

        true
    }    
}

fn main() {
    let tests = [
        19, // T
        2, // F
        1, // T
        234, // F
        7, // T
    ];

    for t in tests.into_iter() {
        let result = Solution::is_happy(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}

/*
29:  0001 1101   0111
 2:  0000 0010
20:  0001 0100
19:  0001 0011   0100
 9:  0000 1001
 1:  0000 0001
10:  0000 1010

 
374: 0001 0111 0110
  4: 0000 0000 0100
  7: 0000 0000 0111
  3: 0000 0000 0011
 70: 0000 0100 0110
300: 0001 0010 1100

0011  1101
0101  0101
----  ----
1001  
*/