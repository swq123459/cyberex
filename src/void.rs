#![allow(dead_code)]

use std::{marker::PhantomData, os::raw::c_void, ptr::NonNull};
pub struct HyVoid<T> {
    ptr: *mut c_void,
    _phantom: PhantomData<T>,
}
unsafe impl<T> Send for HyVoid<T> where T: Send {}
unsafe impl<T> Sync for HyVoid<T> where T: Sync {}

impl<T> HyVoid<T> {
    pub fn from_ref(r: &T) -> Self {
        Self {
            ptr: NonNull::from(r).as_ptr().cast(),
            _phantom: PhantomData,
        }
    }
    pub fn from_ptr(ptr: *mut c_void) -> Self {
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }
    pub unsafe fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

pub  fn opacue_to_mut<'a, T>(user: *mut T) -> &'a mut T {
    unsafe {&mut *(user as *mut T) as &mut T}
}
pub  fn opacue_to_ref<'a, T>(user: *const T) -> &'a  T {
    unsafe {& *(user as *const T) as & T }
}
pub unsafe fn mut_to_opacue<T>(r: &mut T) -> *mut c_void {
    r as *const _ as *mut _
}

pub fn delete<T>(ctx: *mut c_void) {
    drop(unsafe { Box::from_raw(ctx as *mut _ as *mut T) });
}
pub fn new<T>(t: T) -> *mut c_void {
    unsafe { &mut *(Box::into_raw(Box::new(t)) as *mut c_void) }
}
