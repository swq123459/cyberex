#[cfg(test)]
mod tests {
    use std::{ffi::c_void, ptr};

    use cyberex::void::*;

    struct VoidStrut {
        age: i32,
    }

    #[test]
    #[should_panic]
    fn test_null() {
        let raw_void = ptr::null_mut::<c_void>();
        opacue_to_mut::<VoidStrut>(raw_void.cast());
    }

    #[test]
    fn test_point() {
        let mut stru = VoidStrut { age: 1 };
        let raw_stru = ptr::addr_of_mut!(stru);

        opacue_to_mut::<VoidStrut>(raw_stru);
    }

    #[test]
    fn test_hyvoid() {
        let stru = VoidStrut { age: 1 };
        let void = HyVoid::from_ref(&stru);

        let stru_ref = opacue_to_mut::<VoidStrut>(void.as_ptr().cast());
        assert_eq!(stru_ref.age, 1);

        let void_2 = HyVoid::<VoidStrut>::from_ptr(void.as_ptr());
        let stru_re2 = opacue_to_mut::<VoidStrut>(void_2.as_ptr().cast());

        assert_eq!(stru_re2.age, 1);
    }
    #[test]
    fn test_hyvoid_dptr() {
        let value: i32 = 42;

        let mut void = HyVoid::from_ref(&value);
        let dptr: *mut *mut i32 = void.as_dptr() as _;
        unsafe {
            let dereferenced_ptr = *dptr;
            let dereferenced_value = *dereferenced_ptr;
            assert_eq!(dereferenced_value, 42);
        }
    }
}
