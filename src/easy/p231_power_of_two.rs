struct Solution;

impl Solution {
    // pub fn is_power_of_two(n: i32) -> bool {
    //     if n <= 0 { return false; }

    //     let mut n = n as usize;
        
    //     while n & 1 == 0 {
    //         n >>= 1;
    //     }

    //     n == 1
    // }

    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

/*
Explicación: todas las potencias de 2 tienen esta forma en binario: 10...0, y su antecesor es: 01...1
Ejemplo: 8 -> 1000, 7 -> 0111 => 8 & 7 == 0
*/

fn main() {
    let tests = [
        1, // T
        16, // T
        3, // F
    ];

    for t in tests.into_iter() {
        let result = Solution::is_power_of_two(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
