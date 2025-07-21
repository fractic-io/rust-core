use std::sync::{Arc, Mutex};

use fractic_server_error::{define_internal_error, ServerError};

define_internal_error!(MutexError, "Couldn't acquire mutex lock.");

pub trait LockableArc<T: ?Sized> {
    #[must_use = "May fail to acquire lock, returning ServerError which should be handled."]
    fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> Result<O, ServerError>;
    #[must_use = "May fail to acquire lock, returning ServerError which should be handled."]
    fn with_lock<O>(&self, f: impl FnOnce(&T) -> O) -> Result<O, ServerError> {
        self.with_lock_mut(|guard| f(guard))
    }
}

impl<T: ?Sized> LockableArc<T> for Arc<Mutex<T>> {
    fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> Result<O, ServerError> {
        self.lock()
            .map_err(|_| MutexError::new())
            .map(|mut guard| f(&mut guard))
    }
}
