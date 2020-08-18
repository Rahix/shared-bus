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
