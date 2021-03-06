use bobbin_mcu::pin::Pin;
use bobbin_mcu::gate::GateEn;
pub use mcu::pin::*;
pub use bobbin_hal::btn::*;


// BTN0 / PA12 / D2 - Active Low

pub const BTN0: Pa12 = PA12;

pub fn init() {
    BTN0.port().gate_enable();
    BTN0.mode_input().pull_up();
}
pub use mcu::pin::*;

impl ::Board {
    pub fn btn0(&self) -> BtnLow<GpioPin> { BtnLow::new(PA12_PIN) }
}