struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        /*
        Dominio de x : [0, n]
        n es impar entonces x debe ser impar para obtener un cociente entero,
        n es par entonces x debe ser entero para obtener un cociente entero.
        */
        let bit = if n % 2 == 0 { 0 } else { 1 };
        let k = (n - bit) / 2;
        // println!("n: {} , bit: {} , k: {}", n, bit, k);
        
        (0..=k)
            .map(|i| Self::permutation( ((2*i+bit) as u32, (k-i) as u32) ))
            .sum::<u32>() as i32
    }

    fn combination(n: u32, k: u32) -> u32 {
        if k == 0 || k == n {
            return 1;
        }
        if k == 1 || k == n-1 {
            return n;
        }

        let a = k.max(n - k);
        let b = n - a;
        let mut numerator = a+1..=n;
        let mut denominator = 2..=b;
        let mut values: Vec<f64> = vec![];
        // print!("a: {} , b: {} , num: {:?} , den: {:?} , ", a,b, numerator, denominator);

        loop {
            let x = numerator.next();
            let y = denominator.next();
            
            if x.is_none() && y.is_none() {
                break;
            }
            
            values.push(x.unwrap_or(1) as f64 / y.unwrap_or(1) as f64);
        }
        // print!(" C({}, {}) = ", n, k);
        values.iter().product::<f64>().round() as u32
    }

    fn permutation(t: (u32,u32)) -> u32 {
        // print!("> {:?} , ", t);
        Self::combination(t.0 + t.1, t.0)
        // println!("{}", result);
    }
}

fn main() {
    let tests = [
        2,
        3,
        8,
        9,
        35, // 14930352
        38, // 63245986
        45, // 1836311903
    ];

    for t in tests.iter() {
        let result = Solution::climb_stairs(*t);
        println!("{:?}: {}", t, result);
    }

    println!("\nJob done!");
}
