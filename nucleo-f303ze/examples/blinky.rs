#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f303ze as board;

use board::driver::gpio::{self, GPIOA};

// LED @ D13 = PA5

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
            
    let led = gpio::pin(GPIOA, 5).into_digital_output();

    loop {
        led.set(true);
        delay(1_000_000);
        led.set(false);
        delay(1_000_000);
    }
}

fn delay(n: usize) {
    for _ in 0..n {
        unsafe { asm!("nop") }
    }
}
