use std::{cell::UnsafeCell, ops::Deref, ptr::NonNull};
use crate::Cell;
pub struct RcInner<T> {
    value: T,
    refCount: Cell<usize>,
}
pub struct rc<T> {
    inner: NonNull<RcInner<T>>,
}

impl<T> rc<T> {
    pub fn new(v:T) -> Self {
        let inner = Box::new(RcInner{
            value: v,
            refCount:Cell::new(1),
             });
        rc{
            inner: unsafe {
                NonNull::new_unchecked(Box::into_raw(inner))
            }
        }
    }
}

impl<T> Clone for rc<T>  {
    fn clone(&self) -> Self {
       let inner = unsafe{self.inner.as_ref()};
       let c = inner.refCount.get();
       inner.refCount.set(c+1);
       rc { inner: self.inner }
    }
}

impl<T> Deref for  rc<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe {
            self.inner.as_ref()
        }.value
    }
}

impl<T> Drop for rc<T> {
    fn drop(&mut self) {
let inner = unsafe {
    self.inner.as_ref()};
let c = inner.refCount.get();
if c==1 {
        drop(self.inner);
        let _ = unsafe {
            Box::from_raw(self.inner.as_ptr())
        };
} else{
    inner.refCount.set(c-1);
}
    }
}