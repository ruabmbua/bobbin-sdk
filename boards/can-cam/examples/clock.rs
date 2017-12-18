#![no_std]
#![no_main]

#[macro_use]
extern crate can_cam as board;

use board::clock::CLK;
use board::common::digital::DigitalOutput;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("{:?}", CLK);
    loop {
        println!("{:?}", CLK);
        board::delay(1000);
        board::led::LED0.toggle_output();
    }
}
