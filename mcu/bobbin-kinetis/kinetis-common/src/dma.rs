
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMA Peripheral"]
pub struct DmaPeriph(pub usize); 

pub struct DmaCh { pub periph: DmaPeriph, pub index: usize }

impl DmaPeriph {
    #[doc="Get the SAR Register."]
    #[inline] pub fn sar_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Sar, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Sar, 0x100, 0x10)
    }

    #[doc="Get the *mut pointer for the SAR register."]
    #[inline] pub fn sar_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Sar { 
        self.sar_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the SAR register."]
    #[inline] pub fn sar_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Sar { 
        self.sar_reg().ptr(index.into())
    }

    #[doc="Read the SAR register."]
    #[inline] pub fn sar<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Sar { 
        self.sar_reg().read(index.into())
    }

    #[doc="Write the SAR register."]
    #[inline] pub fn write_sar<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Sar) -> &Self {
        self.sar_reg().write(index.into(), value);
        self
    }

    #[doc="Set the SAR register."]
    #[inline] pub fn set_sar<I: Into<::bobbin_bits::R4>, F: FnOnce(Sar) -> Sar>(&self, index: I, f: F) -> &Self {
        self.sar_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the SAR register."]
    #[inline] pub fn with_sar<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Sar) -> Sar>(&self, index: I, f: F) -> &Self {
        self.sar_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DAR Register."]
    #[inline] pub fn dar_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dar, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dar, 0x104, 0x10)
    }

    #[doc="Get the *mut pointer for the DAR register."]
    #[inline] pub fn dar_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Dar { 
        self.dar_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DAR register."]
    #[inline] pub fn dar_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Dar { 
        self.dar_reg().ptr(index.into())
    }

    #[doc="Read the DAR register."]
    #[inline] pub fn dar<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Dar { 
        self.dar_reg().read(index.into())
    }

    #[doc="Write the DAR register."]
    #[inline] pub fn write_dar<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Dar) -> &Self {
        self.dar_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DAR register."]
    #[inline] pub fn set_dar<I: Into<::bobbin_bits::R4>, F: FnOnce(Dar) -> Dar>(&self, index: I, f: F) -> &Self {
        self.dar_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DAR register."]
    #[inline] pub fn with_dar<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Dar) -> Dar>(&self, index: I, f: F) -> &Self {
        self.dar_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DSR_BCR Register."]
    #[inline] pub fn dsr_bcr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<DsrBcr, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut DsrBcr, 0x108, 0x10)
    }

    #[doc="Get the *mut pointer for the DSR_BCR register."]
    #[inline] pub fn dsr_bcr_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut DsrBcr { 
        self.dsr_bcr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DSR_BCR register."]
    #[inline] pub fn dsr_bcr_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const DsrBcr { 
        self.dsr_bcr_reg().ptr(index.into())
    }

    #[doc="Read the DSR_BCR register."]
    #[inline] pub fn dsr_bcr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> DsrBcr { 
        self.dsr_bcr_reg().read(index.into())
    }

    #[doc="Write the DSR_BCR register."]
    #[inline] pub fn write_dsr_bcr<I: Into<::bobbin_bits::R4>>(&self, index: I, value: DsrBcr) -> &Self {
        self.dsr_bcr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DSR_BCR register."]
    #[inline] pub fn set_dsr_bcr<I: Into<::bobbin_bits::R4>, F: FnOnce(DsrBcr) -> DsrBcr>(&self, index: I, f: F) -> &Self {
        self.dsr_bcr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DSR_BCR register."]
    #[inline] pub fn with_dsr_bcr<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(DsrBcr) -> DsrBcr>(&self, index: I, f: F) -> &Self {
        self.dsr_bcr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DSR Register."]
    #[inline] pub fn dsr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dsr, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dsr, 0x10b, 0x10)
    }

    #[doc="Get the *mut pointer for the DSR register."]
    #[inline] pub fn dsr_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Dsr { 
        self.dsr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DSR register."]
    #[inline] pub fn dsr_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Dsr { 
        self.dsr_reg().ptr(index.into())
    }

    #[doc="Read the DSR register."]
    #[inline] pub fn dsr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Dsr { 
        self.dsr_reg().read(index.into())
    }

    #[doc="Write the DSR register."]
    #[inline] pub fn write_dsr<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Dsr) -> &Self {
        self.dsr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DSR register."]
    #[inline] pub fn set_dsr<I: Into<::bobbin_bits::R4>, F: FnOnce(Dsr) -> Dsr>(&self, index: I, f: F) -> &Self {
        self.dsr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DSR register."]
    #[inline] pub fn with_dsr<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Dsr) -> Dsr>(&self, index: I, f: F) -> &Self {
        self.dsr_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DCR Register."]
    #[inline] pub fn dcr_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dcr, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dcr, 0x10c, 0x10)
    }

    #[doc="Get the *mut pointer for the DCR register."]
    #[inline] pub fn dcr_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Dcr { 
        self.dcr_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DCR register."]
    #[inline] pub fn dcr_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Dcr { 
        self.dcr_reg().ptr(index.into())
    }

    #[doc="Read the DCR register."]
    #[inline] pub fn dcr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Dcr { 
        self.dcr_reg().read(index.into())
    }

    #[doc="Write the DCR register."]
    #[inline] pub fn write_dcr<I: Into<::bobbin_bits::R4>>(&self, index: I, value: Dcr) -> &Self {
        self.dcr_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DCR register."]
    #[inline] pub fn set_dcr<I: Into<::bobbin_bits::R4>, F: FnOnce(Dcr) -> Dcr>(&self, index: I, f: F) -> &Self {
        self.dcr_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DCR register."]
    #[inline] pub fn with_dcr<I: Into<::bobbin_bits::R4> + Copy, F: FnOnce(Dcr) -> Dcr>(&self, index: I, f: F) -> &Self {
        self.dcr_reg().with(index.into(), f);
        self
    }

}

