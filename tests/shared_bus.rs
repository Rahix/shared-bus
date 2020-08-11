use embedded_hal as hal;
use embedded_hal_mock as hal_mock;
use shared_bus;

use hal::blocking::i2c::Write;
use hal::blocking::spi::Write as SPIWrite;

use hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use hal_mock::spi::{Mock as SpiMock, Transaction as SpiTransaction};

#[test]
fn fake_i2c_device() {
    let expect = vec![I2cTransaction::write(0xc0, vec![0xff, 0xee])];
    let mut device = I2cMock::new(&expect);
    device.write(0xc0, &[0xff, 0xee]).unwrap();
    device.done()
}

#[test]
fn fake_spi_device() {
    let expect = vec![SpiTransaction::write(vec![0xff, 0xee])];
    let mut device = SpiMock::new(&expect);
    device.write(&[0xff, 0xee]).unwrap();
    device.done()
}

#[test]
fn spi_manager() {
    let expect = vec![];
    let mut device = SpiMock::new(&expect);
    let _manager = shared_bus::StdBusManager::new(device.clone());
    device.done();
}

#[test]
fn i2c_manager() {
    let expect = vec![];
    let mut device = I2cMock::new(&expect);
    let _manager = shared_bus::StdBusManager::new(device.clone());
    device.done();
}

#[test]
fn i2c_proxy() {
    let expect = vec![I2cTransaction::write(0xde, vec![0xad, 0xbe, 0xef])];
    let mut device = I2cMock::new(&expect);

    let manager = shared_bus::StdBusManager::new(device.clone());
    let mut proxy = manager.acquire();

    proxy.write(0xde, &[0xad, 0xbe, 0xef]).unwrap();

    device.done();
}

#[test]
fn spi_proxy() {
    let expect = vec![SpiTransaction::write(vec![0xde, 0xad, 0xbe, 0xef])];
    let mut device = SpiMock::new(&expect);

    let manager = shared_bus::StdBusManager::new(device.clone());
    let mut proxy = manager.acquire();

    proxy.write(&[0xde, 0xad, 0xbe, 0xef]).unwrap();

    device.done();
}

#[test]
fn multiple_proxies() {
    let expect = vec![
        I2cTransaction::write(0x0a, vec![0xab, 0xcd]),
        I2cTransaction::write(0x0b, vec![0x01, 0x23]),
        I2cTransaction::write(0x0a, vec![0x00, 0xff]),
    ];
    let mut device = I2cMock::new(&expect);

    let manager = shared_bus::StdBusManager::new(device.clone());

    let mut proxy1 = manager.acquire();
    let mut proxy2 = manager.acquire();

    proxy1.write(0x0A, &[0xab, 0xcd]).unwrap();
    proxy2.write(0x0B, &[0x01, 0x23]).unwrap();
    proxy1.write(0x0A, &[0x00, 0xFF]).unwrap();

    device.done()
}

#[test]
fn null_manager() {
    let expect = vec![
        I2cTransaction::write(0x0a, vec![0xab, 0xcd]),
        I2cTransaction::write(0x0b, vec![0x01, 0x23]),
        I2cTransaction::write(0x0a, vec![0x00, 0xff]),
    ];
    let mut device = I2cMock::new(&expect);

    let manager = shared_bus::SingleContextBusManager::new(device.clone());

    let mut proxy1 = manager.acquire();
    let mut proxy2 = manager.acquire();

    proxy1.write(0x0A, &[0xab, 0xcd]).unwrap();
    proxy2.write(0x0B, &[0x01, 0x23]).unwrap();
    proxy1.write(0x0A, &[0x00, 0xFF]).unwrap();

    device.done()
}
