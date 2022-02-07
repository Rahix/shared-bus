use core::cell;

/// Common interface for mutex implementations.
///
/// `shared-bus` needs a mutex to ensure only a single device can access the bus at the same time
/// in concurrent situations.  `shared-bus` already implements this trait for a number of existing
/// mutex types.  Most of them are guarded by a feature that needs to be enabled.  Here is an
/// overview:
///
/// | Mutex | Feature Name | Notes |
/// | --- | --- | --- |
/// | [`shared_bus::NullMutex`][null-mutex] | always available | For sharing within a single execution context. |
/// | [`std::sync::Mutex`][std-mutex] | `std` | For platforms where `std` is available. |
/// | [`cortex_m::interrupt::Mutex`][cortexm-mutex] | `cortex-m` | For Cortex-M platforms; Uses a critcal section (i.e. turns off interrupts during bus transactions). |
///
/// [null-mutex]: ./struct.NullMutex.html
/// [std-mutex]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
/// [cortexm-mutex]: https://docs.rs/cortex-m/0.6.3/cortex_m/interrupt/struct.Mutex.html
///
/// For other mutex types, a custom implementation is needed.  Due to the orphan rule, it might be
/// necessary to wrap it in a newtype.  As an example, this is what such a custom implementation
/// might look like:
///
/// ```
/// struct MyMutex<T>(std::sync::Mutex<T>);
///
/// impl<T> shared_bus::BusMutex for MyMutex<T> {
///     type Bus = T;
///
///     fn create(v: T) -> Self {
///         Self(std::sync::Mutex::new(v))
///     }
///
///     fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
///         let mut v = self.0.lock().unwrap();
///         f(&mut v)
///     }
/// }
///
/// // It is also beneficial to define a type alias for the BusManager
/// type BusManagerCustom<BUS> = shared_bus::BusManager<MyMutex<BUS>>;
/// ```
pub trait BusMutex {
    /// The actual bus that is wrapped inside this mutex.
    type Bus;

    /// Create a new mutex of this type.
    fn create(v: Self::Bus) -> Self;

    /// Lock the mutex and give a closure access to the bus inside.
    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R;
}

/// "Dummy" mutex for sharing in a single task/thread.
///
/// This mutex type can be used when all bus users are contained in a single execution context.  In
/// such a situation, no actual mutex is needed, because a RefCell alone is sufficient to ensuring
/// only a single peripheral can access the bus at the same time.
///
/// This mutex type is used with the [`BusManagerSimple`] type.
///
/// To uphold safety, this type is `!Send` and `!Sync`.
///
/// [`BusManagerSimple`]: ./type.BusManagerSimple.html
#[derive(Debug)]
pub struct NullMutex<T> {
    bus: cell::RefCell<T>,
}

impl<T> BusMutex for NullMutex<T> {
    type Bus = T;

    fn create(v: Self::Bus) -> Self {
        NullMutex {
            bus: cell::RefCell::new(v),
        }
    }

    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
        let mut v = self.bus.borrow_mut();
        f(&mut v)
    }
}

#[cfg(feature = "std")]
impl<T> BusMutex for ::std::sync::Mutex<T> {
    type Bus = T;

    fn create(v: Self::Bus) -> Self {
        ::std::sync::Mutex::new(v)
    }

    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
        let mut v = self.lock().unwrap();
        f(&mut v)
    }
}

/// Alias for a Cortex-M mutex.
///
/// Based on [`cortex_m::interrupt::Mutex`][cortexm-mutex].  This mutex works by disabling
/// interrupts while the mutex is locked.
///
/// [cortexm-mutex]: https://docs.rs/cortex-m/0.6.3/cortex_m/interrupt/struct.Mutex.html
///
/// This type is only available with the `cortex-m` feature.
#[cfg(feature = "cortex-m")]
pub type CortexMMutex<T> = cortex_m::interrupt::Mutex<cell::RefCell<T>>;

#[cfg(feature = "cortex-m")]
impl<T> BusMutex for CortexMMutex<T> {
    type Bus = T;

    fn create(v: T) -> Self {
        cortex_m::interrupt::Mutex::new(cell::RefCell::new(v))
    }

    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
        cortex_m::interrupt::free(|cs| {
            let c = self.borrow(cs);
            f(&mut c.borrow_mut())
        })
    }
}

/// Wrapper for an interrupt free spin mutex.
///
/// Based on [`spin::Mutex`][spin-mutex]. This mutex works by disabling
/// interrupts while the mutex is locked.
///
/// [spin-mutex]: https://docs.rs/spin/0.9.2/spin/type.Mutex.html
///
/// This type is only available with the `xtensa` feature.
#[cfg(feature = "xtensa")]
pub struct XtensaMutex<T>(spin::Mutex<T>);

#[cfg(feature = "xtensa")]
impl<T> BusMutex for XtensaMutex<T> {
    type Bus = T;

    fn create(v: T) -> Self {
        XtensaMutex(spin::Mutex::new(v))
    }

    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
        xtensa_lx::interrupt::free(|_| f(&mut (*self.0.lock())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_mutex_api_test() {
        let t = "hello ".to_string();
        let m: std::sync::Mutex<_> = BusMutex::create(t);

        BusMutex::lock(&m, |s| {
            s.push_str("world");
        });

        BusMutex::lock(&m, |s| {
            assert_eq!("hello world", s);
        });
    }
}

/// A simple coherency checker for sharing across multiple tasks/threads.
///
/// This mutex type can be used when all bus users are contained in a single structure, and is
/// intended for use with RTIC. When all bus users are contained within a structure managed by RTIC,
/// RTIC will guarantee that bus collisions do not occur. To protect against accidentaly misuse,
/// this mutex uses an atomic bool to determine when the bus is in use. If a bus collision is
/// detected, the code will panic.
///
/// This mutex type is used with the [`BusManagerAtomicMutex`] type.
///
/// This manager type is explicitly safe to share across threads because it checks to ensure that
/// collisions due to bus sharing do not occur.
///
/// [`BusManagerAtomicMutex`]: ./type.BusManagerAtomicMutex.html
#[cfg(feature = "cortex-m")]
#[derive(Debug)]
pub struct AtomicCheckMutex<BUS> {
    bus: core::cell::UnsafeCell<BUS>,
    busy: atomic_polyfill::AtomicBool,
}

// It is explicitly safe to share this across threads because there is a coherency check using an
// atomic bool comparison.
#[cfg(feature = "cortex-m")]
unsafe impl<BUS> Sync for AtomicCheckMutex<BUS> {}

#[cfg(feature = "cortex-m")]
impl<BUS> BusMutex for AtomicCheckMutex<BUS> {
    type Bus = BUS;

    fn create(v: BUS) -> Self {
        Self {
            bus: core::cell::UnsafeCell::new(v),
            busy: atomic_polyfill::AtomicBool::from(false),
        }
    }

    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R {
        self.busy
            .compare_exchange(
                false,
                true,
                core::sync::atomic::Ordering::SeqCst,
                core::sync::atomic::Ordering::SeqCst,
            )
            .expect("Bus conflict");
        let result = f(unsafe { &mut *self.bus.get() });

        self.busy.store(false, core::sync::atomic::Ordering::SeqCst);

        result
    }
}
