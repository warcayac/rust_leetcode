struct Solution;

#[derive(Debug, Clone, Default)]
struct TRange {
    left: usize,
    right: usize
}

impl TRange {
    fn new(left: usize, right: usize) -> Self {
        assert!(right >= left, "Error >> Invalid range: ({left},{right})");
        Self { left, right }
    }

    #[inline]
    fn len(&self) -> usize {
        self.right - self.left + 1
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 { return s; }

        let s = s.as_bytes();
        let mut indexes = vec![s.len() / 2];
        #[allow(unused_assignments)]
        let mut size = indexes[0];
        let mut result = TRange::default();
        
        loop {
            result = indexes
                .iter()
                .filter_map(|i| Self::search_palindrome(s, *i, &result))
                .max_by_key(|x| x.len())
                .unwrap_or(result)
            ;
            size = indexes[0] / 2;
            if size < 1 { break; }
            indexes = indexes.into_iter().flat_map(|x| [x - size, x + size]).collect();
        }
        
        s[result.left..=result.right].iter().map(|b| *b as char).collect::<String>()
    }

    // we apply Expand Around the Center (EAC) algorithm
    fn search_palindrome(s: &[u8], idx: usize, base: &TRange) -> Option<TRange> {
        if base.len() > 0 {
            let width = base.len() / 2;
            if idx < width || idx + width >= s.len() { return None; }
        }

        let compare = |e: TRange| if e.len() > base.len() { Some(e) } else { None };
        let eval_left = Self::get_palindrome(s, idx, idx);
        if idx + 1 >= s.len() { return eval_left.and_then(compare); }
        let eval_right = Self::get_palindrome(s, idx+1, idx+1);
        let eval_around = Self::get_palindrome(s, idx, idx+1);

        [eval_left, eval_right, eval_around]
            .into_iter()
            .flatten()
            .max_by_key(|e| e.len())
            .and_then(compare)
    }

    fn get_palindrome(s: &[u8], left: usize, right: usize) -> Option<TRange> {
        if left == 0 { return None; }
        let mut left = left - 1;
        let mut right = right;

        while s[left] == s[right] {
            if left == 0 || right == s.len()-1 { break; }
            left -= 1;
            right += 1;
        }

        if s[left] == s[right] { return Some(TRange::new(left, right)); }
        if right > left+1 { Some(TRange::new(left+1, right-1)) } else { None }
    }
}

fn main() {
    let tests = [
        ("babad","aba"), // bab, aba
        ("cbbd","bb"),
        ("efghijjihgfeabcddcbaeeeeeedcb","efghijjihgfe"),
        ("exyeemefghabccbaijjihgfedcbaaaa","abccba"),
        ("exyeemefghabcmcbaijjihgfedcbaaaa","abcmcba"),
        ("eeeeeedcbaaabcdedcbaefghijjihgfe","efghijjihgfe"),
        ("bab","bab"),
        ("bb","bb"),
        ("aaaabbaa","aabbaa"),
    ];

    for (i,t) in tests.into_iter().enumerate() {
        let result = Solution::longest_palindrome(t.0.to_owned());
        let eval = if result == t.1 {('✔', true)} else {('✘', false)};
        let expected = if eval.1 {"".to_string()} else { format!("=> Expected: {:?}", t.1) };
        println!("{}.[{}] {:?} {}", i+1, eval.0, result, expected);
    }

    println!("\nJob done!");
}
