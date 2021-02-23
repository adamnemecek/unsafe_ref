pub struct UnsafeSlice<T> {
    ptr: *const T,
    len: usize,
}

impl<T> Clone for UnsafeSlice<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            len: self.len,
        }
    }
}

impl<T> UnsafeSlice<T> {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }.iter()
    }
}

impl<T> std::ops::Index<usize> for UnsafeSlice<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.ptr.offset(index as _).as_ref().unwrap() }
    }
}

impl<T> From<&[T]> for UnsafeSlice<T> {
    fn from(slice: &[T]) -> Self {
        Self {
            ptr: slice.as_ptr(),
            len: slice.len(),
        }
    }
}

pub trait IntoUnsafeSlice<T> {
    fn unsafe_slice(&self) -> UnsafeSlice<T>;
}

impl<T> IntoUnsafeSlice<T> for Vec<T> {
    fn unsafe_slice(&self) -> UnsafeSlice<T> {
        self[..].into()
    }
}
