# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - XXX
### Changed
- **BREAKING** Replaced `atomic-polyfill` with `portable-atomic` ^1.6 ([#58]).

[#58]: hhttps://github.com/Rahix/shared-bus/pull/58


## [0.3.1] - 2023-10-31
### Added
- Implementations of the `WriteIter` and `WriteIterRead` traits from `embedded-hal` ([#53]).

[#53]: https://github.com/Rahix/shared-bus/pull/53


## [0.3.0] - 2023-08-13
### Changed
- **BREAKING** Upgraded to `cortex-m` 0.7.
- **BREAKING** Upgraded to `atomic-polyfill` 1.0.
- **BREAKING** Upgraded to `xtensa-lx` 0.8 ([#49]).

[#49]: https://github.com/Rahix/shared-bus/pull/49


## [0.2.5] - 2022-11-23
### Changed
- Upgraded to embedded-hal alpha support to version 9.  Please pin your
  dependency on `shared-bus` to an exact version (using `=0.#.#`) when you make
  use of the `eh-alpha` feature flag ([#38]).

### Fixed
- Fixed the `new_xtensa!()` macro not compiling ([#40]).

[#38]: https://github.com/Rahix/shared-bus/pull/38
[#40]: https://github.com/Rahix/shared-bus/pull/40


## [0.2.4] - 2022-06-16
### Added
- Added support for the I2C traits from the `embedded-hal` alpha 8 version
  (`eh-alpha` feature flag) ([#36]).

### Fixed
- Fixed the `atomic-polyfill` dependency being pulled in even in cases where it
  is not used.
- Fixed a problem with the ADC sharing `embedded-hal` dependency.

[#36]: https://github.com/Rahix/shared-bus/pull/36


## [0.2.3] - 2022-03-05
### Added
- A mutex type for Xtensa LX6 micorontrollers (e.g. ESP32) ([#22]).
- A proxy type for sharing an ADC to allow "concurrently" reading its channels
  from multiple tasks ([#24]).
- Support for thumbv6 targets by using `atomic-polyfill` instead of libcore
  atomics ([#30]).

[#22]: https://github.com/rahix/shared-bus/pull/22
[#24]: https://github.com/rahix/shared-bus/pull/24
[#30]: https://github.com/rahix/shared-bus/pull/30


## [0.2.2] - 2021-04-20
### Changed
- Updated the documentation on how to use `shared-bus` with RTIC.
- Removed superfluous trait bounds on the types - the trait bounds are really
  only necessary on the `impl`-blocks.

### Fixed
- Fixed the `Clone` implementations from 0.2.1 being useless due to an
  unnecessary trait bound.


## [0.2.1] - 2021-04-20
### Added
- Added a new "Mutex" and bus-manager for compatibility with RTIC.  Check the
  documentation for the `BusManagerAtomicCheck` for details.

### Changed
- `I2cProxy` and `SpiProxy` now implement `Clone`.  Cloning a proxy might be
  useful if a driver wants to split bus access up even further internally
  without wrapping the bus-proxy in another bus-manager.


## [0.2.0] - 2020-08-24
Complete rework of the crate, most items have changed at least slightly.
Please read the updated documentation to understand how the new version
works.

### Added
- A `BusMutexSimple` for sharing within a single task/thread with minimal
  overhead.
- Macros for instanciating a 'global' bus manager which lives for `'static`.

### Changed
- The `BusMutex` trait's `lock()` method now passes `&mut` to the closure,
  removing the `RefCell` from the manager.
- The generic parameter of `BusMutex` was moved into an associated type.
- Instead of a single proxy-type for everything, separate proxy types were
  introduced, to allow different constraints on their creation.

### Fixed
- The SPI proxy is now `!Send` to make sure it can only be used from
  within a single thread/task.


## 0.1.4 - 2018-11-04
### Changed
- Documentation fixes.


## 0.1.3 - 2018-10-30
### Added
- Added an SPI proxy.


## 0.1.2 - 2018-08-14
### Changed
- Documentation fixes.


## 0.1.1 - 2018-08-13
### Changed
- Documentation fixes.


## 0.1.0 - 2018-08-13
Initial release

[Unreleased]: https://github.com/Rahix/shared-bus/compare/v0.3.1...main
[0.3.1]: https://github.com/Rahix/shared-bus/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Rahix/shared-bus/compare/v0.2.5...v0.3.0
[0.2.5]: https://github.com/Rahix/shared-bus/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/Rahix/shared-bus/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/Rahix/shared-bus/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/Rahix/shared-bus/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/Rahix/shared-bus/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Rahix/shared-bus/compare/e24defd5c802...v0.2.0
