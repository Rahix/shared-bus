use embedded_hal::prelude::*;
use embedded_hal_mock::i2c;
use std::thread;

#[test]
fn fake_i2c_device() {
    let expect = vec![i2c::Transaction::write(0xc0, vec![0xff, 0xee])];
    let mut device = i2c::Mock::new(&expect);
    device.write(0xc0, &[0xff, 0xee]).unwrap();
    device.done()
}

#[test]
fn i2c_manager_manual() {
    let expect = vec![i2c::Transaction::write(0xde, vec![0xad, 0xbe, 0xef])];
    let mut device = i2c::Mock::new(&expect);
    let manager = shared_bus::BusManagerSimple::new(device.clone());
    let mut proxy = manager.acquire_i2c();

    proxy.write(0xde, &[0xad, 0xbe, 0xef]).unwrap();

    device.done();
}

#[test]
fn i2c_manager_macro() {
    let expect = vec![i2c::Transaction::write(0xde, vec![0xad, 0xbe, 0xef])];
    let mut device = i2c::Mock::new(&expect);
    let manager: &'static shared_bus::BusManagerStd<_> =
        shared_bus::new_std!(i2c::Mock = device.clone()).unwrap();
    let mut proxy = manager.acquire_i2c();

    proxy.write(0xde, &[0xad, 0xbe, 0xef]).unwrap();

    device.done();
}

#[test]
fn i2c_proxy() {
    let expect = vec![
        i2c::Transaction::write(0xde, vec![0xad, 0xbe, 0xef]),
        i2c::Transaction::read(0xef, vec![0xbe, 0xad, 0xde]),
        i2c::Transaction::write_read(0x44, vec![0x01, 0x02], vec![0x03, 0x04]),
    ];
    let mut device = i2c::Mock::new(&expect);

    let manager = shared_bus::BusManagerSimple::new(device.clone());
    let mut proxy = manager.acquire_i2c();

    proxy.write(0xde, &[0xad, 0xbe, 0xef]).unwrap();

    let mut buf = [0u8; 3];
    proxy.read(0xef, &mut buf).unwrap();
    assert_eq!(&buf, &[0xbe, 0xad, 0xde]);

    let mut buf = [0u8; 2];
    proxy.write_read(0x44, &[0x01, 0x02], &mut buf).unwrap();
    assert_eq!(&buf, &[0x03, 0x04]);

    device.done();
}

#[test]
fn i2c_multi() {
    let expect = vec![
        i2c::Transaction::write(0xde, vec![0xad, 0xbe, 0xef]),
        i2c::Transaction::read(0xef, vec![0xbe, 0xad, 0xde]),
        i2c::Transaction::write_read(0x44, vec![0x01, 0x02], vec![0x03, 0x04]),
    ];
    let mut device = i2c::Mock::new(&expect);

    let manager = shared_bus::BusManagerSimple::new(device.clone());
    let mut proxy1 = manager.acquire_i2c();
    let mut proxy2 = manager.acquire_i2c();
    let mut proxy3 = manager.acquire_i2c();

    proxy1.write(0xde, &[0xad, 0xbe, 0xef]).unwrap();

    let mut buf = [0u8; 3];
    proxy2.read(0xef, &mut buf).unwrap();
    assert_eq!(&buf, &[0xbe, 0xad, 0xde]);

    let mut buf = [0u8; 2];
    proxy3.write_read(0x44, &[0x01, 0x02], &mut buf).unwrap();
    assert_eq!(&buf, &[0x03, 0x04]);

    device.done();
}

#[test]
fn i2c_concurrent() {
    let expect = vec![
        i2c::Transaction::write(0xde, vec![0xad, 0xbe, 0xef]),
        i2c::Transaction::read(0xef, vec![0xbe, 0xad, 0xde]),
    ];
    let mut device = i2c::Mock::new(&expect);

    let manager = shared_bus::new_std!(i2c::Mock = device.clone()).unwrap();
    let mut proxy1 = manager.acquire_i2c();
    let mut proxy2 = manager.acquire_i2c();

    thread::spawn(move || {
        proxy1.write(0xde, &[0xad, 0xbe, 0xef]).unwrap();
    })
    .join()
    .unwrap();

    thread::spawn(move || {
        let mut buf = [0u8; 3];
        proxy2.read(0xef, &mut buf).unwrap();
        assert_eq!(&buf, &[0xbe, 0xad, 0xde]);
    })
    .join()
    .unwrap();

    device.done();
}
