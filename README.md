shared-bus [![crates.io page](http://meritbadge.herokuapp.com/shared-bus)](https://crates.io/crates/shared-bus) [![Build Status](https://travis-ci.org/Rahix/shared-bus.svg?branch=master)](https://travis-ci.org/Rahix/shared-bus)
==========

**shared-bus** is a crate to allow sharing bus peripherals safely between multiple devices.

Typical usage of this crate might look like this:
```rust
extern crate shared_bus;

// Create your bus peripheral as usual:
let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);

let manager = shared_bus::BusManager::<std::sync::Mutex<_>, _>::new(i2c);

// You can now acquire bus handles:
let mut handle = manager.acquire();
// handle implements `i2c::{Read, Write, WriteRead}`, depending on the
// implementations of the underlying peripheral
let mut mydevice = MyDevice::new(manager.acquire());
```

## Mutex Implementation
To do its job, **shared-bus** needs a mutex. Because each platform has its own
mutex type, **shared-bus** uses an abstraction: `BusMutex`. This type
needs to be implemented for your platforms mutex type to allow using this
crate.

* If `std` is available, activate the `std` feature to enable the implementation
of `BusMutex` for `std::sync::Mutex`.
* If your device used `cortex-m`, activate the `cortexm` feature to enable the implementation
of `BusMutex` for `cortex_m::interrupt::Mutex`.
* If neither is the case, you need to implement a mutex yourself:

```rust
extern crate shared_bus;
extern crate cortex_m;

// You need a newtype because you can't implement foreign traits on
// foreign types.
struct MyMutex<T>(cortex_m::interrupt::Mutex<T>);

impl<T> shared_bus::BusMutex<T> for MyMutex<T> {
    fn create(v: T) -> MyMutex<T> {
        MyMutex(cortex_m::interrupt::Mutex::new(v))
    }

    fn lock<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
        cortex_m::interrupt::free(|cs| {
            let v = self.0.borrow(cs);
            f(v)
        })
    }
}

type MyBusManager<L, P> = shared_bus::BusManager<MyMutex<L>, P>;
```

I am welcoming patches containing mutex implementations for other platforms!

## License
shared-bus is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
