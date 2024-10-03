struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        // Obtenemos un array de 16 elementos (array con índices 0..15) que contiene
        // los códigos ASCII de los caracteres hexadecimales.
        const HEX_CHARS: &[u8] = b"0123456789abcdef";
        let mut num = num as u32; // Convert to unsigned for bitwise operations
        let mut result = String::new();

        while num > 0 {
            // obtenemos los 4 bits menos significativos de num, cuyo valor decimal está entre 0 y 15.
            let n = (num & 0xf) as usize;
            result.push(HEX_CHARS[n] as char);
            num >>= 4;
        }

        result.chars().rev().collect()
    }
}

fn main() {
    let tests = [
        26, // 1a
        -1, // ffffffff
        0,  // 0
    ];

    for t in tests.into_iter() {
        let result = Solution::to_hex(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
