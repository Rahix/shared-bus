use std::sync;

use hal;

#[derive(Debug, Clone)]
pub struct FakeI2CDevice {
    pub transactions: sync::Arc<sync::RwLock<Vec<String>>>,
}

impl FakeI2CDevice {
    pub fn new() -> (FakeI2CDevice, sync::Arc<sync::RwLock<Vec<String>>>) {
        let transactions = sync::Arc::new(sync::RwLock::new(vec![]));
        (
            FakeI2CDevice { transactions: transactions.clone() },
            transactions,
        )
    }
}

impl hal::blocking::i2c::Write for FakeI2CDevice {
    type Error = ();

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut string: String = format!("{:02X}:", addr);
        for byte in bytes.iter() {
            string.push_str(&format!(" {:02X}", byte));
        }
        let mut lock = self.transactions.write().unwrap();
        lock.push(string);
        Ok(())
    }
}
