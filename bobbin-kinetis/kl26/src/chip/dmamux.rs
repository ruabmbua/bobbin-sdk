#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::dmamux::*;

pub const DMAMUX: Dmamux = Periph(0x40021000, DmamuxId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct DmamuxId {}
pub type Dmamux = Periph<DmamuxId>;


