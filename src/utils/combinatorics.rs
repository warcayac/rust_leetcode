pub fn combinatorial_number(n: u32, k: u32) -> u32 {
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

    loop {
        let x = numerator.next();
        let y = denominator.next();
        
        if x.is_none() && y.is_none() {
            break;
        }
        
        values.push(x.unwrap_or(1) as f64 / y.unwrap_or(1) as f64);
    }

    values.iter().product::<f64>().round() as u32
}
