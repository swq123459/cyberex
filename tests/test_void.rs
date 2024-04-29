#[cfg(test)]
mod tests {
    use std::{ffi::c_void, ptr};

    use cyberex::void::*;

    struct VoidStrut {
        age: i32,
    }

    #[test]
    #[should_panic(expected = "Pointer is null")]
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
        let mut stru = VoidStrut { age: 1 };
        let void = HyVoid::from_ref(&mut stru);

        let stru_ref = opacue_to_mut::<VoidStrut>(void.as_ptr().cast());
        assert_eq!(stru_ref.age, 1);

        let void_2 = HyVoid::<VoidStrut>::from_ptr(void.as_ptr());
        let stru_re2 = opacue_to_mut::<VoidStrut>(void_2.as_ptr().cast());
        stru_re2.age = 2;

        assert_eq!(stru_re2.age, 2);
    }
    #[test]
    fn test_hyvoidconst() {
        let stru = VoidStrut { age: 1 };
        let void = HyVoidConst::from_ref(&stru);

        let stru_ref = opacue_to_ref::<VoidStrut>(void.as_ptr().cast());
        assert_eq!(stru_ref.age, 1);

        let void_2 = HyVoidConst::<VoidStrut>::from_ptr(void.as_ptr());
        let stru_re2 = opacue_to_ref::<VoidStrut>(void_2.as_ptr().cast());

        assert_eq!(stru_re2.age, 1);
    }
    #[test]
    fn test_hyvoid_dptr() {
        let mut value: i32 = 42;

        let mut void = HyVoid::from_ref(&mut value);
        let dptr: *mut *mut i32 = void.as_dptr() as _;
        unsafe {
            let dereferenced_ptr = *dptr;
            let dereferenced_value = *dereferenced_ptr;
            assert_eq!(dereferenced_value, 42);
        }
    }

    #[test]
    fn test_opacue_to_mut() {
        let mut stru = VoidStrut { age: 1 };
        let raw_stru = ptr::addr_of_mut!(stru);

        let stru_ref = opacue_to_mut(raw_stru);
        stru_ref.age = 2;
        assert_eq!(stru.age, 2);
    }

    #[test]
    fn test_opacue_to_ref() {
        let stru = VoidStrut { age: 1 };
        let raw_stru = ptr::addr_of!(stru);

        let stru_ref = opacue_to_ref(raw_stru);
        assert_eq!(stru_ref.age, 1);
    }

    #[test]
    fn test_new_and_delete() {
        let ptr = new(VoidStrut { age: 1 });
        delete::<VoidStrut>(ptr);
    }
    #[test]
    fn test_new_and_then_and_delete() {
        {
            let ptr = new_and_then(VoidStrut { age: 1 }, |b| {
                b.age = 2;
                Ok(())
            })
            .unwrap();
            assert_eq!(of_addr(ptr.cast::<VoidStrut>()).age, 2);

            delete::<VoidStrut>(ptr);
        }
        {
            let ptr = new_and_then(VoidStrut { age: 1 }, |b| {
                b.age = 2;

                anyhow::bail!("err");
            });
            assert!(ptr.is_err());
        }
    }
}
