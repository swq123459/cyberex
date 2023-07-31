#[cfg(test)]
mod tests {
    use std::{ffi::c_void, ptr};

    use cyberex::void::*;

    struct VoidStrut {}

    #[test]
    #[should_panic]
    fn test_null() {
        let raw_void = ptr::null_mut::<c_void>();
        opacue_to_mut::<VoidStrut>(raw_void.cast());

        assert!(true);
    }

    #[test]
    fn test_point() {
        let mut stru = VoidStrut {};
        let raw_stru = ptr::addr_of_mut!(stru);

        opacue_to_mut::<VoidStrut>(raw_stru);
    }
}