#[doc="Source Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sar(pub u32);
impl Sar {
    #[doc="Each SAR contains the byte address used by the DMA controller to read data"]
    #[inline] pub fn sar(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SAR != 0"]
    #[inline] pub fn test_sar(&self) -> bool {
        self.sar() != 0
    }

    #[doc="Sets the SAR field."]
    #[inline] pub fn set_sar<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sar {
    #[inline]
    fn from(other: u32) -> Self {
         Sar(other)
    }
}

impl ::core::fmt::Display for Sar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Destination Address Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dar(pub u32);
impl Dar {
    #[doc="Each DAR contains the byte address used by the DMA controller to write data"]
    #[inline] pub fn dar(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if DAR != 0"]
    #[inline] pub fn test_dar(&self) -> bool {
        self.dar() != 0
    }

    #[doc="Sets the DAR field."]
    #[inline] pub fn set_dar<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dar {
    #[inline]
    fn from(other: u32) -> Self {
         Dar(other)
    }
}

impl ::core::fmt::Display for Dar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Status Register / Byte Count Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DsrBcr(pub u32);
impl DsrBcr {
    #[doc="This field contains the number of bytes yet to be transferred for a given block"]
    #[inline] pub fn bcr(&self) -> ::bobbin_bits::U24 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffff) as u32) } // [23:0]
    }

    #[doc="Returns true if BCR != 0"]
    #[inline] pub fn test_bcr(&self) -> bool {
        self.bcr() != 0
    }

    #[doc="Sets the BCR field."]
    #[inline] pub fn set_bcr<V: Into<::bobbin_bits::U24>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U24 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transactions Done"]
    #[inline] pub fn done(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if DONE != 0"]
    #[inline] pub fn test_done(&self) -> bool {
        self.done() != 0
    }

    #[doc="Sets the DONE field."]
    #[inline] pub fn set_done<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn bsy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if BSY != 0"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Sets the BSY field."]
    #[inline] pub fn set_bsy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Request"]
    #[inline] pub fn req(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if REQ != 0"]
    #[inline] pub fn test_req(&self) -> bool {
        self.req() != 0
    }

    #[doc="Sets the REQ field."]
    #[inline] pub fn set_req<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Bus Error on Destination"]
    #[inline] pub fn bed(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if BED != 0"]
    #[inline] pub fn test_bed(&self) -> bool {
        self.bed() != 0
    }

    #[doc="Sets the BED field."]
    #[inline] pub fn set_bed<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Bus Error on Source"]
    #[inline] pub fn bes(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if BES != 0"]
    #[inline] pub fn test_bes(&self) -> bool {
        self.bes() != 0
    }

    #[doc="Sets the BES field."]
    #[inline] pub fn set_bes<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Configuration Error"]
    #[inline] pub fn ce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if CE != 0"]
    #[inline] pub fn test_ce(&self) -> bool {
        self.ce() != 0
    }

    #[doc="Sets the CE field."]
    #[inline] pub fn set_ce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for DsrBcr {
    #[inline]
    fn from(other: u32) -> Self {
         DsrBcr(other)
    }
}

