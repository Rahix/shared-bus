use embedded_hal::prelude::*;
use embedded_hal_mock::spi;

#[test]
fn fake_spi_device() {
    let expect = vec![spi::Transaction::write(vec![0xff, 0xee])];
    let mut device = spi::Mock::new(&expect);
    device.write(&[0xff, 0xee]).unwrap();
    device.done()
}

#[test]
fn spi_manager_manual() {
    let expect = vec![spi::Transaction::write(vec![0xab, 0xcd, 0xef])];
    let mut device = spi::Mock::new(&expect);
    let manager = shared_bus::BusManagerSimple::new(device.clone());
    let mut proxy = manager.acquire_spi();

    proxy.write(&[0xab, 0xcd, 0xef]).unwrap();

    device.done();
}

#[test]
fn spi_proxy() {
    let expect = vec![
        spi::Transaction::write(vec![0xab, 0xcd, 0xef]),
        spi::Transaction::transfer(vec![0x01, 0x02], vec![0x03, 0x04]),
    ];
    let mut device = spi::Mock::new(&expect);
    let manager = shared_bus::BusManagerSimple::new(device.clone());
    let mut proxy = manager.acquire_spi();

    proxy.write(&[0xab, 0xcd, 0xef]).unwrap();

    let mut buf = vec![0x01, 0x02];
    proxy.transfer(&mut buf).unwrap();
    assert_eq!(&buf, &[0x03, 0x04]);

    device.done();
}

#[test]
fn spi_multi() {
    let expect = vec![
        spi::Transaction::write(vec![0xab, 0xcd, 0xef]),
        spi::Transaction::transfer(vec![0x01, 0x02], vec![0x03, 0x04]),
    ];
    let mut device = spi::Mock::new(&expect);
    let manager = shared_bus::BusManagerSimple::new(device.clone());
    let mut proxy1 = manager.acquire_spi();
    let mut proxy2 = manager.acquire_spi();

    proxy1.write(&[0xab, 0xcd, 0xef]).unwrap();

    let mut buf = vec![0x01, 0x02];
    proxy2.transfer(&mut buf).unwrap();
    assert_eq!(&buf, &[0x03, 0x04]);

    device.done();
}
