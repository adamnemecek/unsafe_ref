pub struct UnsafeSlice<T> {
    ptr: *const T,
    len: usize,
}

impl<T> UnsafeSlice<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len).iter() }
    }
}

impl<T> std::ops::Index<usize> for UnsafeSlice<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        todo!()
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

