use chip::gpio::*;
use hal::rcc;
use hal::gpio;

pub type Led = gpio::PinOutput;

// LED @ D13 = PB3
pub fn led0() -> Led {
    let pin = PB3;
    rcc::set_gpio_enabled(pin.port(), true);
    gpio::pin((pin.port(), pin.index())).into_output()
}