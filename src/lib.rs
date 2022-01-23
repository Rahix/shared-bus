//! **shared-bus** is a crate to allow sharing bus peripherals safely between multiple devices.
//!
//! In the `embedded-hal` ecosystem, it is convention for drivers to "own" the bus peripheral they
//! are operating on.  This implies that only _one_ driver can have access to a certain bus.  That,
//! of course, poses an issue when multiple devices are connected to a single bus.
//!
//! _shared-bus_ solves this by giving each driver a bus-proxy to own which internally manages
//! access to the actual bus in a safe manner.  For a more in-depth introduction of the problem
//! this crate is trying to solve, take a look at the [blog post][blog-post].
//!
//! There are different 'bus managers' for different use-cases:
//!
//! # Sharing within a single task/thread
//! As long as all users of a bus are contained in a single task/thread, bus sharing is very
//! simple.  With no concurrency possible, no special synchronization is needed.  This is where
//! a [`BusManagerSimple`] should be used:
//!
//! ```
//! # use embedded_hal::blocking::i2c;
//! # use embedded_hal::blocking::i2c::Write as _;
//! # struct MyDevice<T>(T);
//! # impl<T: i2c::Write> MyDevice<T> {
//! #     pub fn new(t: T) -> Self { MyDevice(t) }
//! #     pub fn do_something_on_the_bus(&mut self) {
//! #         self.0.write(0xab, &[0x00]);
//! #     }
//! # }
//! #
//! # fn _example(i2c: impl i2c::Write) {
//! // For example:
//! // let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);
//!
//! let bus = shared_bus::BusManagerSimple::new(i2c);
//!
//! let mut proxy1 = bus.acquire_i2c();
//! let mut my_device = MyDevice::new(bus.acquire_i2c());
//!
//! proxy1.write(0x39, &[0xc0, 0xff, 0xee]);
//! my_device.do_something_on_the_bus();
//! # }
//! ```
//!
//! The `BusManager::acquire_*()` methods can be called as often as needed; each call will yield
//! a new bus-proxy of the requested type.
//!
//! # Sharing across multiple tasks/threads
//! For sharing across multiple tasks/threads, synchronization is needed to ensure all bus-accesses
//! are strictly serialized and can't race against each other.  The synchronization is handled by
//! a platform-specific [`BusMutex`] implementation.  _shared-bus_ already contains some
//! implementations for common targets.  For each one, there is also a macro for easily creating
//! a bus-manager with `'static` lifetime, which is almost always a requirement when sharing across
//! task/thread boundaries.  As an example:
//!
//! ```
//! # struct MyDevice<T>(T);
//! # impl<T> MyDevice<T> {
//! #     pub fn new(t: T) -> Self { MyDevice(t) }
//! #     pub fn do_something_on_the_bus(&mut self) { }
//! # }
//! #
//! # struct SomeI2cBus;
//! # let i2c = SomeI2cBus;
//! // For example:
//! // let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);
//!
//! // The bus is a 'static reference -> it lives forever and references can be
//! // shared with other threads.
//! let bus: &'static _ = shared_bus::new_std!(SomeI2cBus = i2c).unwrap();
//!
//! let mut proxy1 = bus.acquire_i2c();
//! let mut my_device = MyDevice::new(bus.acquire_i2c());
//!
//! // We can easily move a proxy to another thread:
//! # let t =
//! std::thread::spawn(move || {
//!     my_device.do_something_on_the_bus();
//! });
//! # t.join().unwrap();
//! ```
//!
//! Those platform-specific bits are guarded by a feature that needs to be enabled.  Here is an
//! overview of what's already available:
//!
//! | Mutex | Bus Manager | `'static` Bus Macro | Feature Name |
//! | --- | --- | --- | --- |
//! | `std::sync::Mutex` | [`BusManagerStd`] | [`new_std!()`] | `std` |
//! | `cortex_m::interrupt::Mutex` | [`BusManagerCortexM`] | [`new_cortexm!()`] | `cortex-m` |
//! | `shared_bus::XtensaMutex` (`spin::Mutex` in critical section) | [`BusManagerXtensa`] | [`new_xtensa!()`] | `xtensa` |
//! | None (Automatically Managed) | [`BusManagerAtomicCheck`] | [`new_atomic_check!()`] | `cortex-m` |
//!
//! # Supported buses and hardware blocks
//! Currently, the following buses/blocks can be shared with _shared-bus_:
//!
//! | Bus/Block | Proxy Type | Acquire Method | Comments |
//! | --- | --- | --- | --- |
//! | I2C | [`I2cProxy`] | [`.acquire_i2c()`] | |
//! | SPI | [`SpiProxy`] | [`.acquire_spi()`] | SPI can only be shared within a single task (See [`SpiProxy`] for details). |
//! | ADC | [`AdcProxy`] | [`.acquire_adc()`] | |
//!
//!
//! [`.acquire_i2c()`]: ./struct.BusManager.html#method.acquire_i2c
//! [`.acquire_spi()`]: ./struct.BusManager.html#method.acquire_spi
//! [`.acquire_adc()`]: ./struct.BusManager.html#method.acquire_adc
//! [`BusManagerCortexM`]: ./type.BusManagerCortexM.html
//! [`BusManagerXtensa`]: ./type.BusManagerXtensa.html
//! [`BusManagerAtomicCheck`]: ./type.BusManagerAtomicCheck.html
//! [`BusManagerSimple`]: ./type.BusManagerSimple.html
//! [`BusManagerStd`]: ./type.BusManagerStd.html
//! [`BusMutex`]: ./trait.BusMutex.html
//! [`I2cProxy`]: ./struct.I2cProxy.html
//! [`SpiProxy`]: ./struct.SpiProxy.html
//! [`AdcProxy`]: ./struct.AdcProxy.html
//! [`new_cortexm!()`]: ./macro.new_cortexm.html
//! [`new_xtensa!()`]: ./macro.new_xtensa.html
//! [`new_std!()`]: ./macro.new_std.html
//! [`new_atomic_check!()`]: ./macro.new_atomic_check.html
//! [blog-post]: https://blog.rahix.de/001-shared-bus
#![doc(html_root_url = "https://docs.rs/shared-bus")]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

