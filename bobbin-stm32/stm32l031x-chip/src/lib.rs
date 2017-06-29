#![no_std]
extern crate bobbin_cortexm;
pub use bobbin_cortexm::chip::exc;
pub use bobbin_cortexm::chip::nvic;
pub use bobbin_cortexm::chip::scb;
pub use bobbin_cortexm::chip::systick;
pub use bobbin_cortexm::chip::mpu;
pub use bobbin_cortexm::chip::fpu;

extern crate stm32_common;

pub mod irq;
pub mod sig;
pub mod flash;
pub mod pwr;
pub mod rcc;
pub mod syscfg;
pub mod iwdg;
pub mod wwdg;
pub mod crc;
pub mod rtc;
pub mod lptim;
pub mod exti;
pub mod dma;
pub mod i2c;
pub mod tim_gen;
pub mod tim_bas;
pub mod gpio;
pub mod usart;
pub mod lpuart;
pub mod spi;
