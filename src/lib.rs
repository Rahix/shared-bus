#![doc(html_root_url = "https://docs.rs/shared-bus")]
#![cfg_attr(not(feature = "std"), no_std)]

mod manager;
mod mutex;
mod proxies;
mod macros;

#[doc(hidden)]
#[cfg(feature = "std")]
pub use once_cell;

#[doc(hidden)]
#[cfg(feature = "cortex-m")]
pub use cortex_m;

pub use manager::BusManager;
pub use mutex::BusMutex;
pub use mutex::NullMutex;
#[cfg(feature = "cortex-m")]
pub use mutex::CortexMMutex;
pub use proxies::I2cProxy;
pub use proxies::SpiProxy;

pub type BusManagerSimple<BUS> = BusManager<NullMutex<BUS>>;

#[cfg(feature = "std")]
pub type BusManagerStd<BUS> = BusManager<::std::sync::Mutex<BUS>>;

#[cfg(feature = "cortex-m")]
pub type BusManagerCortexM<BUS> = BusManager<CortexMMutex<BUS>>;
