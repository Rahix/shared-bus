use embedded_hal::i2c;
use embedded_hal::spi;

/// Proxy type for I2C bus sharing.
///
/// The `I2cProxy` implements all (blocking) I2C traits so it can be passed to drivers instead of
/// the bus instance.  Internally, it holds reference to the bus via a mutex, ensuring that all
/// accesses are strictly synchronized.
///
/// An `I2cProxy` is created by calling [`BusManager::acquire_i2c()`][acquire_i2c].
///
/// [acquire_i2c]: ./struct.BusManager.html#method.acquire_i2c
#[derive(Debug)]
pub struct I2cProxy<'a, M> {
    pub(crate) mutex: &'a M,
}

impl<'a, M: crate::BusMutex> Clone for I2cProxy<'a, M> {
    fn clone(&self) -> Self {
        Self { mutex: self.mutex }
    }
}

// Implementations for the embedded_hal alpha

impl<'a, M: crate::BusMutex> i2c::ErrorType for I2cProxy<'a, M>
where
    M::Bus: i2c::ErrorType,
{
    type Error = <M::Bus as i2c::ErrorType>::Error;
}

impl<'a, M: crate::BusMutex> i2c::I2c for I2cProxy<'a, M>
where
    M::Bus: i2c::I2c,
{
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.read(address, buffer))
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.write(address, bytes))
    }

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.write_read(address, bytes, buffer))
    }

    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.transaction(address, operations))
    }
}

/// Proxy type for SPI bus sharing.
///
/// The `SpiProxy` implements all (blocking) SPI traits so it can be passed to drivers instead of
/// the bus instance.  An `SpiProxy` is created by calling [`BusManager::acquire_spi()`][acquire_spi].
///
/// **Note**: The `SpiProxy` can only be used for sharing **withing a single task/thread**.  This
/// is due to drivers usually managing the chip-select pin manually which would be inherently racy
/// in a concurrent environment (because the mutex is locked only after asserting CS).  To ensure
/// safe usage, a `SpiProxy` can only be created when using [`BusManagerSimple`] and is `!Send`.
///
/// [acquire_spi]: ./struct.BusManager.html#method.acquire_spi
/// [`BusManagerSimple`]: ./type.BusManagerSimple.html
#[derive(Debug)]
pub struct SpiProxy<'a, M> {
    pub(crate) mutex: &'a M,
    pub(crate) _u: core::marker::PhantomData<*mut ()>,
}

impl<'a, M: crate::BusMutex> Clone for SpiProxy<'a, M> {
    fn clone(&self) -> Self {
        Self {
            mutex: self.mutex,
            _u: core::marker::PhantomData,
        }
    }
}

impl<'a, M: crate::BusMutex> spi::ErrorType for SpiProxy<'a, M>
where
    M::Bus: spi::ErrorType,
{
    type Error = <M::Bus as spi::ErrorType>::Error;
}

impl<'a, M: crate::BusMutex> spi::SpiDevice<u8> for SpiProxy<'a, M>
where
    M::Bus: spi::SpiDevice<u8>,
{

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.write(words))
    }

    fn transaction(&mut self, operations: &mut [spi::Operation<'_, u8>]) -> Result<(), Self::Error> {
        self.mutex
            .lock(|bus| bus.transaction(operations))
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.transaction(&mut [spi::Operation::Read(buf)])
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.transaction(&mut [spi::Operation::Transfer(read, write)])
    }

    fn transfer_in_place(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.transaction(&mut [spi::Operation::TransferInPlace(buf)])
    }
}