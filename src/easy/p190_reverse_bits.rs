struct Solution;

impl Solution {
    // pub fn reverse_bits(x: u32) -> u32 {
    //     let binary_string = format!("{:032b}", x).chars().rev().collect::<String>();
    //     u32::from_str_radix(&binary_string, 2).unwrap()
    // }

    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        // This optimized version uses a technique called "bit hacking" or "parallel bit reversal"
        x = ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1);
        x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
        x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
        x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
        
        x.rotate_right(16) // (x >> 16) | (x << 16)
    }
}

fn main() {
    let tests = [
        0b00000010100101000001111010011100, // 43261596 -> 964176192 (00111001011110000010100101000000)
        0b11111111111111111111111111111101, // 4294967293 -> 3221225471 (10111111111111111111111111111111)
        0b1101, // 13 -> 2952790016 (10110000000000000000000000000000)
    ];

    for t in tests.into_iter() {
        let result = Solution::reverse_bits(t);
        println!("=> {:?}", result);
    }

    println!("\nJob done!");
}
/*
Al trabajar con un determinado tipo de entero sin signo, debemos tener en cuenta que ello tiene una 
representación binaria de longitud fija de acuerdo al tipo de entero en uso.
Para este caso, el tipo de entero es: u32
cuyo máximo valor es: 4294967295
y su representación binaria es: 1111 1111 1111 1111 1111 1111 1111 1111
que comprenden 32 dígitos. Esta cantidad de dígitos es lo que tendrá cualquier número entero que sea 
del tipo u32.
Así pues: 43261596_u32 no es: 10 1001 0100 0001 1110 1001 1100 
si no: 0000 0010 1001 0100 0001 1110 1001 1100
y esto debe tenerse en cuenta al momento de revertir los bits de un número entero de tipo u32.

To convert the decimal number 43261596 to its reverse bits representation:

Step 1: Convert the decimal number to binary
- Start with the original decimal number: 43261596
- Apply bitwise AND (&) with 1 to get the least significant bit: 43261596 & 1 = 0
- Apply bitwise right shift (>>) by 1 bit to get the next bit: 43261596 >> 1 = 21630798
- Repeat the above two steps until the decimal number becomes 0:

43261596 & 1 = 0
43261596 >> 1 = 21630798
21630798 & 1 = 0 
21630798 >> 1 = 10815399
10815399 & 1 = 1
10815399 >> 1 = 5407699
5407699 & 1 = 1
5407699 >> 1 = 2703849
2703849 & 1 = 1
2703849 >> 1 = 1351924
1351924 & 1 = 0
1351924 >> 1 = 675962
675962 & 1 = 0
675962 >> 1 = 337981
337981 & 1 = 1
337981 >> 1 = 168990
168990 & 1 = 0
168990 >> 1 = 84495
84495 & 1 = 1
84495 >> 1 = 42247
42247 & 1 = 1
42247 >> 1 = 21123
21123 & 1 = 1
21123 >> 1 = 10561
10561 & 1 = 1
10561 >> 1 = 5280
5280 & 1 = 0
5280 >> 1 = 2640
2640 & 1 = 0
2640 >> 1 = 1320
1320 & 1 = 0
1320 >> 1 = 660
660 & 1 = 0
660 >> 1 = 330
330 & 1 = 0
330 >> 1 = 165
165 & 1 = 1
165 >> 1 = 82
82 & 1 = 0
82 >> 1 = 41
41 & 1 = 1
41 >> 1 = 20
20 & 1 = 0
20 >> 1 = 10
10 & 1 = 0
10 >> 1 = 5
5 & 1 = 1
5 >> 1 = 2
2 & 1 = 0
2 >> 1 = 1
1 & 1 = 1
1 >> 1 = 0

The binary representation of 43261596 is:
00000010 10000101 00010100 00001100

Step 2: Reverse the binary number
The reversed binary number is:
00001100 00010100 10000101 00000010

Step 3: Convert the reversed binary number back to decimal
Using bitwise OR (|) to combine the bits:
(0 * 2^24) | (0 * 2^23) | (1 * 2^22) | (1 * 2^21) | (0 * 2^20) | (0 * 2^19) | (0 * 2^18) | (0 * 2^17) |
(1 * 2^16) | (0 * 2^15) | (0 * 2^14) | (1 * 2^13) | (0 * 2^12) | (0 * 2^11) | (0 * 2^10) | (0 * 2^9) |
(0 * 2^8) | (0 * 2^7) | (0 * 2^6) | (0 * 2^5) | (0 * 2^4) | (1 * 2^3) | (1 * 2^2) | (0 * 2^1) | (0 * 2^0)
= 964176192

Therefore, the reverse bits representation of the decimal number 43261596 is 964176192.

======================================================================
technique called "bit hacking" or "parallel bit reversal"

Reversing the bits of the number 13 (binary: 00000000000000000000000000001101).
Let's go through each step:

1) Initial state:
x = 13 (00000000000000000000000000001101)

2) Step 1: Swap odd and even bits
Copyx = ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1);

0xaaaaaaaa is 10101010101010101010101010101010 in binary
0x55555555 is 01010101010101010101010101010101 in binary

Result: 00000000000000000000000000001110

3) Step 2: Swap consecutive pairs of bits
Copyx = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);

0xcccccccc is 11001100110011001100110011001100 in binary
0x33333333 is 00110011001100110011001100110011 in binary

Result: 00000000000000000000000000001011

4) Step 3: Swap 4-bit chunks
Copyx = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);

0xf0f0f0f0 is 11110000111100001111000011110000 in binary
0x0f0f0f0f is 00001111000011110000111100001111 in binary

Result: 00000000000000000000000000001011 (no change in this case)

5) Step 4: Swap 8-bit chunks
Copyx = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);

0xff00ff00 is 11111111000000001111111100000000 in binary
0x00ff00ff is 00000000111111110000000011111111 in binary

Result: 00000000000000001011000000000000

6) Step 5: Swap 16-bit halves
Copy(x >> 16) | (x << 16)

Final result: 10110000000000000000000000000000

The final result is indeed the bit-reversed version of 13.
This method works by progressively swapping larger and larger chunks of bits. In each step, 
we use carefully chosen bitmasks to isolate specific bits, shift them to their new positions, 
and then combine the results.

The beauty of this algorithm is that it always performs these five steps, regardless of the 
input number. This makes it consistently fast for any 32-bit input.

It's worth noting that for small numbers like 13, where most of the bits are zero, many of 
these operations don't change the number. However, the algorithm is designed to work correctly 
for all possible 32-bit inputs, including those with more complex bit patterns.
*/