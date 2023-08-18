#[cfg(test)]
mod tests {
    use cyberex::xemcpy;
    #[test]
    fn test_xemcpy() {
        let source: [i32; 4] = [1, 2, 3, 4];
        let mut destination: [i32; 4] = [0, 0, 0, 0];

        unsafe {
            let src_ptr = source.as_ptr();
            let dest_ptr = destination.as_mut_ptr();
            let count = source.len();
            xemcpy!(dest_ptr, src_ptr, count);
        }
        assert_eq!(destination, source);
    }
    #[test]
    fn test_xemcpy_type() {
        let source = vec![1_u32, 2, 3, 4];
        let mut destination: [i32; 4] = [0, 0, 0, 0];

        unsafe {
            let src_ptr = source.as_ptr();
            let dest_ptr = destination.as_mut_ptr();
            let count = source.len();
            xemcpy!(dest_ptr, src_ptr, count);
        }
        assert_eq!(destination, [1, 2, 3, 4]);
    }
    #[test]
    fn test_xemcpy_type_and_fuzzy() {
        let source = vec![1_u32, 2, 3, 4];
        let mut destination: [i32; 3] = [0, 0, 0];

        unsafe {
            // note, here no need mut ref to destination
            xemcpy!(&mut destination[0], &source[1], 3);
        }
        assert_eq!(destination, [2, 3, 4]);
    }
    
}
