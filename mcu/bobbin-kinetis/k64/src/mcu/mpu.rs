#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::mpu::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( MPU, Mpu, MPU_PERIPH, MpuPeriph, 0x4000d000, 0x02);

