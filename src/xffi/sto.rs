use std::os::raw::c_char;
#[allow(clippy::needless_range_loop)]
pub fn string_to_array<const COUNT: usize>(s: &str) -> [c_char; COUNT] {
    let mut a = [0 as c_char; COUNT];
    let len = std::cmp::min(a.len() - 1, s.len());
    for i in 0..len {
        a[i] = s.as_bytes()[i] as c_char;
    }
    a
}