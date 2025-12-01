use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(async_fn_in_trait)]
pub trait LockableArc<T: ?Sized> {
    async fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> O;

    async fn with_lock<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        self.with_lock_mut(|g| f(&*g)).await
    }
}

impl<T: ?Sized + Send> LockableArc<T> for Arc<Mutex<T>> {
    async fn with_lock_mut<O>(&self, f: impl FnOnce(&mut T) -> O) -> O {
        let mut guard = self.clone().lock_owned().await;
        f(&mut *guard)
    }
}
