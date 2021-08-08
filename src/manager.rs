/// "Manager" for a shared bus.
///
/// The manager owns the original bus peripheral (wrapped inside a mutex) and hands out proxies
/// which can be used by device drivers for accessing the bus.  Certain bus proxies can only be
/// created with restrictions (see the individual methods for details).
///
/// Usually the type-aliases defined in this crate should be used instead of `BusManager` directly.
/// Otherwise, the mutex type needs to be specified explicitly.  Here is an overview of aliases
/// (some are only available if a certain feature is enabled):
///
/// | Bus Manager | Mutex Type | Feature Name | Notes |
/// | --- | --- | --- | --- |
/// | [`BusManagerSimple`] | `shared_bus::NullMutex` | always available | For sharing within a single execution context. |
/// | [`BusManagerStd`] | `std::sync::Mutex` | `std` | For platforms where `std` is available. |
/// | [`BusManagerCortexM`] | `cortex_m::interrupt::Mutex` | `cortex-m` | For Cortex-M platforms; Uses a critcal section (i.e. turns off interrupts during bus transactions). |
///
/// [`BusManagerSimple`]: ./type.BusManagerSimple.html
/// [`BusManagerStd`]: ./type.BusManagerStd.html
/// [`BusManagerCortexM`]: ./type.BusManagerCortexM.html
///
/// # Constructing a `BusManager`
/// There are two ways to instanciate a bus manager.  Which one to use depends on the kind of
/// sharing that is intended.
///
/// 1. When all bus users live in the same task/thread, a `BusManagerSimple` can be used:
///
///    ```
///    # use embedded_hal::blocking::i2c;
///    # use embedded_hal::blocking::i2c::Write as _;
///    # struct MyDevice<T>(T);
///    # impl<T: i2c::Write> MyDevice<T> {
///    #     pub fn new(t: T) -> Self { MyDevice(t) }
///    #     pub fn do_something_on_the_bus(&mut self) {
///    #         self.0.write(0xab, &[0x00]);
///    #     }
///    # }
///    #
///    # fn _example(i2c: impl i2c::Write) {
///    // For example:
///    // let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);
///
///    let bus = shared_bus::BusManagerSimple::new(i2c);
///
///    let mut proxy1 = bus.acquire_i2c();
///    let mut my_device = MyDevice::new(bus.acquire_i2c());
///
///    proxy1.write(0x39, &[0xc0, 0xff, 0xee]);
///    my_device.do_something_on_the_bus();
///    # }
///    ```
///
/// 2. When users are in different execution contexts, a proper mutex type is needed and the
///    manager must be made `static` to ensure it lives long enough.  For this, `shared-bus`
///    provides a number of macros creating such a `static` instance:
///
///    ```
///    # struct MyDevice<T>(T);
///    # impl<T> MyDevice<T> {
///    #     pub fn new(t: T) -> Self { MyDevice(t) }
///    #     pub fn do_something_on_the_bus(&mut self) { }
///    # }
///    #
///    # struct SomeI2cBus;
///    # let i2c = SomeI2cBus;
///    // For example:
///    // let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);
///
///    // The bus is a 'static reference -> it lives forever and references can be
///    // shared with other threads.
///    let bus: &'static _ = shared_bus::new_std!(SomeI2cBus = i2c).unwrap();
///
///    let mut proxy1 = bus.acquire_i2c();
///    let mut my_device = MyDevice::new(bus.acquire_i2c());
///
///    // We can easily move a proxy to another thread:
///    # let t =
///    std::thread::spawn(move || {
///        my_device.do_something_on_the_bus();
///    });
///    # t.join().unwrap();
///    ```
///
///    For other platforms, similar macros exist (e.g. [`new_cortexm!()`]).
///
/// [`new_cortexm!()`]: ./macro.new_cortexm.html
#[derive(Debug)]
pub struct BusManager<M> {
    mutex: M,
}

impl<M: crate::BusMutex> BusManager<M> {
    /// Create a new bus manager for a bus.
    ///
    /// See the documentation for `BusManager` for more details.
    pub fn new(bus: M::Bus) -> Self {
        let mutex = M::create(bus);

        BusManager { mutex }
    }
}

