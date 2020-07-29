/// An abstraction over a mutex lock.
///
/// Any type that can implement this trait can be used as a mutex for sharing a bus.
///
/// If the `std` feature is enabled, [`BusMutex`] is implemented for [`std::sync::Mutex`].
/// If the `cortexm` feature is enabled, [`BusMutex`] is implemented for [`cortex_m::interrupt::Mutex`].
///
/// If there is no feature available for your Mutex type, you have to write one yourself. It should
/// look something like this (for [`cortex_m`] as an example):
/// ```
/// use shared_bus;
/// use cortex_m;
///
/// // You need a newtype because you can't implement foreign traits on
/// // foreign types.
/// struct MyMutex<T>(cortex_m::interrupt::Mutex<T>);
///
/// impl<T> shared_bus::BusMutex<T> for MyMutex<T> {
///     fn create(v: T) -> Self {
///         Self(cortex_m::interrupt::Mutex::new(v))
///     }
///
///     fn lock<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
///         cortex_m::interrupt::free(|cs| {
///             let v = self.0.borrow(cs);
///             f(v)
///         })
///     }
/// }
///
/// type MyBusManager<L, P> = shared_bus::BusManager<MyMutex<L>, P>;
/// ```
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
