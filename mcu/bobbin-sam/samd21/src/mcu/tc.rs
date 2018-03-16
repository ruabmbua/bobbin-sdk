#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::tc::*;

// //! Timer/Counter

periph!( TC3, Tc3, TC3_PERIPH, TcPeriph, 0x42002c00, 0x0b);
periph!( TC4, Tc4, TC4_PERIPH, TcPeriph, 0x42003000, 0x0c);
periph!( TC5, Tc5, TC5_PERIPH, TcPeriph, 0x42003400, 0x0d);

pub struct TcCh { pub periph: TcPeriph, pub index: usize }
channel!(TC3_CH0, Tc3Ch0, TC3, Tc3, TC3_CH0_CH, TcCh, TC3_PERIPH, 0);
channel!(TC3_CH1, Tc3Ch1, TC3, Tc3, TC3_CH1_CH, TcCh, TC3_PERIPH, 1);
channel!(TC4_CH0, Tc4Ch0, TC4, Tc4, TC4_CH0_CH, TcCh, TC4_PERIPH, 0);
channel!(TC4_CH1, Tc4Ch1, TC4, Tc4, TC4_CH1_CH, TcCh, TC4_PERIPH, 1);
channel!(TC5_CH0, Tc5Ch0, TC5, Tc5, TC5_CH0_CH, TcCh, TC5_PERIPH, 0);
channel!(TC5_CH1, Tc5Ch1, TC5, Tc5, TC5_CH1_CH, TcCh, TC5_PERIPH, 1);
