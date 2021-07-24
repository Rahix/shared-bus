use embedded_hal::blocking::i2c;
use embedded_hal::blocking::spi;
use embedded_hal::blocking::spi::spi_with_cs::SpiWithCsError;
use embedded_hal::blocking::digital;

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
        Self {
            mutex: &self.mutex,
        }
    }
}

impl<'a, M: crate::BusMutex> i2c::Write for I2cProxy<'a, M>
where
    M::Bus: i2c::Write,
{
    type Error = <M::Bus as i2c::Write>::Error;

    fn write(&mut self, addr: u8, buffer: &[u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.write(addr, buffer))
    }
}

impl<'a, M: crate::BusMutex> i2c::Read for I2cProxy<'a, M>
where
    M::Bus: i2c::Read,
{
    type Error = <M::Bus as i2c::Read>::Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.read(addr, buffer))
    }
}

impl<'a, M: crate::BusMutex> i2c::WriteRead for I2cProxy<'a, M>
where
    M::Bus: i2c::WriteRead,
{
    type Error = <M::Bus as i2c::WriteRead>::Error;

    fn write_read(
        &mut self,
        addr: u8,
        buffer_in: &[u8],
        buffer_out: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.write_read(addr, buffer_in, buffer_out))
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
            mutex: &self.mutex,
            _u: core::marker::PhantomData,
        }
    }
}

impl<'a, M: crate::BusMutex> spi::Transfer<u8> for SpiProxy<'a, M>
where
    M::Bus: spi::Transfer<u8>,
{
    type Error = <M::Bus as spi::Transfer<u8>>::Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.mutex.lock(move |bus| bus.transfer(words))
    }
}

impl<'a, M: crate::BusMutex> spi::Write<u8> for SpiProxy<'a, M>
where
    M::Bus: spi::Write<u8>,
{
    type Error = <M::Bus as spi::Write<u8>>::Error;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.write(words))
    }
}

impl<'a, M: crate::BusMutex> spi::Transactional<u8> for SpiProxy<'a, M>
where
    M::Bus: spi::Transactional<u8>,
{
    type Error = <M::Bus as spi::Transactional<u8>>::Error;

    fn exec(&mut self, ops: &mut [spi::Operation<u8>]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| bus.exec(ops))
    }
}



/// Proxy type for SPI bus sharing with Chip Select management.
/// The `SpiProxyCs` implements all (blocking) SPI traits so it can be passed to drivers instead of
/// the bus instance.  An `SpiProxyCs` is created by calling [`BusManager::acquire_spi_cs()`][acquire_spi_cs].
/// 
/// This implementation locks the mutex _prior_ to asserting CS, ensuring exclusive access.
/// See [embedded_hal::blocking::spi::ManagedCS] for more details.
#[derive(Debug)]
pub struct SpiProxyCs<'a, M, P> {
    pub(crate) mutex: &'a M,
    pub(crate) cs: P,
    pub(crate) _u: core::marker::PhantomData<*mut ()>,
}

/// SpiProxyCx implements ManagedCs, include CS assert/de-assert in the mutex.
impl<'a, M: crate::BusMutex, CsPin> spi::ManagedCs for SpiProxyCs<'a, M, CsPin> {}

impl<'a, M: crate::BusMutex, CsPin, PinError> spi::Transfer<u8> for SpiProxyCs<'a, M, CsPin>
where
    M::Bus: spi::Transfer<u8>,
    CsPin: digital::OutputPin<Error=PinError>
{
    type Error = SpiWithCsError<<M::Bus as spi::Transfer<u8>>::Error, PinError>;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.mutex.lock(move |bus| {
            // Assert CS
            self.cs.set_low().map_err(SpiWithCsError::Pin)?;
            // Execute bus operation
            let r = bus.transfer(words).map_err(SpiWithCsError::Spi)?;
            // De-assert CS
            self.cs.set_high().map_err(SpiWithCsError::Pin)?;

            Ok(r)
        })
    }
}

impl<'a, M: crate::BusMutex, CsPin, PinError> spi::Write<u8> for SpiProxyCs<'a, M, CsPin>
where
    M::Bus: spi::Write<u8>,
    CsPin: digital::OutputPin<Error=PinError>
{
    type Error = SpiWithCsError<<M::Bus as spi::Write<u8>>::Error, PinError>;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| {
            // Assert CS
            self.cs.set_low().map_err(SpiWithCsError::Pin)?;
            // Execute bus operation
            bus.write(words).map_err(SpiWithCsError::Spi)?;
            // De-assert CS
            self.cs.set_high().map_err(SpiWithCsError::Pin)?;

            Ok(())
        })
    }
}

impl<'a, M: crate::BusMutex, CsPin, PinError> spi::Transactional<u8> for SpiProxyCs<'a, M, CsPin>
where
    M::Bus: spi::Transactional<u8>,
    CsPin: digital::OutputPin<Error=PinError>,
{
    type Error = SpiWithCsError<<M::Bus as spi::Transactional<u8>>::Error, PinError>;

    fn exec(&mut self, ops: &mut [spi::Operation<u8>]) -> Result<(), Self::Error> {
        self.mutex.lock(|bus| {
            // Assert CS
            self.cs.set_low().map_err(SpiWithCsError::Pin)?;
            // Execute bus operation
            bus.exec(ops).map_err(SpiWithCsError::Spi)?;
            // De-assert CS
            self.cs.set_high().map_err(SpiWithCsError::Pin)?;

            Ok(())
        })
    }
}
