shared-bus [![crates.io page](https://img.shields.io/crates/v/shared-bus)](https://crates.io/crates/shared-bus) [![docs.rs](https://docs.rs/shared-bus/badge.svg)](https://docs.rs/shared-bus) [![Continuous Integration](https://github.com/Rahix/shared-bus/actions/workflows/ci.yml/badge.svg)](https://github.com/Rahix/shared-bus/actions/workflows/ci.yml)
==========

**shared-bus** is a crate to allow sharing bus peripherals safely between multiple devices.

In the `embedded-hal` ecosystem, it is convention for drivers to "own" the bus peripheral they
are operating on.  This implies that only _one_ driver can have access to a certain bus.  That,
of course, poses an issue when multiple devices are connected to a single bus.

_shared-bus_ solves this by giving each driver a bus-proxy to own which internally manages
access to the actual bus in a safe manner.  For a more in-depth introduction of the problem
this crate is trying to solve, take a look at the [blog post][blog-post].

There are different 'bus managers' for different use-cases:

# Sharing within a single task/thread
As long as all users of a bus are contained in a single task/thread, bus sharing is very
simple.  With no concurrency possible, no special synchronization is needed.  This is where
a [`BusManagerSimple`] should be used:

```rust
// For example:
let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);

let bus = shared_bus::BusManagerSimple::new(i2c);

let mut proxy1 = bus.acquire_i2c();
let mut my_device = MyDevice::new(bus.acquire_i2c());

proxy1.write(0x39, &[0xc0, 0xff, 0xee]);
my_device.do_something_on_the_bus();
```

The `BusManager::acquire_*()` methods can be called as often as needed; each call will yield
a new bus-proxy of the requested type.

# Sharing across multiple tasks/threads
For sharing across multiple tasks/threads, synchronization is needed to ensure all bus-accesses
are strictly serialized and can't race against each other.  The synchronization is handled by
a platform-specific [`BusMutex`] implementation.  _shared-bus_ already contains some
implementations for common targets.  For each one, there is also a macro for easily creating
a bus-manager with `'static` lifetime, which is almost always a requirement when sharing across
task/thread boundaries.  As an example:

```rust
// For example:
let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 90.khz(), clocks, &mut rcc.apb1);

// The bus is a 'static reference -> it lives forever and references can be
// shared with other threads.
let bus: &'static _ = shared_bus::new_std!(SomeI2cBus = i2c).unwrap();

let mut proxy1 = bus.acquire_i2c();
let mut my_device = MyDevice::new(bus.acquire_i2c());

// We can easily move a proxy to another thread:
# let t =
std::thread::spawn(move || {
    my_device.do_something_on_the_bus();
});
# t.join().unwrap();
```

Those platform-specific bits are guarded by a feature that needs to be enabled.  Here is an
overview of what's already available:

| Mutex | Bus Manager | `'static` Bus Macro | Feature Name |
| --- | --- | --- | --- |
| `std::sync::Mutex` | [`BusManagerStd`] | [`new_std!()`] | `std` |
| `cortex_m::interrupt::Mutex` | [`BusManagerCortexM`] | [`new_cortexm!()`] | `cortex-m` |
| `shared_bus::XtensaMutex` (`spin::Mutex` in critical section) | [`BusManagerXtensa`] | [`new_xtensa!()`] | `xtensa` |
| NA | [`BusManagerAtomicCheck`] | [`new_atomic_check!()`] | `cortex-m` |

# Supported Busses
Currently, the following busses can be shared with _shared-bus_:

| Bus | Proxy Type | Acquire Method | Comments |
| --- | --- | --- | --- |
| I2C | [`I2cProxy`] | [`.acquire_i2c()`] | |
| SPI | [`SpiProxy`] | [`.acquire_spi()`] | SPI can only be shared within a single task (See [`SpiProxy`] for details). |
| ADC | [`AdcProxy`] | [`.acquire_adc()`] | |


[`.acquire_i2c()`]: https://docs.rs/shared-bus/latest/shared_bus/struct.BusManager.html#method.acquire_i2c
[`.acquire_spi()`]: https://docs.rs/shared-bus/latest/shared_bus/struct.BusManager.html#method.acquire_spi
[`.acquire_adc()`]: https://docs.rs/shared-bus/latest/shared_bus/struct.BusManager.html#method.acquire_adc
[`BusManagerCortexM`]: https://docs.rs/shared-bus/latest/shared_bus/type.BusManagerCortexM.html
[`BusManagerSimple`]: https://docs.rs/shared-bus/latest/shared_bus/type.BusManagerSimple.html
[`BusManagerAtomicCheck`]: https://docs.rs/shared-bus/latest/shared_bus/type.BusManagerAtomicCheck.html
[`BusManagerStd`]: https://docs.rs/shared-bus/latest/shared_bus/type.BusManagerStd.html
[`BusManagerXtensa`]: https://docs.rs/shared-bus/latest/shared_bus/type.BusManagerXtensa.html
[`BusMutex`]: https://docs.rs/shared-bus/latest/shared_bus/trait.BusMutex.html
[`I2cProxy`]: https://docs.rs/shared-bus/latest/shared_bus/struct.I2cProxy.html
[`SpiProxy`]: https://docs.rs/shared-bus/latest/shared_bus/struct.SpiProxy.html
[`AdcProxy`]: https://docs.rs/shared-bus/latest/shared_bus/struct.AdcProxy.html
[`new_cortexm!()`]: https://docs.rs/shared-bus/latest/shared_bus/macro.new_cortexm.html
[`new_atomic_check!()`]: https://docs.rs/shared-bus/latest/shared_bus/macro.new_atomic_check.html
[`new_xtensa!()`]: https://docs.rs/shared-bus/latest/shared_bus/macro.new_xtensa.html
[`new_std!()`]: https://docs.rs/shared-bus/latest/shared_bus/macro.new_std.html
[blog-post]: https://blog.rahix.de/001-shared-bus

## License
shared-bus is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
