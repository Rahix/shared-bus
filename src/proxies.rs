use embedded_hal::blocking::i2c;

pub struct I2cProxy<'a, M: crate::BusMutex> {
    pub(crate) mutex: &'a M,
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
