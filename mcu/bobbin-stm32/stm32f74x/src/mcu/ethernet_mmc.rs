#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::ethernet_mmc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( ETHERNET_MMC, EthernetMmc, ETHERNET_MMC_PERIPH, EthernetMmcPeriph, ETHERNET_MMC_OWNED, ETHERNET_MMC_REF_COUNT, 0x40028100, 0x00, 0x06);


