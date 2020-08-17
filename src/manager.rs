pub struct BusManager<M: crate::BusMutex> {
    mutex: M,
}

impl<M: crate::BusMutex> BusManager<M> {
    pub fn new(bus: M::Bus) -> Self {
        let mutex = M::create(bus);

        BusManager { mutex }
    }
}

impl<M: crate::BusMutex> BusManager<M> {
    pub fn acquire_i2c<'a>(&'a self) -> crate::I2cProxy<'a, M> {
        crate::I2cProxy { mutex: &self.mutex }
    }
}
