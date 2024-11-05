pub fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

pub fn get_power_of_two(n: usize) -> Option<u32> {
    if !is_power_of_two(n) { return None; }
    Some(n.trailing_zeros())
}