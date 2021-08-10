use embedded_hal::prelude::*;
use embedded_hal_mock::adc;
use std::thread;

#[test]
fn adc_mock_device() {
    let expectations = [
        adc::Transaction::read(0, 0xabcd),
        adc::Transaction::read(1, 0xabba),
        adc::Transaction::read(2, 0xbaab),
    ];

    let mut device = adc::Mock::new(&expectations);
    assert_eq!(0xabcd, device.read(&mut adc::MockChan0).unwrap());
    assert_eq!(0xabba, device.read(&mut adc::MockChan1).unwrap());
    assert_eq!(0xbaab, device.read(&mut adc::MockChan2).unwrap());
    device.done()
}

#[test]
fn adc_manager_simple() {
    let expectations = [
        adc::Transaction::read(0, 0xabcd),
        adc::Transaction::read(1, 0xabba),
        adc::Transaction::read(2, 0xbaab),
    ];

    let mut device = adc::Mock::new(&expectations);
    let manager = shared_bus::BusManagerSimple::new(device.clone());
    let mut proxy = manager.acquire_adc();

    assert_eq!(0xabcd, proxy.read(&mut adc::MockChan0).unwrap());
    assert_eq!(0xabba, proxy.read(&mut adc::MockChan1).unwrap());
    assert_eq!(0xbaab, proxy.read(&mut adc::MockChan2).unwrap());
    device.done()
}

#[test]
fn adc_manager_std() {
    let expectations = [
        adc::Transaction::read(0, 0xabcd),
        adc::Transaction::read(1, 0xabba),
        adc::Transaction::read(2, 0xbaab),
    ];

    let mut device = adc::Mock::new(&expectations);
    let manager: &'static shared_bus::BusManagerStd<_> =
        shared_bus::new_std!(adc::Mock<u16> = device.clone()).unwrap();
    let mut proxy = manager.acquire_adc();

    assert_eq!(0xabcd, proxy.read(&mut adc::MockChan0).unwrap());
    assert_eq!(0xabba, proxy.read(&mut adc::MockChan1).unwrap());
    assert_eq!(0xbaab, proxy.read(&mut adc::MockChan2).unwrap());
    device.done()
}

#[test]
fn adc_proxy_multi() {
    let expectations = [
        adc::Transaction::read(0, 0xabcd),
        adc::Transaction::read(1, 0xabba),
        adc::Transaction::read(2, 0xbaab),
    ];

    let mut device = adc::Mock::new(&expectations);
    let manager = shared_bus::BusManagerSimple::new(device.clone());
    let mut proxy1 = manager.acquire_adc();
    let mut proxy2 = manager.acquire_adc();
    let mut proxy3 = manager.acquire_adc();

    assert_eq!(0xabcd, proxy1.read(&mut adc::MockChan0).unwrap());
    assert_eq!(0xabba, proxy2.read(&mut adc::MockChan1).unwrap());
    assert_eq!(0xbaab, proxy3.read(&mut adc::MockChan2).unwrap());
    device.done()
}

#[test]
fn adc_proxy_concurrent() {
    let expectations = [
        adc::Transaction::read(0, 0xabcd),
        adc::Transaction::read(1, 0xabba),
        adc::Transaction::read(2, 0xbaab),
    ];

    let mut device = adc::Mock::new(&expectations);
    let manager: &'static shared_bus::BusManagerStd<_> =
        shared_bus::new_std!(adc::Mock<u32> = device.clone()).unwrap();
    let mut proxy1 = manager.acquire_adc();
    let mut proxy2 = manager.acquire_adc();
    let mut proxy3 = manager.acquire_adc();

    thread::spawn(move || {
        assert_eq!(0xabcd, proxy1.read(&mut adc::MockChan0).unwrap());
    })
    .join()
    .unwrap();

    thread::spawn(move || {
        assert_eq!(0xabba, proxy2.read(&mut adc::MockChan1).unwrap());
    })
    .join()
    .unwrap();

    assert_eq!(0xbaab, proxy3.read(&mut adc::MockChan2).unwrap());

    device.done()
}
