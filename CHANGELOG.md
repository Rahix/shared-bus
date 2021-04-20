# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
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

[Unreleased]: https://github.com/Rahix/shared-bus/compare/v0.2.1...master
[0.2.1]: https://github.com/Rahix/shared-bus/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Rahix/shared-bus/compare/e24defd5c802...v0.2.0
