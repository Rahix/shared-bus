#![cfg_attr(not(feature = "std"), no_std)]

extern crate embedded_hal as hal;

pub mod mutex;
pub mod proxy;

pub use mutex::BusMutex;
pub use proxy::BusManager;
pub use proxy::BusProxy;