impl ::core::fmt::Display for DsrBcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for DsrBcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bcr() != 0 { try!(write!(f, " bcr=0x{:x}", self.bcr()))}
        if self.done() != 0 { try!(write!(f, " done"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        if self.req() != 0 { try!(write!(f, " req"))}
        if self.bed() != 0 { try!(write!(f, " bed"))}
        if self.bes() != 0 { try!(write!(f, " bes"))}
        if self.ce() != 0 { try!(write!(f, " ce"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA_DSR0 register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dsr(pub u8);
impl Dsr {
}

impl From<u8> for Dsr {
    #[inline]
    fn from(other: u8) -> Self {
         Dsr(other)
    }
}

impl ::core::fmt::Display for Dsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DMA Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc="Link Channel 2"]
    #[inline] pub fn lch2(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if LCH2 != 0"]
    #[inline] pub fn test_lch2(&self) -> bool {
        self.lch2() != 0
    }

    #[doc="Sets the LCH2 field."]
    #[inline] pub fn set_lch2<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Link Channel 1"]
    #[inline] pub fn lch1(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if LCH1 != 0"]
    #[inline] pub fn test_lch1(&self) -> bool {
        self.lch1() != 0
    }

    #[doc="Sets the LCH1 field."]
    #[inline] pub fn set_lch1<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Link Channel Control"]
    #[inline] pub fn linkcc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if LINKCC != 0"]
    #[inline] pub fn test_linkcc(&self) -> bool {
        self.linkcc() != 0
    }

    #[doc="Sets the LINKCC field."]
    #[inline] pub fn set_linkcc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Disable Request"]
    #[inline] pub fn d_req(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if D_REQ != 0"]
    #[inline] pub fn test_d_req(&self) -> bool {
        self.d_req() != 0
    }

    #[doc="Sets the D_REQ field."]
    #[inline] pub fn set_d_req<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Destination Address Modulo"]
    #[inline] pub fn dmod(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if DMOD != 0"]
    #[inline] pub fn test_dmod(&self) -> bool {
        self.dmod() != 0
    }

    #[doc="Sets the DMOD field."]
    #[inline] pub fn set_dmod<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Source Address Modulo"]
    #[inline] pub fn smod(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if SMOD != 0"]
    #[inline] pub fn test_smod(&self) -> bool {
        self.smod() != 0
    }

    #[doc="Sets the SMOD field."]
    #[inline] pub fn set_smod<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Start Transfer"]
    #[inline] pub fn start(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if START != 0"]
    #[inline] pub fn test_start(&self) -> bool {
        self.start() != 0
    }

    #[doc="Sets the START field."]
    #[inline] pub fn set_start<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Destination Size"]
    #[inline] pub fn dsize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Returns true if DSIZE != 0"]
    #[inline] pub fn test_dsize(&self) -> bool {
        self.dsize() != 0
    }

    #[doc="Sets the DSIZE field."]
    #[inline] pub fn set_dsize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Destination Increment"]
    #[inline] pub fn dinc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if DINC != 0"]
    #[inline] pub fn test_dinc(&self) -> bool {
        self.dinc() != 0
    }

    #[doc="Sets the DINC field."]
    #[inline] pub fn set_dinc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Source Size"]
    #[inline] pub fn ssize(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if SSIZE != 0"]
    #[inline] pub fn test_ssize(&self) -> bool {
        self.ssize() != 0
    }

    #[doc="Sets the SSIZE field."]
    #[inline] pub fn set_ssize<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Source Increment"]
    #[inline] pub fn sinc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if SINC != 0"]
    #[inline] pub fn test_sinc(&self) -> bool {
        self.sinc() != 0
    }

    #[doc="Sets the SINC field."]
    #[inline] pub fn set_sinc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Enable asynchronous DMA requests"]
    #[inline] pub fn eadreq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if EADREQ != 0"]
    #[inline] pub fn test_eadreq(&self) -> bool {
        self.eadreq() != 0
    }

    #[doc="Sets the EADREQ field."]
    #[inline] pub fn set_eadreq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Auto-align"]
    #[inline] pub fn aa(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if AA != 0"]
    #[inline] pub fn test_aa(&self) -> bool {
        self.aa() != 0
    }

    #[doc="Sets the AA field."]
    #[inline] pub fn set_aa<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Cycle Steal"]
    #[inline] pub fn cs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if CS != 0"]
    #[inline] pub fn test_cs(&self) -> bool {
        self.cs() != 0
    }

    #[doc="Sets the CS field."]
    #[inline] pub fn set_cs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Enable Peripheral Request"]
    #[inline] pub fn erq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if ERQ != 0"]
    #[inline] pub fn test_erq(&self) -> bool {
        self.erq() != 0
    }

    #[doc="Sets the ERQ field."]
    #[inline] pub fn set_erq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Enable Interrupt on Completion of Transfer"]
    #[inline] pub fn eint(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if EINT != 0"]
    #[inline] pub fn test_eint(&self) -> bool {
        self.eint() != 0
    }

    #[doc="Sets the EINT field."]
    #[inline] pub fn set_eint<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Dcr {
    #[inline]
    fn from(other: u32) -> Self {
         Dcr(other)
    }
}

