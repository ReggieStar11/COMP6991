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

        // Convert the Box to a raw pointer
        // SAFETY: Box::into_raw returns a valid pointer that we will manage
        let inner_ptr = Box::into_raw(inner);

        MyRc { inner: inner_ptr }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        // SAFETY: self.inner is a valid pointer to Inner<T> that was created
        //         by Box::into_raw in MyRc::new and hasn't been freed yet
        //         (since we still have a MyRc pointing to it)
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
        // SAFETY: self.inner is a valid pointer to Inner<T> that was created
        //         by Box::into_raw in MyRc::new
        unsafe {
            // Decrement the reference count
            (*self.inner).refcount -= 1;

            // If this was the last reference, drop the inner data
            if (*self.inner).refcount == 0 {
                // SAFETY: We've verified that refcount is 0, meaning this is
                //         the last MyRc pointing to this Inner. We can safely
                //         convert the raw pointer back to a Box and drop it.
                //         The pointer was originally created by Box::into_raw,
                //         so Box::from_raw is safe here.
                drop(Box::from_raw(self.inner));
            }
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: self.inner is a valid pointer to Inner<T> that was created
        //         by Box::into_raw in MyRc::new and hasn't been freed yet
        //         (since we still have a MyRc pointing to it, the refcount > 0)
        unsafe {
            // Return a reference to the data field of Inner
            &(*self.inner).data
        }
    }
}
