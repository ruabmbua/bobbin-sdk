#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let sys = board::init();

    println!("Running MsTick");
    let mt = ::board::ms_tick::MS_TICK;
    loop {
        if let Some(c) = sys.console() {
            c.write(b"Tick: ");
            c.write_u32(mt.counter(), 10);
            c.write(b"\r\n");
        }
        mt.delay(1000);
    }
}