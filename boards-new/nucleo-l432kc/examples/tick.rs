#![no_std]
#![no_main]

extern crate nucleo_l432kc as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    
    let b = board::board();
    examples::tick::run(b);
}