use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

// Direct locks.
// ---------------------------------------------------------------------------

/// Syntactic sugar for handling lockable types.
#[allow(async_fn_in_trait)]
pub trait Lockable<T: ?Sized> {
    async fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> O;
    async fn with_lock<O>(&self, f: impl FnOnce(&T) -> O) -> O;
}

// Mutex<T>
impl<T: ?Sized> Lockable<T> for Mutex<T> {
    async fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> O {
        let mut guard = self.lock().await;
        f(&mut *guard)
    }

    async fn with_lock<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        // Sugar for with_lock_mut; concurrent reads are not possible with
        // Mutex, so behavior is identical (but nice to have type safety).
        self.with_lock_mut(|g| f(&*g)).await
    }
}

// RwLock<T>
impl<T: ?Sized> Lockable<T> for RwLock<T> {
    async fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> O {
        let mut guard = self.write().await;
        f(&mut *guard)
    }

    async fn with_lock<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        let guard = self.read().await;
        f(&*guard)
    }
}

// Arc-wrapped locks.
// ---------------------------------------------------------------------------

/// Syntactic sugar for handling Arc-wrapped lockable types.
#[allow(async_fn_in_trait)]
pub trait LockableArc<T: ?Sized> {
    async fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> O;
    async fn with_lock<O>(&self, f: impl FnOnce(&T) -> O) -> O;
}

// Arc<Mutex<T>>
impl<T: ?Sized + Send> LockableArc<T> for Arc<Mutex<T>> {
    async fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> O {
        let mut guard = self.clone().lock_owned().await;
        f(&mut *guard)
    }

    async fn with_lock<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        // Sugar for with_lock_mut; concurrent reads are not possible with
        // Mutex, so behavior is identical (but nice to have type safety).
        self.with_lock_mut(|g| f(&*g)).await
    }
}

// Arc<RwLock<T>>
impl<T: ?Sized + Send + Sync> LockableArc<T> for Arc<RwLock<T>> {
    async fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> O {
        let mut guard = self.clone().write_owned().await;
        f(&mut *guard)
    }

    async fn with_lock<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        let guard = self.clone().read_owned().await;
        f(&*guard)
    }
}