impl<M: crate::BusMutex> BusManager<M> {
    /// Acquire an [`I2cProxy`] for this bus.
    ///
    /// [`I2cProxy`]: ./struct.I2cProxy.html
    ///
    /// The returned proxy object can then be used for accessing the bus by e.g. a driver:
    ///
    /// ```
    /// # use embedded_hal::blocking::i2c;
    /// # use embedded_hal::blocking::i2c::Write as _;
    /// # struct MyDevice<T>(T);
    /// # impl<T: i2c::Write> MyDevice<T> {
    /// #     pub fn new(t: T) -> Self { MyDevice(t) }
    /// #     pub fn do_something_on_the_bus(&mut self) {
    /// #         self.0.write(0xab, &[0x00]);
    /// #     }
    /// # }
    /// #
    /// # fn _example(i2c: impl i2c::Write) {
    /// let bus = shared_bus::BusManagerSimple::new(i2c);
    ///
    /// let mut proxy1 = bus.acquire_i2c();
    /// let mut my_device = MyDevice::new(bus.acquire_i2c());
    ///
    /// proxy1.write(0x39, &[0xc0, 0xff, 0xee]);
    /// my_device.do_something_on_the_bus();
    /// # }
    /// ```
    pub fn acquire_i2c<'a>(&'a self) -> crate::I2cProxy<'a, M> {
        crate::I2cProxy { mutex: &self.mutex }
    }

    /// Acquire an [`AdcProxy`] for this hardware block.
    ///
    /// [`AdcProxy`]: ./struct.AdcProxy.html
    ///
    /// The returned proxy object can then be used for accessing the bus by e.g. a driver:
    ///
    /// ```ignore
    /// // For example:
    /// // let ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);
    /// // let ch1 = gpioa.pa1.into_analog(&mut gpioa.crl);
    /// // let adc = Adc::adc1(p.ADC1, &mut rcc.apb2, clocks);
    ///
    /// let adc_bus: &'static _ = shared_bus::new_cortexm!(Adc<ADC1> = adc).unwrap();
    /// let mut proxy1 = adc_bus.acquire_adc();
    /// let mut proxy2 = adc_bus.acquire_adc();
    ///
    /// proxy1.read(ch0).unwrap();
    /// proxy2.read(ch1).unwrap();
    ///
    /// ```

    pub fn acquire_adc<'a>(&'a self) -> crate::AdcProxy<'a, M> {
        crate::AdcProxy { mutex: &self.mutex }
    }
}

impl<T> BusManager<crate::NullMutex<T>> {
    /// Acquire an [`SpiProxy`] for this bus.
    ///
    /// **Note**: SPI Proxies can only be created from [`BusManagerSimple`] (= bus managers using
    /// the [`NullMutex`]).  See [`SpiProxy`] for more details why.
    ///
    /// [`BusManagerSimple`]: ./type.BusManagerSimple.html
    /// [`NullMutex`]: ./struct.NullMutex.html
    /// [`SpiProxy`]: ./struct.SpiProxy.html
    ///
    /// The returned proxy object can then be used for accessing the bus by e.g. a driver:
    ///
    /// ```
    /// # use embedded_hal::blocking::spi;
    /// # use embedded_hal::digital::v2;
    /// # use embedded_hal::blocking::spi::Write as _;
    /// # struct MyDevice<T>(T);
    /// # impl<T: spi::Write<u8>> MyDevice<T> {
    /// #     pub fn new(t: T) -> Self { MyDevice(t) }
    /// #     pub fn do_something_on_the_bus(&mut self) {
    /// #         self.0.write(&[0x00]);
    /// #     }
    /// # }
    /// #
    /// # fn _example(mut cs1: impl v2::OutputPin, spi: impl spi::Write<u8>) {
    /// let bus = shared_bus::BusManagerSimple::new(spi);
    ///
    /// let mut proxy1 = bus.acquire_spi();
    /// let mut my_device = MyDevice::new(bus.acquire_spi());
    ///
    /// // Chip-select needs to be managed manually
    /// cs1.set_high();
    /// proxy1.write(&[0xc0, 0xff, 0xee]);
    /// cs1.set_low();
    ///
    /// my_device.do_something_on_the_bus();
    /// # }
    /// ```
    pub fn acquire_spi<'a>(&'a self) -> crate::SpiProxy<'a, crate::NullMutex<T>> {
        crate::SpiProxy {
            mutex: &self.mutex,
            _u: core::marker::PhantomData,
        }
    }
}
