#![doc(html_root_url = "https://docs.rs/shared-bus")]
#![cfg_attr(not(feature = "std"), no_std)]

mod manager;
mod mutex;
mod proxies;
mod macros;

#[doc(hidden)]
#[cfg(feature = "std")]
pub use once_cell;

pub use manager::BusManager;
pub use mutex::BusMutex;
pub use proxies::I2cProxy;

#[cfg(feature = "std")]
pub type BusManagerStd<BUS> = BusManager<::std::sync::Mutex<BUS>>;
