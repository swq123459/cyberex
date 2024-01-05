#[cfg(test)]
mod tests {
    use cyberex::xffi::sto::*;
    use std::{os::raw::c_char, ffi::CString};

    #[test]
    fn test_string_to_array() {
        {
            let a = string_to_array::<2>("h");
            let a_must = ['h' as i8, '\0' as _];
            assert_eq!(a, a_must);

            assert!(a.len() == 2);
        }
        {
            let a = string_to_array::<2>("hel");
            let a_must = ['h' as i8, '\0' as _];
            assert_eq!(a, a_must);

            assert!(a.len() == 2);
        }
    }

    #[test]
    fn test_cchar_to_string() {
        {
            let c: *const c_char = std::ptr::null();
            assert_eq!(cchar_to_string(c), String::new());
        }
        {
            let c: *const c_char = "hello\0".as_ptr() as _;
            assert_eq!(cchar_to_string(c), "hello".to_string());
        }
        {
            let c = CString::new("hello").unwrap();
            assert_eq!(cchar_to_string(c.as_ptr()), "hello".to_string());

        }
    }
}
