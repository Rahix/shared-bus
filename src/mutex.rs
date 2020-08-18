use core::cell;

pub trait BusMutex {
    type Bus;

    fn create(v: Self::Bus) -> Self;
    fn lock<R, F: FnOnce(&mut Self::Bus) -> R>(&self, f: F) -> R;
}

pub struct NullMutex<T> {
    bus: cell::RefCell<T>,
}

impl<T> BusMutex for NullMutex<T> {
    type Bus = T;

    fn create(v: Self::Bus) -> Self {
        NullMutex {
            bus: cell::RefCell::new(v)
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
