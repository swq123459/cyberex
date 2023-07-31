#[cfg(test)]
mod tests {
    use cyberex::xffi::sto::*;

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
}
