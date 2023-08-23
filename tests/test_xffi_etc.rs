#[cfg(test)]
mod tests {
    use std::{
        ffi::{CStr, CString},
        ptr,
    };
    use cyberex::xffi::xtr::*;


    #[test]
    fn test_string_to_buffer() {
        {
            let mut buf = vec![0; 10];
            let s = "hello".to_string();

            string_to_buffer(&s, buf.as_mut_ptr(), buf.len());

            assert_eq!(
                CStr::from_bytes_until_nul(buf.as_slice()).unwrap(),
                CString::new("hello").unwrap().as_c_str()
            )
        }
        {
            let mut buf = vec![1, 2, 3, 4]; // Not init
            let s = "hello".to_string();

            string_to_buffer(&s, buf.as_mut_ptr(), buf.len());

            assert_eq!(
                CStr::from_bytes_until_nul(buf.as_slice()).unwrap(),
                CString::new("hel").unwrap().as_c_str()
            )
        }
    }

    #[test]
    fn test_string_to_dbuffer() {
        let mut null_p = ptr::null_mut::<u8>();
        let mut size_out: usize = 0;
        let test_str = "hello World";
        string_to_dbuffer("hello World", ptr::addr_of_mut!(null_p), ptr::addr_of_mut!(size_out));
        assert_eq!(
            String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(null_p, size_out) }),
            test_str
        );
    }
}