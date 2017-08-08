pub use chip::crc_24::*;

pub trait CrcExt {
    fn reset(&self) -> &Self;
    fn write(&self, value: u32) -> &Self;
    fn read(&self) -> u32;
}

impl<T> CrcExt for Periph<T> {
    fn reset(&self) -> &Self {    
        self.set_cr(|r| r.set_reset(1))
    }

    fn write(&self, value: u32) -> &Self {
        self.set_dr(|r| r.set_dr(value))
    }

    fn read(&self) -> u32 {
        self.dr().dr().into()
    }
}