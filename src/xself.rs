use std::pin::Pin;

pub unsafe fn self_mut_from_pinbox<T>(p: &mut Pin<Box<T>>) -> &mut T {
    Pin::get_unchecked_mut(Pin::as_mut(p))
}
pub fn self_from_pinbox<T>(p: &Pin<Box<T>>) -> &T {
    Pin::get_ref(Pin::as_ref(p))
}
