#[cfg(test)]
mod tests {
    use cyberex::xself::*;

    use std::{marker::PhantomPinned, pin::Pin};

    pub struct TestXSelf {
        name: String,
        _pin: PhantomPinned,
    }

    impl TestXSelf {
        fn new(name: &str) -> Pin<Box<Self>> {
            Box::pin(Self {
                name: name.to_owned(),
                _pin: PhantomPinned,
            })
        }
        fn get_name<'a>(self: &'a Pin<Box<Self>>) -> &'a str {
            let this = self_from_pinbox(self);

            &this.name
        }
        fn set_name<'a>(self: &'a mut Pin<Box<Self>>, name: &str) {
            let this = unsafe { self_mut_from_pinbox(self) };
            this.name = name.to_string();
        }
    }

    #[test]
    fn test_case() {
        let mut obj = TestXSelf::new("hello");
        assert_eq!(obj.get_name(), "hello");

        obj.set_name("world");

        assert_eq!(obj.get_name(), "world");


    }
}
