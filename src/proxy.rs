use hal::blocking::i2c;
use mutex;

#[cfg(feature = "std")]
use std::cell;

#[cfg(not(feature = "std"))]
use core::cell;

#[cfg(feature = "std")]
use std::marker;

#[cfg(not(feature = "std"))]
use core::marker;

pub struct BusManager<M: mutex::BusMutex<cell::RefCell<T>>, T>(M, marker::PhantomData<T>);

impl<M: mutex::BusMutex<cell::RefCell<T>>, T> BusManager<M, T> {
    pub fn new(i: T) -> BusManager<M, T> {
        let mutex = M::create(cell::RefCell::new(i));

        BusManager(mutex, marker::PhantomData)
    }

    pub fn acquire<'a>(&'a self) -> BusProxy<'a, M, T> {
        BusProxy(&self.0, marker::PhantomData)
    }
}

pub struct BusProxy<'a, M: 'a + mutex::BusMutex<cell::RefCell<T>>, T>(
    &'a M,
    marker::PhantomData<T>
);

impl<'a, M: 'a + mutex::BusMutex<cell::RefCell<T>>, T: i2c::Write> i2c::Write for BusProxy<'a, M, T> {
    type Error = T::Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.lock(|lock| {
            let mut i = lock.borrow_mut();
            i.write(addr, bytes)
        })
    }
}

impl<'a, M: 'a + mutex::BusMutex<cell::RefCell<T>>, T: i2c::Read> i2c::Read for BusProxy<'a, M, T> {
    type Error = T::Error;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.0.lock(|lock| {
            let mut i = lock.borrow_mut();
            i.read(address, buffer)
        })
    }
}

impl<'a, M: 'a + mutex::BusMutex<cell::RefCell<T>>, T: i2c::WriteRead> i2c::WriteRead
    for BusProxy<'a, M, T> {
    type Error = T::Error;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.0.lock(|lock| {
            let mut i = lock.borrow_mut();
            i.write_read(address, bytes, buffer)
        })
    }
}
