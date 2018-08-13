extern crate embedded_hal as hal;
extern crate shared_bus;

mod i2c_mock;

use hal::blocking::i2c::Write;

#[test]
fn fake_device() {
    let (mut device, transactions) = i2c_mock::FakeI2CDevice::new();
    device.write(0xc0, &[0xff, 0xee]).unwrap();

    assert_eq!(*transactions.read().unwrap(), vec!["C0: FF EE"]);
}

#[test]
fn manager() {
    let (mut device, transactions) = i2c_mock::FakeI2CDevice::new();
    device.write(0xc0, &[0xff, 0xee]).unwrap();
    let _manager = shared_bus::BusManager::<std::sync::Mutex<_>, _>::new(device);

    assert_eq!(*transactions.read().unwrap(), vec!["C0: FF EE"]);
}

#[test]
fn proxy() {
    let (mut device, transactions) = i2c_mock::FakeI2CDevice::new();
    device.write(0xc0, &[0xff, 0xee]).unwrap();

    let manager = shared_bus::BusManager::<std::sync::Mutex<_>, _>::new(device);
    let mut proxy = manager.acquire();

    proxy.write(0xde, &[0xad, 0xbe, 0xef]).unwrap();

    assert_eq!(*transactions.read().unwrap(), vec!["C0: FF EE", "DE: AD BE EF"]);
}

#[test]
fn multiple_proxies() {
    let (device, transactions) = i2c_mock::FakeI2CDevice::new();
    let manager = shared_bus::BusManager::<std::sync::Mutex<_>, _>::new(device);

    let mut proxy1 = manager.acquire();
    let mut proxy2 = manager.acquire();

    proxy1.write(0x0A, &[0xab, 0xcd]).unwrap();
    proxy2.write(0x0B, &[0x01, 0x23]).unwrap();
    proxy1.write(0x0A, &[0x00, 0xFF]).unwrap();

    assert_eq!(*transactions.read().unwrap(), vec![
        "0A: AB CD",
        "0B: 01 23",
        "0A: 00 FF",
    ]);
}