mod macros;
mod manager;
mod mutex;
mod proxies;

#[doc(hidden)]
#[cfg(feature = "std")]
pub use once_cell;

#[doc(hidden)]
#[cfg(feature = "cortex-m")]
pub use cortex_m;

#[doc(hidden)]
#[cfg(feature = "xtensa")]
pub use xtensa_lx;

pub use manager::BusManager;
pub use mutex::BusMutex;
#[cfg(feature = "cortex-m")]
pub use mutex::CortexMMutex;
pub use mutex::NullMutex;
#[cfg(feature = "xtensa")]
pub use mutex::XtensaMutex;
pub use proxies::AdcProxy;
pub use proxies::I2cProxy;
pub use proxies::SpiProxy;

#[cfg(feature = "cortex-m")]
pub use mutex::AtomicCheckMutex;

/// A bus manager for sharing within a single task/thread.
///
/// This is the bus manager with the least overhead; it should always be used when all bus users
/// are confined to a single task/thread as it has no side-effects (like blocking or turning off
/// interrupts).
///
/// # Example
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
/// // For example:
/// // let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);
///
/// let bus = shared_bus::BusManagerSimple::new(i2c);
///
/// let mut proxy1 = bus.acquire_i2c();
/// let mut my_device = MyDevice::new(bus.acquire_i2c());
///
/// proxy1.write(0x39, &[0xc0, 0xff, 0xee]);
/// my_device.do_something_on_the_bus();
/// # }
/// ```
pub type BusManagerSimple<BUS> = BusManager<NullMutex<BUS>>;

/// A bus manager for safely sharing between threads on a platform with `std` support.
///
/// This manager internally uses a `std::sync::Mutex` for synchronizing bus accesses.  As sharing
/// across threads will in most cases require a manager with `'static` lifetime, the
/// [`shared_bus::new_std!()`][new_std] macro exists to create such a bus manager.
///
/// [new_std]: ./macro.new_std.html
///
/// This type is only available with the `std` feature.
#[cfg(feature = "std")]
pub type BusManagerStd<BUS> = BusManager<::std::sync::Mutex<BUS>>;

