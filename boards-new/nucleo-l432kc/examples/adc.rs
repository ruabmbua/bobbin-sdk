#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::adc::*;
use board::mcu::c_adc::*;
use board::clock::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");

    println!("clock for ADC1: {:?}", tree().u32_for(ADC1));

    
    let a0 = PA0;  // A0
    let a1 = PA1;  // A1

    let adc = ADC1;
    let ch1 = ADC1_CH5;
    let ch2 = ADC1_CH6;

    adc.gate_enable();
    a0.port().gate_enable();
    a1.port().gate_enable();

    // a0.connect_to(ch1);
    // a1.connect_to(ch2);

    println!("Initializing");
    C_ADC.with_ccr(|r| r.set_ckmode(0b01));
    println!("C_ADC[CCR]: {:?}", C_ADC.ccr());

    println!("Exit deep power down mode");
    adc.with_cr(|r| r.set_deeppwd(0));
    adc.init();

    println!("Initialized...");
    loop {        
        let v0: U12 = ch1.analog_read();
        let v1: U10 = ch2.analog_read();
        println!("{} {}", v0, v1);
        board::delay(1_000);
    }
}
