struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }

        let mut ab = (0, x / 2);

        while ab.1 - ab.0 > 1 {
            // println!("(a,b): {:?}", ab);
            let m = (ab.0 + ab.1) / 2;
            ab = if m < x/m { (m, ab.1) } else { (ab.0, m) };
        }
        // println!("(a,b): {:?}", ab);

        if ab.1 <= x/ab.1 { ab.1 } else { ab.0 }
    }
}

fn main() {
    let tests = [
        4,
        8,
        546813, // 739
        1,
        2147483647, // 46340
    ];

    for t in tests.iter() {
        let result = Solution::my_sqrt(*t);
        println!("{:?}: {}", t, result);
    }

    println!("\nJob done!");
}