/// A bus manager for safely sharing between tasks on Cortex-M.
///
/// This manager works by turning off interrupts for each bus transaction which prevents racy
/// accesses from different tasks/execution contexts (e.g. interrupts).  Usually, for sharing
/// between tasks, a manager with `'static` lifetime is needed which can be created using the
/// [`shared_bus::new_cortexm!()`][new_cortexm] macro.
///
/// [new_cortexm]: ./macro.new_cortexm.html
///
/// This type is only available with the `cortex-m` feature.
#[cfg(feature = "cortex-m")]
pub type BusManagerCortexM<BUS> = BusManager<CortexMMutex<BUS>>;

/// A bus manager for safely sharing between tasks on Xtensa-lx6.
///
/// This manager works by turning off interrupts for each bus transaction which prevents racy
/// accesses from different tasks/execution contexts (e.g. interrupts).  Usually, for sharing
/// between tasks, a manager with `'static` lifetime is needed which can be created using the
/// [`shared_bus::new_xtensa!()`][new_xtensa] macro.
///
/// [new_xtensa]: ./macro.new_xtensa.html
///
/// This type is only available with the `xtensa` feature.
#[cfg(feature = "xtensa")]
pub type BusManagerXtensa<BUS> = BusManager<XtensaMutex<BUS>>;

/// A bus manager for safely sharing the bus when using concurrency frameworks (such as RTIC).
///
/// This manager relies on RTIC or some other concurrency framework to manage resource
/// contention automatically. As a redundancy, this manager uses an atomic boolean to check
/// whether or not a resource is currently in use. This is purely used as a fail-safe against
/// misuse.
///
/// ## Warning
/// If devices on the same shared bus are not treated as a singular resource, it is possible that
/// pre-emption may occur. In this case, the manger will panic to prevent the race condition.
///
/// ## Usage
/// In order to use this manager with a concurrency framework such as RTIC, all devices on the
/// shared bus must be stored in the same logic resource. The concurrency framework will require a
/// resource lock if pre-emption is possible.
///
/// In order to use this with RTIC (as an example), all devices on the shared bus must be stored in
/// a singular resource.  Additionally, a manager with `'static` lifetime is needed which can be
/// created using the [`shared_bus::new_atomic_check!()`][new_atomic_check] macro.  It should
/// roughly look like this (there is also a [full example][shared-bus-rtic-example] available):
///
/// ```rust
/// struct Device<T> { bus: T };
/// struct OtherDevice<T> { bus: T };
///
/// // the HAL I2C driver type
/// type I2cType = ();
/// type Proxy = shared_bus::I2cProxy<'static, shared_bus::AtomicCheckMutex<I2cType>>;
///
/// struct SharedBusDevices {
///     device: Device<Proxy>,
///     other_device: OtherDevice<Proxy>,
/// }
///
/// struct Resources {
///     shared_bus_devices: SharedBusDevices,
/// }
///
///
/// // in the RTIC init function
/// fn init() -> Resources {
///     // init the bus like usual
///     let i2c: I2cType = ();
///
///     let bus_manager: &'static _ = shared_bus::new_atomic_check!(I2cType = i2c).unwrap();
///
///     let devices = SharedBusDevices {
///         device: Device { bus: bus_manager.acquire_i2c() },
///         other_device: OtherDevice { bus: bus_manager.acquire_i2c() },
///     };
///
///     Resources {
///         shared_bus_devices: devices,
///     }
/// }
/// ```
///
/// [new_atomic_check]: ./macro.new_atomic_check.html
/// [shared-bus-rtic-example]: https://github.com/ryan-summers/shared-bus-example/blob/master/src/main.rs
///
/// This type is only available with the `cortex-m` feature (but this may change in the future!).
#[cfg(feature = "cortex-m")]
pub type BusManagerAtomicCheck<T> = BusManager<AtomicCheckMutex<T>>;
