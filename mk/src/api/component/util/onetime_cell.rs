use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub struct OnetimeCell<T> {
    inner: Rc<RefCell<Option<T>>>,
}

impl<T> OnetimeCell<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Some(inner))),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<Option<T>>
    where
        T: Default,
    {
        self.inner.borrow_mut()
    }

    pub fn replace(&self, inner: T) {
        *self.inner.borrow_mut() = Some(inner);
    }

    pub fn take(&self) -> Option<T> {
        self.inner.take()
    }
}

impl<T> Default for OnetimeCell<T> {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(None)),
        }
    }
}

impl<T> Clone for OnetimeCell<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> From<T> for OnetimeCell<T> {
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}
