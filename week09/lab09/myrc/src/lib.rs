use std::ops::Deref;

struct Inner<T> {
    refcount: usize,
    data: T,
}

pub struct MyRc<T> {
    inner: *mut Inner<T>,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        // Create a new Inner with refcount 1 and the value
        let inner = Box::new(Inner {
            refcount: 1,
            data: value,
        });
        let inner_ptr = Box::into_raw(inner);
        MyRc { inner: inner_ptr }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            // Increment the reference count
            (*self.inner).refcount += 1;
        }
        // Return a new MyRc pointing to the same inner data
        MyRc { inner: self.inner }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            // Decrement the reference count
            (*self.inner).refcount -= 1;
            // If this was the last reference, drop the inner data
            if (*self.inner).refcount == 0 {
                drop(Box::from_raw(self.inner));
            }
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            // Return a reference to the data field of Inner
            &(*self.inner).data
        }
    }
}
