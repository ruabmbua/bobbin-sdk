#![no_std]
#![feature(const_fn)]

#[cfg(not(target_os="none"))]
#[macro_use]
extern crate std;

pub extern crate bobbin_hz as hz;
extern crate bobbin_bits as bits;
pub extern crate bobbin_tree as tree;

#[macro_use]
mod macros;

pub mod mcu;
pub mod clock;
pub mod register;
pub mod periph;
pub mod pin;
pub mod channel;
pub mod irq;
pub mod signal;
pub mod gate;
pub mod owned;

pub use register::*;
pub use periph::*;
pub use pin::*;
pub use channel::*;
pub use irq::*;

#[cfg(not(target_os="none"))]
pub mod vm;

#[cfg(target_os="none")]
pub use core::ptr::{read_volatile, write_volatile};

#[cfg(not(target_os="none"))]
pub mod rw;

#[cfg(not(target_os="none"))]
pub use rw::*;
pub trait En {
    fn en(&self) -> bits::U1;
    fn set_en<V: Into<bits::U1>>(&self, value: V);
}    