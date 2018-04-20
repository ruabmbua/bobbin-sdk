#![no_std]
#![feature(asm)]

#[cfg(feature="logger")]
extern crate log;

extern crate bobbin_mcu;
extern crate bobbin_hal;

#[macro_use]
mod macros;

pub mod prelude;
pub mod console;
pub mod heap;
pub mod tick;
pub mod memory;
pub mod irq_dispatch;
pub mod ring;
pub mod system;
pub mod board;

#[cfg(feature="logger")]
pub mod logger;