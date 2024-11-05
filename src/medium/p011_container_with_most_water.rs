struct Solution;

impl Solution {
    // pub fn max_area(height: Vec<i32>) -> i32 {
    //     let (mut lidx, mut ridx) = (0, height.len()-1);
    //     let mut area = 0;
    //     let mut optimal_height = 0;

    //     while lidx < ridx {
    //         let h = height[lidx].min(height[ridx]);
    //         if h > 0 && h >= optimal_height {
    //             let d = (ridx - lidx) as i32;
    //             area = area.max(h * d);
    //             optimal_height = h;
    //         }
    //         match (height[lidx], height[ridx]) {
    //             (0, 0) => {
    //                 lidx += 1;
    //                 ridx -= 1;
    //             },
    //             (a, b) if a < b => lidx += 1,
    //             _ => ridx -= 1,
    //         }
    //     }

    //     area
    // }
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut iter = height.iter().enumerate();
        let mut p1 = iter.next();
        let mut p2 = iter.next_back();
        while let (Some((i, h1)), Some((j, h2))) = (p1, p2) {
            result = result.max(h1.min(h2) * (j - i) as i32);
            if h1 < h2 {
                p1 = iter.next();
            } else {
                p2 = iter.next_back();
            }
        }
        result
    }
}

fn main() {
    let tests = leetcode::data::data_011::TESTS_011.to_vec();

    for (i, t) in tests.into_iter().enumerate() {
        let result = Solution::max_area(t.0.to_vec());
        let eval = if result == t.1 {
            ('✔', true)
        } else {
            ('✘', false)
        };
        let expected = if eval.1 {
            "".to_string()
        } else {
            format!("=> Expected: {:?}", t.1)
        };
        println!("{}.[{}] {:?} {}", i + 1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
/*

Length: 71044
First 5: [(4507, 1), (7104, 1), (50823, 1), (56107, 1), (59151, 1)]
Middle 5: [(23952, 5001), (32101, 5001), (47303, 5001), (60218, 5001), (60579, 5001)]
Last 5: [(44779, 10000), (58928, 10000), (62431, 10000), (70419, 10000), (70803, 10000)]

midx = 70802, (x0, h) = (13, 9968), dx = 70790, area = 705634720
iter: 71043
1.[✔] 705634720

*/
