//! FLASH
#[allow(unused_imports)] use bobbin_common::*;

periph!(FLASH, Flash, 0x40023c00);

#[doc="FLASH"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Flash(pub usize);
impl Flash {
    #[doc="Get the *mut pointer for the ACR register."]
    #[inline] pub fn acr_mut(&self) -> *mut Acr { 
        (self.0 + 0x0) as *mut Acr
    }

    #[doc="Get the *const pointer for the ACR register."]
    #[inline] pub fn acr_ptr(&self) -> *const Acr { 
           self.acr_mut()
    }

    #[doc="Read the ACR register."]
    #[inline] pub fn acr(&self) -> Acr { 
        unsafe {
            read_volatile(self.acr_ptr())
        }
    }

    #[doc="Write the ACR register."]
    #[inline] pub fn set_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acr_mut(), f(Acr(0)));
        }
        self
    }

    #[doc="Modify the ACR register."]
    #[inline] pub fn with_acr<F: FnOnce(Acr) -> Acr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.acr_mut(), f(self.acr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the KEYR register."]
    #[inline] pub fn keyr_mut(&self) -> *mut Keyr { 
        (self.0 + 0x4) as *mut Keyr
    }

    #[doc="Get the *const pointer for the KEYR register."]
    #[inline] pub fn keyr_ptr(&self) -> *const Keyr { 
           self.keyr_mut()
    }

    #[doc="Write the KEYR register."]
    #[inline] pub fn set_keyr<F: FnOnce(Keyr) -> Keyr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.keyr_mut(), f(Keyr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_mut(&self) -> *mut Optkeyr { 
        (self.0 + 0x8) as *mut Optkeyr
    }

    #[doc="Get the *const pointer for the OPTKEYR register."]
    #[inline] pub fn optkeyr_ptr(&self) -> *const Optkeyr { 
           self.optkeyr_mut()
    }

    #[doc="Write the OPTKEYR register."]
    #[inline] pub fn set_optkeyr<F: FnOnce(Optkeyr) -> Optkeyr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.optkeyr_mut(), f(Optkeyr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0xc) as *mut Sr
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
           self.sr_mut()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            read_volatile(self.sr_ptr())
        }
    }

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(Sr(0)));
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(self.sr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        (self.0 + 0x10) as *mut Cr
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
           self.cr_mut()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            read_volatile(self.cr_ptr())
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(Cr(0)));
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr_mut(), f(self.cr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the OPTCR register."]
    #[inline] pub fn optcr_mut(&self) -> *mut Optcr { 
        (self.0 + 0x14) as *mut Optcr
    }

    #[doc="Get the *const pointer for the OPTCR register."]
    #[inline] pub fn optcr_ptr(&self) -> *const Optcr { 
           self.optcr_mut()
    }

    #[doc="Read the OPTCR register."]
    #[inline] pub fn optcr(&self) -> Optcr { 
        unsafe {
            read_volatile(self.optcr_ptr())
        }
    }

    #[doc="Write the OPTCR register."]
    #[inline] pub fn set_optcr<F: FnOnce(Optcr) -> Optcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.optcr_mut(), f(Optcr(0)));
        }
        self
    }

    #[doc="Modify the OPTCR register."]
    #[inline] pub fn with_optcr<F: FnOnce(Optcr) -> Optcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.optcr_mut(), f(self.optcr()));
        }
        self
    }

}

#[doc="Flash access control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc="Latency"]
    #[inline] pub fn latency(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if LATENCY != 0"]
    #[inline] pub fn test_latency(&self) -> bool {
        self.latency() != 0
    }

    #[doc="Sets the LATENCY field."]
    #[inline] pub fn set_latency<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Prefetch enable"]
    #[inline] pub fn prften(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PRFTEN != 0"]
    #[inline] pub fn test_prften(&self) -> bool {
        self.prften() != 0
    }

    #[doc="Sets the PRFTEN field."]
    #[inline] pub fn set_prften<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Instruction cache enable"]
    #[inline] pub fn icen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ICEN != 0"]
    #[inline] pub fn test_icen(&self) -> bool {
        self.icen() != 0
    }

    #[doc="Sets the ICEN field."]
    #[inline] pub fn set_icen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data cache enable"]
    #[inline] pub fn dcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DCEN != 0"]
    #[inline] pub fn test_dcen(&self) -> bool {
        self.dcen() != 0
    }

    #[doc="Sets the DCEN field."]
    #[inline] pub fn set_dcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Instruction cache reset"]
    #[inline] pub fn icrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ICRST != 0"]
    #[inline] pub fn test_icrst(&self) -> bool {
        self.icrst() != 0
    }

    #[doc="Sets the ICRST field."]
    #[inline] pub fn set_icrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Data cache reset"]
    #[inline] pub fn dcrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DCRST != 0"]
    #[inline] pub fn test_dcrst(&self) -> bool {
        self.dcrst() != 0
    }

    #[doc="Sets the DCRST field."]
    #[inline] pub fn set_dcrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Acr {
    #[inline]
    fn from(other: u32) -> Self {
         Acr(other)
    }
}

