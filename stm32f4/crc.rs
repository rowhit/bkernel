//! CRC calculation unit.
use volatile::{WO, RW};

extern {
    pub static CRC: Crc;
}

/// Don't forget to enable CRC peripheral before use.
///
/// ```no_run
/// # use stm32f4::rcc;
/// rcc::RCC.ahb1_clock_enable(rcc::Ahb1Enable::CRC);
/// ```
#[repr(C)]
pub struct Crc {
    dr:  RW<u32>, // 0x0
    idr: RW<u32>, // 0x4
    cr:  WO<u32>, // 0x8
}

impl Crc {
    /// Resets the CRC Data register (DR).
    pub fn reset(&self) {
        unsafe {
            self.cr.set(0x1);
        }
    }

    /// Computes the 32-bit CRC of a given data word (32-bit).
    pub fn calculate_crc(&self, data: u32) -> u32 {
        unsafe {
            self.dr.set(data);
            self.dr.get()
        }
    }

    /// Returns the current CRC value.
    pub fn get_crc(&self) -> u32 {
        unsafe {
            self.dr.get()
        }
    }

    /// Stores 8-bit value in the Independent Data Register.
    pub fn set_idr(&self, value: u8) {
        unsafe {
            self.idr.set(value as u32);
        }
    }

    /// Reads 8-bit value from the Indenpendent Data Register.
    pub fn get_idr(&self) -> u8 {
        unsafe {
            self.idr.get() as u8
        }
    }

    pub fn block_crc(&self, data: &[u32]) -> u32 {
        unsafe {
            for x in data {
                self.dr.set(*x);
            }
            self.dr.get()
        }
    }
}
