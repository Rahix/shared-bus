//! # shared_bus
//!
//! `shared_bus` is a crate to allow sharing bus peripherals
//! safely between multiple devices.
//!
//! To do so, `shared_bus` needs a mutex. Because each platform has its own
//! mutex type, `shared_bus` uses an abstraction: [`BusMutex`]. This type
//! needs to be implemented for your platforms mutex type to allow using this
//! crate.
//! If `std` is available, activate the `std` feature to enable the implementation
//! of [`BusMutex`] for [`std::sync::Mutex`].
//!
//! Typical usage of this crate might look like this:
//! ```
//! extern crate shared_bus;
//! # struct MyDevice;
//! # impl MyDevice {
//! #     pub fn new<T>(t: T) -> MyDevice { MyDevice }
//! # }
//!
//! # let i2c = ();
//! // Create your bus peripheral as usual:
//! // let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);
//!
//! let manager = shared_bus::BusManager::<std::sync::Mutex<_>, _>::new(i2c);
//!
//! // You can now acquire bus handles:
//! let mut handle = manager.acquire();
//! // handle implements `i2c::{Read, Write, WriteRead}`, depending on the
//! // implementations of the underlying peripheral
//! let mut mydevice = MyDevice::new(manager.acquire());
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

extern crate embedded_hal as hal;

pub mod mutex;
pub mod proxy;

pub use mutex::BusMutex;
pub use proxy::BusManager;
pub use proxy::BusProxy;

/// Type alias for a bus manager using [`std::sync::Mutex`].
#[cfg(feature = "std")]
pub type StdBusManager<L, P> = BusManager<std::sync::Mutex<L>, P>;
