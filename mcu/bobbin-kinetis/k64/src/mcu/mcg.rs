#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::mcg::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( MCG, Mcg, MCG_PERIPH, McgPeriph, MCG_OWNED, MCG_REF_COUNT, 0x40064000, 0x00, 0x01);


