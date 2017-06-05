#![no_std]
#![allow(unused_unsafe)]

extern crate bobbin_common;
extern crate bobbin_cortexm;
extern crate stm32_common;
extern crate stm32l031x_chip as chip;

pub use bobbin_cortexm::hal::{nvic, scb};

pub mod pwr;
pub mod rcc;
pub mod clock;

pub mod gpio {
    pub use chip::gpio::*;
    pub use stm32_common::hal::gpio::*;
    pub use rcc::RccEnabled;
    use chip::sig::{SignalTx, SignalRx};

    pub trait ModeTx<T, S> {
        fn mode_tx(&self, _: &S) -> &Self;
    }

    pub trait ModeRx<T, S> {
        fn mode_rx(&self, _: &S) -> &Self;
    }

    impl<P, O, S, T> ModeTx<T, S> for Pin<P, O> where S: SignalTx<T>, P: AltFn<T> {
        fn mode_tx(&self, _: &S) -> &Self {
            self.mode_altfn(self.id.alt_fn());
            self
        }
    }

    impl<P, O, S, T> ModeRx<T, S> for Pin<P, O> where S: SignalRx<T>, P: AltFn<T> {
        fn mode_rx(&self, _: &S) -> &Self {
            self.mode_altfn(self.id.alt_fn());
            self
        }
    }

}

pub mod usart {
    pub use chip::usart::*;
    pub use stm32_common::hal::usart::*;
    pub use rcc::RccEnabled;
}

pub mod tim {
    pub use chip::tim_gen::*;
    pub use stm32_common::hal::tim_gen::*;
    pub use rcc::RccEnabled;
}