impl ::core::fmt::Display for Dcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lch2() != 0 { try!(write!(f, " lch2=0x{:x}", self.lch2()))}
        if self.lch1() != 0 { try!(write!(f, " lch1=0x{:x}", self.lch1()))}
        if self.linkcc() != 0 { try!(write!(f, " linkcc=0x{:x}", self.linkcc()))}
        if self.d_req() != 0 { try!(write!(f, " d_req"))}
        if self.dmod() != 0 { try!(write!(f, " dmod=0x{:x}", self.dmod()))}
        if self.smod() != 0 { try!(write!(f, " smod=0x{:x}", self.smod()))}
        if self.start() != 0 { try!(write!(f, " start"))}
        if self.dsize() != 0 { try!(write!(f, " dsize=0x{:x}", self.dsize()))}
        if self.dinc() != 0 { try!(write!(f, " dinc"))}
        if self.ssize() != 0 { try!(write!(f, " ssize=0x{:x}", self.ssize()))}
        if self.sinc() != 0 { try!(write!(f, " sinc"))}
        if self.eadreq() != 0 { try!(write!(f, " eadreq"))}
        if self.aa() != 0 { try!(write!(f, " aa"))}
        if self.cs() != 0 { try!(write!(f, " cs"))}
        if self.erq() != 0 { try!(write!(f, " erq"))}
        if self.eint() != 0 { try!(write!(f, " eint"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

