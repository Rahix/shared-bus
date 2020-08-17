#![doc(html_root_url = "https://docs.rs/shared-bus")]
#![cfg_attr(not(feature = "std"), no_std)]

mod mutex;
mod manager;
mod proxies;

pub use mutex::BusMutex;
pub use manager::BusManager;
pub use proxies::I2cProxy;

#[cfg(feature = "std")]
pub type BusManagerStd<BUS> = BusManager<::std::sync::Mutex<BUS>>;
