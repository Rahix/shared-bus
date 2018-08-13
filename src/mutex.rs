pub trait BusMutex<T> {
    fn create(v: T) -> Self;
    fn lock<R, F: FnOnce(&T) -> R>(&self, f: F) -> R;
}

#[cfg(feature = "std")]
impl<T> BusMutex<T> for ::std::sync::Mutex<T> {
    fn create(v: T) -> Self {
        ::std::sync::Mutex::new(v)
    }

    fn lock<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
        let v = self.lock().unwrap();
        f(&v)
    }
}
