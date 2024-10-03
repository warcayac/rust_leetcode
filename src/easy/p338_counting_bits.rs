struct Solution;

impl Solution {
    // pub fn count_bits(n: i32) -> Vec<i32> {
    //     let mut ones = Vec::with_capacity((n+ 1) as usize);

    //     (0..=n as u32).for_each(|mut x| {
    //         let mut counter = 0;
    //         while x > 0 {
    //             counter += x & 1;
    //             x >>= 1;
    //         }
    //         ones.push(counter as i32);
    //     });

    //     ones
    // }

    pub fn count_bits(n: i32) -> Vec<i32> {
        const BASE: [i32; 4] = [0, 1, 1, 2];
        let inc_vec = |v: &[i32], x: i32| v.iter().map(|&b| b + x).collect::<Vec<_>>();
        let mut sequences = vec![];
        
        sequences.push({
            let mut base_row = Vec::with_capacity(16);
            BASE.iter().for_each(|&i| base_row.extend_from_slice(&inc_vec(&BASE, i)));
            base_row
        });

        let mut row = 0;
        (0..=(n/64) as usize).for_each(|cursor| {
            for i in BASE {
                if row > n/16 { break; }
                if !(i == 0 && cursor == 0) {
                    sequences.push(inc_vec(&sequences[cursor], i));
                }
                row += 1;
            }
        });
        
        sequences.into_iter().flatten().take(n as usize + 1).collect()
    }
}

fn main() {
    let tests = [
        2, // [0,1,1]
        5, // [0,1,1,2,1,2]
    ];

    for t in tests.into_iter() {
        let result = Solution::count_bits(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
/*
0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, R0 : R0+0   0..15
1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, R1 : R0+1   16..31
1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, R2 : R0+1   32..47
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R3 : R0+2   48..63
1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, R4 : R0+1   64..79
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R5 : R0+2   80..95
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R6 : R0+2   96..111
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R7 : R0+3   112..127
1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, R8 : R0+1   128..143
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R9 : R0+2   144..159
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R9 : R0+2   160..175
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R10: R0+2   176..191
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R11: R0+3   192..207
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R12: R0+2   208..223
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R13: R0+3   224..239
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R14: R0+3   240..255
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R15: R0+4   256..271
1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, R16: R0+1   272..287
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R17: R0+2   288..303
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R18: R0+2   304..319
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R19: R0+3   320..335
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R20: R0+2   336..351
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R21: R0+3   352..367
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R22: R0+3   368..383
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R23: R0+4   384..399
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R24: R0+2   400..415
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R25: R0+3   416..431
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R26: R0+3   432..447
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R27: R0+4   448..463
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R28: R0+3   464..479
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R29: R0+4   480..495
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R30: R0+4   496..511
5, 6, 6, 7, 6, 7, 7, 8, 6, 7, 7, 8, 7, 8, 8, 9, R31: R0+5   512..527
1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, R32: R0+1   528..543
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R33: R0+2   544..559
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R34: R0+2   560..575
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R35: R0+3   576..591
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R36: R0+2   592..607
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R37: R0+3   608..623
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R38: R0+3   624..639
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R39: R0+4   640..655
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R40: R0+2   656..671
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R41: R0+3   672..687
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R42: R0+3   688..703
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R43: R0+4   640..655
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R44: R0+3   656..671
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R45: R0+4   672..687
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R46: R0+4   688..703
5, 6, 6, 7, 6, 7, 7, 8, 6, 7, 7, 8, 7, 8, 8, 9, R47: R0+5   704..719
2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, R48: R0+2   720..735
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R49: R0+3   736..751
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R50: R0+3   752..767
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R51: R0+4   768..783
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R52: R0+3   784..799
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R53: R0+4   800..815
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R54: R0+4   816..831
5, 6, 6, 7, 6, 7, 7, 8, 6, 7, 7, 8, 7, 8, 8, 9, R55: R0+5   832..847
3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, R56: R0+3   848..863
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R57: R0+4   864..879
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R58: R0+4   880..895
5, 6, 6, 7, 6, 7, 7, 8, 6, 7, 7, 8, 7, 8, 8, 9, R59: R0+5   896..911
4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8, R60: R0+4   912..927
5, 6, 6, 7, 6, 7, 7, 8, 6, 7, 7, 8, 7, 8, 8, 9, R61: R0+5   928..943
5, 6, 6, 7, 6, 7, 7, 8, 6, 7, 7, 8, 7, 8, 8, 9, R62: R0+5   944..959
6, 7, 7, 8, 7, 8, 8, 9, 7, 8, 8, 9, 8, 9, 9, 10, R63: R0+6   960..975
1

0 1 1 2
1 2 2 3 
1 2 2 3 
2 3 3 4 

1 2 2 3
2 3 3 4
2 3 3 4
3 4 4 5

1 2 2 3
2 3 3 4
2 3 3 4
3 4 4 5

2 3 3 4
3 4 4 5
3 4 4 5
4 5 5 6
*/