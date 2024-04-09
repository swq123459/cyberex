use std::pin::Pin;

pub unsafe fn self_mut_from_pinbox<T>(p: &mut Pin<Box<T>>) -> &mut T {
    p.as_mut().get_unchecked_mut()
}
pub fn self_from_pinbox<T>(p: &Pin<Box<T>>) -> &T {
    p.as_ref().get_ref()
    // Pin::get_ref(Pin::as_ref(p))
}