impl ::core::fmt::Display for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Acr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.latency() != 0 { try!(write!(f, " latency=0x{:x}", self.latency()))}
        if self.prften() != 0 { try!(write!(f, " prften"))}
        if self.icen() != 0 { try!(write!(f, " icen"))}
        if self.dcen() != 0 { try!(write!(f, " dcen"))}
        if self.icrst() != 0 { try!(write!(f, " icrst"))}
        if self.dcrst() != 0 { try!(write!(f, " dcrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Keyr(pub u32);
impl Keyr {
    #[doc="FPEC key"]
    #[inline] pub fn key(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Keyr {
    #[inline]
    fn from(other: u32) -> Self {
         Keyr(other)
    }
}

impl ::core::fmt::Display for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Keyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash option key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optkeyr(pub u32);
impl Optkeyr {
    #[doc="Option byte key"]
    #[inline] pub fn optkey(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if OPTKEY != 0"]
    #[inline] pub fn test_optkey(&self) -> bool {
        self.optkey() != 0
    }

    #[doc="Sets the OPTKEY field."]
    #[inline] pub fn set_optkey<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Optkeyr {
    #[inline]
    fn from(other: u32) -> Self {
         Optkeyr(other)
    }
}

impl ::core::fmt::Display for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optkeyr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="End of operation"]
    #[inline] pub fn eop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EOP != 0"]
    #[inline] pub fn test_eop(&self) -> bool {
        self.eop() != 0
    }

    #[doc="Sets the EOP field."]
    #[inline] pub fn set_eop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Operation error"]
    #[inline] pub fn operr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OPERR != 0"]
    #[inline] pub fn test_operr(&self) -> bool {
        self.operr() != 0
    }

    #[doc="Sets the OPERR field."]
    #[inline] pub fn set_operr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Write protection error"]
    #[inline] pub fn wrperr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if WRPERR != 0"]
    #[inline] pub fn test_wrperr(&self) -> bool {
        self.wrperr() != 0
    }

    #[doc="Sets the WRPERR field."]
    #[inline] pub fn set_wrperr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Programming alignment error"]
    #[inline] pub fn pgaerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PGAERR != 0"]
    #[inline] pub fn test_pgaerr(&self) -> bool {
        self.pgaerr() != 0
    }

    #[doc="Sets the PGAERR field."]
    #[inline] pub fn set_pgaerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Programming parallelism error"]
    #[inline] pub fn pgperr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PGPERR != 0"]
    #[inline] pub fn test_pgperr(&self) -> bool {
        self.pgperr() != 0
    }

    #[doc="Sets the PGPERR field."]
    #[inline] pub fn set_pgperr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Programming sequence error"]
    #[inline] pub fn pgserr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PGSERR != 0"]
    #[inline] pub fn test_pgserr(&self) -> bool {
        self.pgserr() != 0
    }

    #[doc="Sets the PGSERR field."]
    #[inline] pub fn set_pgserr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Busy"]
    #[inline] pub fn bsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BSY != 0"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Sets the BSY field."]
    #[inline] pub fn set_bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.eop() != 0 { try!(write!(f, " eop"))}
        if self.operr() != 0 { try!(write!(f, " operr"))}
        if self.wrperr() != 0 { try!(write!(f, " wrperr"))}
        if self.pgaerr() != 0 { try!(write!(f, " pgaerr"))}
        if self.pgperr() != 0 { try!(write!(f, " pgperr"))}
        if self.pgserr() != 0 { try!(write!(f, " pgserr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Programming"]
    #[inline] pub fn pg(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PG != 0"]
    #[inline] pub fn test_pg(&self) -> bool {
        self.pg() != 0
    }

    #[doc="Sets the PG field."]
    #[inline] pub fn set_pg<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Sector Erase"]
    #[inline] pub fn ser(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SER != 0"]
    #[inline] pub fn test_ser(&self) -> bool {
        self.ser() != 0
    }

    #[doc="Sets the SER field."]
    #[inline] pub fn set_ser<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Mass Erase"]
    #[inline] pub fn mer(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MER != 0"]
    #[inline] pub fn test_mer(&self) -> bool {
        self.mer() != 0
    }

    #[doc="Sets the MER field."]
    #[inline] pub fn set_mer<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Sector number"]
    #[inline] pub fn snb(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0xf) as u8) } // [6:3]
    }

    #[doc="Returns true if SNB != 0"]
    #[inline] pub fn test_snb(&self) -> bool {
        self.snb() != 0
    }

    #[doc="Sets the SNB field."]
    #[inline] pub fn set_snb<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Program size"]
    #[inline] pub fn psize(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if PSIZE != 0"]
    #[inline] pub fn test_psize(&self) -> bool {
        self.psize() != 0
    }

    #[doc="Sets the PSIZE field."]
    #[inline] pub fn set_psize<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Start"]
    #[inline] pub fn strt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if STRT != 0"]
    #[inline] pub fn test_strt(&self) -> bool {
        self.strt() != 0
    }

    #[doc="Sets the STRT field."]
    #[inline] pub fn set_strt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="End of operation interrupt enable"]
    #[inline] pub fn eopie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if EOPIE != 0"]
    #[inline] pub fn test_eopie(&self) -> bool {
        self.eopie() != 0
    }

    #[doc="Sets the EOPIE field."]
    #[inline] pub fn set_eopie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Lock"]
    #[inline] pub fn lock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pg() != 0 { try!(write!(f, " pg"))}
        if self.ser() != 0 { try!(write!(f, " ser"))}
        if self.mer() != 0 { try!(write!(f, " mer"))}
        if self.snb() != 0 { try!(write!(f, " snb=0x{:x}", self.snb()))}
        if self.psize() != 0 { try!(write!(f, " psize=0x{:x}", self.psize()))}
        if self.strt() != 0 { try!(write!(f, " strt"))}
        if self.eopie() != 0 { try!(write!(f, " eopie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.lock() != 0 { try!(write!(f, " lock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash option control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Optcr(pub u32);
impl Optcr {
    #[doc="Option lock"]
    #[inline] pub fn optlock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OPTLOCK != 0"]
    #[inline] pub fn test_optlock(&self) -> bool {
        self.optlock() != 0
    }

    #[doc="Sets the OPTLOCK field."]
    #[inline] pub fn set_optlock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Option start"]
    #[inline] pub fn optstrt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OPTSTRT != 0"]
    #[inline] pub fn test_optstrt(&self) -> bool {
        self.optstrt() != 0
    }

    #[doc="Sets the OPTSTRT field."]
    #[inline] pub fn set_optstrt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="BOR reset Level"]
    #[inline] pub fn bor_lev(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if BOR_LEV != 0"]
    #[inline] pub fn test_bor_lev(&self) -> bool {
        self.bor_lev() != 0
    }

    #[doc="Sets the BOR_LEV field."]
    #[inline] pub fn set_bor_lev<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="WDG_SW User option bytes"]
    #[inline] pub fn wdg_sw(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WDG_SW != 0"]
    #[inline] pub fn test_wdg_sw(&self) -> bool {
        self.wdg_sw() != 0
    }

    #[doc="Sets the WDG_SW field."]
    #[inline] pub fn set_wdg_sw<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="nRST_STOP User option bytes"]
    #[inline] pub fn nrst_stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if nRST_STOP != 0"]
    #[inline] pub fn test_nrst_stop(&self) -> bool {
        self.nrst_stop() != 0
    }

    #[doc="Sets the nRST_STOP field."]
    #[inline] pub fn set_nrst_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="nRST_STDBY User option bytes"]
    #[inline] pub fn nrst_stdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if nRST_STDBY != 0"]
    #[inline] pub fn test_nrst_stdby(&self) -> bool {
        self.nrst_stdby() != 0
    }

    #[doc="Sets the nRST_STDBY field."]
    #[inline] pub fn set_nrst_stdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Read protect"]
    #[inline] pub fn rdp(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if RDP != 0"]
    #[inline] pub fn test_rdp(&self) -> bool {
        self.rdp() != 0
    }

    #[doc="Sets the RDP field."]
    #[inline] pub fn set_rdp<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Not write protect"]
    #[inline] pub fn nwrp(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xfff) as u16) } // [27:16]
    }

    #[doc="Returns true if nWRP != 0"]
    #[inline] pub fn test_nwrp(&self) -> bool {
        self.nwrp() != 0
    }

    #[doc="Sets the nWRP field."]
    #[inline] pub fn set_nwrp<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Optcr {
    #[inline]
    fn from(other: u32) -> Self {
         Optcr(other)
    }
}

impl ::core::fmt::Display for Optcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Optcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.optlock() != 0 { try!(write!(f, " optlock"))}
        if self.optstrt() != 0 { try!(write!(f, " optstrt"))}
        if self.bor_lev() != 0 { try!(write!(f, " bor_lev=0x{:x}", self.bor_lev()))}
        if self.wdg_sw() != 0 { try!(write!(f, " wdg_sw"))}
        if self.nrst_stop() != 0 { try!(write!(f, " nrst_stop"))}
        if self.nrst_stdby() != 0 { try!(write!(f, " nrst_stdby"))}
        if self.rdp() != 0 { try!(write!(f, " rdp=0x{:x}", self.rdp()))}
        if self.nwrp() != 0 { try!(write!(f, " nwrp=0x{:x}", self.nwrp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

