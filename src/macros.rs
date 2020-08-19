/// Macro for creating a `std`-based bus manager with `'static` lifetime.
///
/// This macro is a convenience helper for creating a bus manager that lives for the `'static`
/// lifetime an thus can be safely shared across threads.
///
/// This macro is only available with the `std` feature.
///
/// # Syntax
/// ```ignore
/// let bus = shared_bus::new_std!(<Full Bus Type Signature> = <bus>).unwrap();
/// ```
///
/// The macro returns an Option which will be `Some(&'static bus_manager)` on the first run and
/// `None` afterwards.  This is necessary to uphold safety around the inner `static` variable.
///
/// # Example
/// ```
/// # struct MyDevice<T>(T);
/// # impl<T> MyDevice<T> {
/// #     pub fn new(t: T) -> Self { MyDevice(t) }
/// #     pub fn do_something_on_the_bus(&mut self) { }
/// # }
/// #
/// # struct SomeI2cBus;
/// # let i2c = SomeI2cBus;
/// // For example:
/// // let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);
///
/// // The bus is a 'static reference -> it lives forever and references can be
/// // shared with other threads.
/// let bus: &'static _ = shared_bus::new_std!(SomeI2cBus = i2c).unwrap();
///
/// let mut proxy1 = bus.acquire_i2c();
/// let mut my_device = MyDevice::new(bus.acquire_i2c());
///
/// // We can easily move a proxy to another thread:
/// # let t =
/// std::thread::spawn(move || {
///     my_device.do_something_on_the_bus();
/// });
/// # t.join().unwrap();
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! new_std {
    ($bus_type:ty = $bus:expr) => {{
        use $crate::once_cell::sync::OnceCell;

        static MANAGER: OnceCell<$crate::BusManagerStd<$bus_type>> = OnceCell::new();

        let m = $crate::BusManagerStd::new($bus);
        match MANAGER.set(m) {
            Ok(_) => MANAGER.get(),
            Err(_) => None,
        }
    }};
}

/// Macro for creating a Cortex-M bus manager with `'static` lifetime.
///
/// This macro is a convenience helper for creating a bus manager that lives for the `'static`
/// lifetime an thus can be safely shared across tasks/execution contexts (like interrupts).
///
/// This macro is only available with the `cortex-m` feature.
///
/// # Syntax
/// ```ignore
/// let bus = shared_bus::new_cortexm!(<Full Bus Type Signature> = <bus>).unwrap();
/// ```
///
/// The macro returns an Option which will be `Some(&'static bus_manager)` on the first run and
/// `None` afterwards.  This is necessary to uphold safety around the inner `static` variable.
///
/// # Example
/// ```no_run
/// # use embedded_hal::blocking::i2c::Write;
/// # struct MyDevice<T>(T);
/// # impl<T> MyDevice<T> {
/// #     pub fn new(t: T) -> Self { MyDevice(t) }
/// #     pub fn do_something_on_the_bus(&mut self) { }
/// # }
/// #
/// # struct SomeI2cBus;
/// # impl Write for SomeI2cBus {
/// #     type Error = ();
/// #     fn write(&mut self, addr: u8, buffer: &[u8]) -> Result<(), Self::Error> { Ok(()) }
/// # }
/// static mut SHARED_DEVICE:
///     Option<MyDevice<shared_bus::I2cProxy<shared_bus::CortexMMutex<SomeI2cBus>>>>
///     = None;
///
/// fn main() -> ! {
/// #   let i2c = SomeI2cBus;
///     // For example:
///     // let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);
///
///     // The bus is a 'static reference -> it lives forever and references can be
///     // shared with other tasks.
///     let bus: &'static _ = shared_bus::new_cortexm!(SomeI2cBus = i2c).unwrap();
///
///     let mut proxy1 = bus.acquire_i2c();
///     let my_device = MyDevice::new(bus.acquire_i2c());
///
///     unsafe {
///         SHARED_DEVICE = Some(my_device);
///     }
///
///     cortex_m::asm::dmb();
///
///     // enable the interrupt
///
///     loop {
///         proxy1.write(0x39, &[0xaa]);
///     }
/// }
///
/// fn INTERRUPT() {
///     let dev = unsafe {SHARED_DEVICE.as_mut().unwrap()};
///
///     dev.do_something_on_the_bus();
/// }
/// ```
#[cfg(feature = "cortex-m")]
#[macro_export]
macro_rules! new_cortexm {
    ($bus_type:ty = $bus:expr) => {{
        let m: Option<&'static mut _> = $crate::cortex_m::singleton!(
            : $crate::BusManagerCortexM<$bus_type> =
                $crate::BusManagerCortexM::new($bus)
        );

        m
    }};
}
