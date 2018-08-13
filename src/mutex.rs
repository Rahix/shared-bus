/// An abstraction over a mutex lock.
///
/// Any type that can implement this trait can be used as a mutex for sharing a bus.
///
/// If the `std` feature is enabled, [`BusMutex`] is implemented for [`std::sync::Mutex`].
/// If the `cortexm` feature is enabled, [`BusMutex`] is implemented for [`cortex_m::interrupt::Mutex`].
pub trait BusMutex<T> {
    /// Create a new instance of this mutex type containing the value `v`.
    fn create(v: T) -> Self;

    /// Lock the mutex for the duration of the closure `f`.
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

#[cfg(feature = "cortexm")]
impl<T> BusMutex<T> for ::cortex_m::interrupt::Mutex<T> {
    fn create(v: T) -> ::cortex_m::interrupt::Mutex<T> {
        ::cortex_m::interrupt::Mutex::new(v)
    }

    fn lock<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
        ::cortex_m::interrupt::free(|cs| {
            let v = self.borrow(cs);
            f(v)
        })
    }
}
