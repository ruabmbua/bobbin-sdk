#![no_std]
#![no_main]

#[macro_use]
extern crate blue_f4 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for nucleo-f303ze");
    test_systick();
    println!("[done] All tests passed");
    loop {}
}

fn test_systick() {
    use board::hal::systick::*;

    println!("# Testing SYSTICK");
    test_systick(&SYSTICK, ClockSource::Internal);
    println!("[pass] SYSTICK OK");
}