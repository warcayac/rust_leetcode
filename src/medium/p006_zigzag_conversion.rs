struct Solution;


impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() <= 1 || num_rows <= 1 || s.len() <= num_rows as usize { return s; }
        
        let s = s.as_bytes();
        let length = s.len();
        let rows = num_rows as usize;
        let get_ceil = |x: usize, y: usize| ((x as f32) / (y as f32)).ceil() as usize; 
        let cols = get_ceil(length, 2.max(rows-1));
        let mut result = Vec::with_capacity(length);

        for r in 0..rows {
            for c in 0..cols {
                let index = match (rows, c % 2, r) {
                    (2, _, _) => Some(2 * c + r),
                    (_, 0, _) => Some(c * (rows - 1) + r),
                    (_, _, r) if r == 0 || r == rows - 1 => None,
                    (_, _, r) => Some((c + 1) * (rows - 1) - r)
                };
                
                if let Some(i) = index {
                    if i < length {
                        result.push(s[i] as char);
                    }
                }
            }
        }

        result.iter().collect::<String>()
    }
}

fn main() {
    let tests = [
        ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
        ("PAYPALISHIRING", 4, "PINALSIGYAHRPI"),
        ("A", 1, "A"),
        ("Apalindromeisaword,phrase,number,orothersequenceofunitsthatcanbereadthesamewayineitherdirection,withgeneralallowancesforadjustmentstopunctuationandworddividers.", 3, "Aiosrhem,tseoihartaaeeriwgrlasasnoctaoieplnrmiaodprs,ubroohreunefnttacneedhsmwynihrieto,iheeaalwnefrdutettpntainnwrdvdr.adew,anereqcustbaeeitdcntnlocojmsuuoddis"),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::convert(t.0.to_owned(), t.1);
        let eval = if result == t.2 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.2) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
