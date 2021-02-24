use std::ops::Deref;

pub struct UnsafeRef<T> {
    inner: *const T,
}

impl<T> UnsafeRef<T> {
    pub fn new(inner: &T) -> Self {
        Self {
            inner
        }
    }
}

impl<T> Clone for UnsafeRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner
        }
    }
}

impl<T> Deref for UnsafeRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().unwrap() }
    }
}
