pub fn search_in<T: PartialEq>(input: &[T], iden: &[T]) -> Option<usize> {
    input
        .windows(iden.len())
        .enumerate()
        .find_map(|(i, w)| if w == iden { Some(i) } else { None })
}
pub fn filter_in<T: PartialEq>(input: &[T], iden: &[T]) -> Vec<usize> {
    input
        .windows(iden.len())
        .enumerate()
        .filter_map(|(i, w)| if w == iden { Some(i) } else { None })
        .collect()
}