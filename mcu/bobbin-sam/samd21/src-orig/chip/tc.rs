//! Timer/Counter

periph!( TC3, Tc3, TC3_PERIPH, TcPeriph, 0x42002c00, 0x0e);
periph!( TC4, Tc4, TC4_PERIPH, TcPeriph, 0x42003000, 0x0f);
periph!( TC5, Tc5, TC5_PERIPH, TcPeriph, 0x42003400, 0x10);

pub struct TcCh { pub periph: TcPeriph, pub index: usize }
channel!(TC3_CH0, Tc3Ch0, TC3, Tc3, TC3_CH0_CH, TcCh, TC3_PERIPH, 0);
channel!(TC3_CH1, Tc3Ch1, TC3, Tc3, TC3_CH1_CH, TcCh, TC3_PERIPH, 1);
channel!(TC4_CH0, Tc4Ch0, TC4, Tc4, TC4_CH0_CH, TcCh, TC4_PERIPH, 0);
channel!(TC4_CH1, Tc4Ch1, TC4, Tc4, TC4_CH1_CH, TcCh, TC4_PERIPH, 1);
channel!(TC5_CH0, Tc5Ch0, TC5, Tc5, TC5_CH0_CH, TcCh, TC5_PERIPH, 0);
channel!(TC5_CH1, Tc5Ch1, TC5, Tc5, TC5_CH1_CH, TcCh, TC5_PERIPH, 1);
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TC Peripheral"]
pub struct TcPeriph(pub usize); 

impl TcPeriph {
    #[doc="Get 8-bit Counter Mode Peripheral"]
    #[inline] pub fn count8(&self) -> count8::Count8 {
        count8::Count8(self.0 + 0x0)
    }
    #[doc="Get 16-bit Counter Mode Peripheral"]
    #[inline] pub fn count16(&self) -> count16::Count16 {
        count16::Count16(self.0 + 0x0)
    }
    #[doc="Get 32-bit Counter Mode Peripheral"]
    #[inline] pub fn count32(&self) -> count32::Count32 {
        count32::Count32(self.0 + 0x0)
    }
}

#[doc="8-bit Counter Mode Cluster"]
pub mod count8 {
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="8-bit Counter Mode Peripheral"]
    pub struct Count8(pub usize);
impl Count8 {
    #[doc="Get the *mut pointer for the CC register."]
    #[inline] pub fn cc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Cc { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x18 + (index)) as *mut Cc
    }

    #[doc="Get the *const pointer for the CC register."]
    #[inline] pub fn cc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Cc { 
           self.cc_mut(index)
    }

    #[doc="Read the CC register."]
    #[inline] pub fn cc<I: Into<bits::R2>>(&self, index: I) -> Cc { 
        unsafe {
            read_volatile(self.cc_ptr(index))
        }
    }

    #[doc="Write the CC register."]
    #[inline] pub fn set_cc<I: Into<bits::R2>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(index), f(Cc(0)));
        }
        self
    }

    #[doc="Modify the CC register."]
    #[inline] pub fn with_cc<I: Into<bits::R2> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(index), f(self.cc(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the COUNT register."]
    #[inline] pub fn count_mut(&self) -> *mut Count { 
        (self.0 + 0x10) as *mut Count
    }

    #[doc="Get the *const pointer for the COUNT register."]
    #[inline] pub fn count_ptr(&self) -> *const Count { 
           self.count_mut()
    }

    #[doc="Read the COUNT register."]
    #[inline] pub fn count(&self) -> Count { 
        unsafe {
            read_volatile(self.count_ptr())
        }
    }

    #[doc="Write the COUNT register."]
    #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(Count(0)));
        }
        self
    }

    #[doc="Modify the COUNT register."]
    #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(self.count()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PER register."]
    #[inline] pub fn per_mut(&self) -> *mut Per { 
        (self.0 + 0x14) as *mut Per
    }

    #[doc="Get the *const pointer for the PER register."]
    #[inline] pub fn per_ptr(&self) -> *const Per { 
           self.per_mut()
    }

    #[doc="Read the PER register."]
    #[inline] pub fn per(&self) -> Per { 
        unsafe {
            read_volatile(self.per_ptr())
        }
    }

    #[doc="Write the PER register."]
    #[inline] pub fn set_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.per_mut(), f(Per(0)));
        }
        self
    }

    #[doc="Modify the PER register."]
    #[inline] pub fn with_per<F: FnOnce(Per) -> Per>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.per_mut(), f(self.per()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut Ctrla { 
        (self.0 + 0x0) as *mut Ctrla
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const Ctrla { 
           self.ctrla_mut()
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        unsafe {
            read_volatile(self.ctrla_ptr())
        }
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrla_mut(), f(Ctrla(0)));
        }
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrla_mut(), f(self.ctrla()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_mut(&self) -> *mut Ctrlbclr { 
        (self.0 + 0x4) as *mut Ctrlbclr
    }

    #[doc="Get the *const pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_ptr(&self) -> *const Ctrlbclr { 
           self.ctrlbclr_mut()
    }

    #[doc="Read the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
        unsafe {
            read_volatile(self.ctrlbclr_ptr())
        }
    }

    #[doc="Write the CTRLBCLR register."]
    #[inline] pub fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbclr_mut(), f(Ctrlbclr(0)));
        }
        self
    }

    #[doc="Modify the CTRLBCLR register."]
    #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbclr_mut(), f(self.ctrlbclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_mut(&self) -> *mut Ctrlbset { 
        (self.0 + 0x5) as *mut Ctrlbset
    }

    #[doc="Get the *const pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_ptr(&self) -> *const Ctrlbset { 
           self.ctrlbset_mut()
    }

    #[doc="Read the CTRLBSET register."]
    #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
        unsafe {
            read_volatile(self.ctrlbset_ptr())
        }
    }

    #[doc="Write the CTRLBSET register."]
    #[inline] pub fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbset_mut(), f(Ctrlbset(0)));
        }
        self
    }

    #[doc="Modify the CTRLBSET register."]
    #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbset_mut(), f(self.ctrlbset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_mut(&self) -> *mut Ctrlc { 
        (self.0 + 0x6) as *mut Ctrlc
    }

    #[doc="Get the *const pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_ptr(&self) -> *const Ctrlc { 
           self.ctrlc_mut()
    }

    #[doc="Read the CTRLC register."]
    #[inline] pub fn ctrlc(&self) -> Ctrlc { 
        unsafe {
            read_volatile(self.ctrlc_ptr())
        }
    }

    #[doc="Write the CTRLC register."]
    #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlc_mut(), f(Ctrlc(0)));
        }
        self
    }

    #[doc="Modify the CTRLC register."]
    #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlc_mut(), f(self.ctrlc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_mut(&self) -> *mut Dbgctrl { 
        (self.0 + 0x8) as *mut Dbgctrl
    }

    #[doc="Get the *const pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_ptr(&self) -> *const Dbgctrl { 
           self.dbgctrl_mut()
    }

    #[doc="Read the DBGCTRL register."]
    #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
        unsafe {
            read_volatile(self.dbgctrl_ptr())
        }
    }

    #[doc="Write the DBGCTRL register."]
    #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgctrl_mut(), f(Dbgctrl(0)));
        }
        self
    }

    #[doc="Modify the DBGCTRL register."]
    #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgctrl_mut(), f(self.dbgctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        (self.0 + 0xa) as *mut Evctrl
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
           self.evctrl_mut()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        unsafe {
            read_volatile(self.evctrl_ptr())
        }
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evctrl_mut(), f(Evctrl(0)));
        }
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evctrl_mut(), f(self.evctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        (self.0 + 0xc) as *mut Intenclr
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
           self.intenclr_mut()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        unsafe {
            read_volatile(self.intenclr_ptr())
        }
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(Intenclr(0)));
        }
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(self.intenclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        (self.0 + 0xd) as *mut Intenset
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
           self.intenset_mut()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        unsafe {
            read_volatile(self.intenset_ptr())
        }
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(Intenset(0)));
        }
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(self.intenset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        (self.0 + 0xe) as *mut Intflag
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
           self.intflag_mut()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        unsafe {
            read_volatile(self.intflag_ptr())
        }
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(Intflag(0)));
        }
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(self.intflag()));
        }
        self
    }

    #[doc="Get the *mut pointer for the READREQ register."]
    #[inline] pub fn readreq_mut(&self) -> *mut Readreq { 
        (self.0 + 0x2) as *mut Readreq
    }

    #[doc="Get the *const pointer for the READREQ register."]
    #[inline] pub fn readreq_ptr(&self) -> *const Readreq { 
           self.readreq_mut()
    }

    #[doc="Read the READREQ register."]
    #[inline] pub fn readreq(&self) -> Readreq { 
        unsafe {
            read_volatile(self.readreq_ptr())
        }
    }

    #[doc="Write the READREQ register."]
    #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.readreq_mut(), f(Readreq(0)));
        }
        self
    }

    #[doc="Modify the READREQ register."]
    #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.readreq_mut(), f(self.readreq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0xf) as *mut Status
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
           self.status_mut()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            read_volatile(self.status_ptr())
        }
    }

}

#[doc="COUNT8 Compare/Capture"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u8);
impl Cc {
    #[doc="Compare/Capture Value"]
    #[inline] pub fn cc(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc(&self) -> bool {
        self.cc() != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Cc {
    #[inline]
    fn from(other: u8) -> Self {
         Cc(other)
    }
}

impl ::core::fmt::Display for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="COUNT8 Counter Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u8);
impl Count {
    #[doc="Counter Value"]
    #[inline] pub fn count(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Count {
    #[inline]
    fn from(other: u8) -> Self {
         Count(other)
    }
}

impl ::core::fmt::Display for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="COUNT8 Period Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Per(pub u8);
impl Per {
    #[doc="Period Value"]
    #[inline] pub fn per(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if PER != 0"]
    #[inline] pub fn test_per(&self) -> bool {
        self.per() != 0
    }

    #[doc="Sets the PER field."]
    #[inline] pub fn set_per<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Per {
    #[inline]
    fn from(other: u8) -> Self {
         Per(other)
    }
}

impl ::core::fmt::Display for Per {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Per {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TC Mode"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Waveform Generation Operation"]
    #[inline] pub fn wavegen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if WAVEGEN != 0"]
    #[inline] pub fn test_wavegen(&self) -> bool {
        self.wavegen() != 0
    }

    #[doc="Sets the WAVEGEN field."]
    #[inline] pub fn set_wavegen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Prescaler"]
    #[inline] pub fn prescaler(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Prescaler and Counter Synchronization"]
    #[inline] pub fn prescsync(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if PRESCSYNC != 0"]
    #[inline] pub fn test_prescsync(&self) -> bool {
        self.prescsync() != 0
    }

    #[doc="Sets the PRESCSYNC field."]
    #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Ctrla {
    #[inline]
    fn from(other: u16) -> Self {
         Ctrla(other)
    }
}

impl ::core::fmt::Display for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command"]
    #[inline] pub fn cmd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Ctrlbclr {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbclr(other)
    }
}

impl ::core::fmt::Display for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command"]
    #[inline] pub fn cmd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Ctrlbset {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbset(other)
    }
}

impl ::core::fmt::Display for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
    #[doc="Output Waveform 0 Invert Enable"]
    #[inline] pub fn inven0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INVEN0 != 0"]
    #[inline] pub fn test_inven0(&self) -> bool {
        self.inven0() != 0
    }

    #[doc="Sets the INVEN0 field."]
    #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Output Waveform 1 Invert Enable"]
    #[inline] pub fn inven1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INVEN1 != 0"]
    #[inline] pub fn test_inven1(&self) -> bool {
        self.inven1() != 0
    }

    #[doc="Sets the INVEN1 field."]
    #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capture Channel 0 Enable"]
    #[inline] pub fn cpten0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CPTEN0 != 0"]
    #[inline] pub fn test_cpten0(&self) -> bool {
        self.cpten0() != 0
    }

    #[doc="Sets the CPTEN0 field."]
    #[inline] pub fn set_cpten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Capture Channel 1 Enable"]
    #[inline] pub fn cpten1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CPTEN1 != 0"]
    #[inline] pub fn test_cpten1(&self) -> bool {
        self.cpten1() != 0
    }

    #[doc="Sets the CPTEN1 field."]
    #[inline] pub fn set_cpten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Ctrlc {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlc(other)
    }
}

impl ::core::fmt::Display for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inven0() != 0 { try!(write!(f, " inven0"))}
        if self.inven1() != 0 { try!(write!(f, " inven1"))}
        if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
        if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Run Mode"]
    #[inline] pub fn dbgrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGRUN != 0"]
    #[inline] pub fn test_dbgrun(&self) -> bool {
        self.dbgrun() != 0
    }

    #[doc="Sets the DBGRUN field."]
    #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Dbgctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Dbgctrl(other)
    }
}

impl ::core::fmt::Display for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
    #[doc="Event Action"]
    #[inline] pub fn evact(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EVACT != 0"]
    #[inline] pub fn test_evact(&self) -> bool {
        self.evact() != 0
    }

    #[doc="Sets the EVACT field."]
    #[inline] pub fn set_evact<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TC Inverted Event Input"]
    #[inline] pub fn tcinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCINV != 0"]
    #[inline] pub fn test_tcinv(&self) -> bool {
        self.tcinv() != 0
    }

    #[doc="Sets the TCINV field."]
    #[inline] pub fn set_tcinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TC Event Input"]
    #[inline] pub fn tcei(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TCEI != 0"]
    #[inline] pub fn test_tcei(&self) -> bool {
        self.tcei() != 0
    }

    #[doc="Sets the TCEI field."]
    #[inline] pub fn set_tcei<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Overflow/Underflow Event Output Enable"]
    #[inline] pub fn ovfeo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVFEO != 0"]
    #[inline] pub fn test_ovfeo(&self) -> bool {
        self.ovfeo() != 0
    }

    #[doc="Sets the OVFEO field."]
    #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Match or Capture Channel 0 Event Output Enable"]
    #[inline] pub fn mceo0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MCEO0 != 0"]
    #[inline] pub fn test_mceo0(&self) -> bool {
        self.mceo0() != 0
    }

    #[doc="Sets the MCEO0 field."]
    #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Match or Capture Channel 1 Event Output Enable"]
    #[inline] pub fn mceo1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MCEO1 != 0"]
    #[inline] pub fn test_mceo1(&self) -> bool {
        self.mceo1() != 0
    }

    #[doc="Sets the MCEO1 field."]
    #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Evctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
        if self.tcinv() != 0 { try!(write!(f, " tcinv"))}
        if self.tcei() != 0 { try!(write!(f, " tcei"))}
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
        if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Match or Capture Channel 0 Interrupt Enable"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Match or Capture Channel 1 Interrupt Enable"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Intenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Intenclr(other)
    }
}

impl ::core::fmt::Display for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Match or Capture Channel 0 Interrupt Enable"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Match or Capture Channel 1 Interrupt Enable"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Intenset {
    #[inline]
    fn from(other: u8) -> Self {
         Intenset(other)
    }
}

impl ::core::fmt::Display for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Match or Capture Channel 0"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Match or Capture Channel 1"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Intflag {
    #[inline]
    fn from(other: u8) -> Self {
         Intflag(other)
    }
}

impl ::core::fmt::Display for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Read Request"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
    #[doc="Address"]
    #[inline] pub fn addr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Read Continuously"]
    #[inline] pub fn rcont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Read Request"]
    #[inline] pub fn rreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RREQ != 0"]
    #[inline] pub fn test_rreq(&self) -> bool {
        self.rreq() != 0
    }

    #[doc="Sets the RREQ field."]
    #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Readreq {
    #[inline]
    fn from(other: u16) -> Self {
         Readreq(other)
    }
}

impl ::core::fmt::Display for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rreq() != 0 { try!(write!(f, " rreq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Stop"]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Slave"]
    #[inline] pub fn slave(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SLAVE != 0"]
    #[inline] pub fn test_slave(&self) -> bool {
        self.slave() != 0
    }

    #[doc="Sets the SLAVE field."]
    #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization Busy"]
    #[inline] pub fn syncbusy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNCBUSY != 0"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy() != 0
    }

    #[doc="Sets the SYNCBUSY field."]
    #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.slave() != 0 { try!(write!(f, " slave"))}
        if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of count8

#[doc="16-bit Counter Mode Cluster"]
pub mod count16 {
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="16-bit Counter Mode Peripheral"]
    pub struct Count16(pub usize);
impl Count16 {
    #[doc="Get the *mut pointer for the CC register."]
    #[inline] pub fn cc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Cc { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x18 + (index << 1)) as *mut Cc
    }

    #[doc="Get the *const pointer for the CC register."]
    #[inline] pub fn cc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Cc { 
           self.cc_mut(index)
    }

    #[doc="Read the CC register."]
    #[inline] pub fn cc<I: Into<bits::R2>>(&self, index: I) -> Cc { 
        unsafe {
            read_volatile(self.cc_ptr(index))
        }
    }

    #[doc="Write the CC register."]
    #[inline] pub fn set_cc<I: Into<bits::R2>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(index), f(Cc(0)));
        }
        self
    }

    #[doc="Modify the CC register."]
    #[inline] pub fn with_cc<I: Into<bits::R2> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(index), f(self.cc(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the COUNT register."]
    #[inline] pub fn count_mut(&self) -> *mut Count { 
        (self.0 + 0x10) as *mut Count
    }

    #[doc="Get the *const pointer for the COUNT register."]
    #[inline] pub fn count_ptr(&self) -> *const Count { 
           self.count_mut()
    }

    #[doc="Read the COUNT register."]
    #[inline] pub fn count(&self) -> Count { 
        unsafe {
            read_volatile(self.count_ptr())
        }
    }

    #[doc="Write the COUNT register."]
    #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(Count(0)));
        }
        self
    }

    #[doc="Modify the COUNT register."]
    #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(self.count()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut Ctrla { 
        (self.0 + 0x0) as *mut Ctrla
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const Ctrla { 
           self.ctrla_mut()
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        unsafe {
            read_volatile(self.ctrla_ptr())
        }
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrla_mut(), f(Ctrla(0)));
        }
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrla_mut(), f(self.ctrla()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_mut(&self) -> *mut Ctrlbclr { 
        (self.0 + 0x4) as *mut Ctrlbclr
    }

    #[doc="Get the *const pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_ptr(&self) -> *const Ctrlbclr { 
           self.ctrlbclr_mut()
    }

    #[doc="Read the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
        unsafe {
            read_volatile(self.ctrlbclr_ptr())
        }
    }

    #[doc="Write the CTRLBCLR register."]
    #[inline] pub fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbclr_mut(), f(Ctrlbclr(0)));
        }
        self
    }

    #[doc="Modify the CTRLBCLR register."]
    #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbclr_mut(), f(self.ctrlbclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_mut(&self) -> *mut Ctrlbset { 
        (self.0 + 0x5) as *mut Ctrlbset
    }

    #[doc="Get the *const pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_ptr(&self) -> *const Ctrlbset { 
           self.ctrlbset_mut()
    }

    #[doc="Read the CTRLBSET register."]
    #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
        unsafe {
            read_volatile(self.ctrlbset_ptr())
        }
    }

    #[doc="Write the CTRLBSET register."]
    #[inline] pub fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbset_mut(), f(Ctrlbset(0)));
        }
        self
    }

    #[doc="Modify the CTRLBSET register."]
    #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbset_mut(), f(self.ctrlbset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_mut(&self) -> *mut Ctrlc { 
        (self.0 + 0x6) as *mut Ctrlc
    }

    #[doc="Get the *const pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_ptr(&self) -> *const Ctrlc { 
           self.ctrlc_mut()
    }

    #[doc="Read the CTRLC register."]
    #[inline] pub fn ctrlc(&self) -> Ctrlc { 
        unsafe {
            read_volatile(self.ctrlc_ptr())
        }
    }

    #[doc="Write the CTRLC register."]
    #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlc_mut(), f(Ctrlc(0)));
        }
        self
    }

    #[doc="Modify the CTRLC register."]
    #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlc_mut(), f(self.ctrlc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_mut(&self) -> *mut Dbgctrl { 
        (self.0 + 0x8) as *mut Dbgctrl
    }

    #[doc="Get the *const pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_ptr(&self) -> *const Dbgctrl { 
           self.dbgctrl_mut()
    }

    #[doc="Read the DBGCTRL register."]
    #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
        unsafe {
            read_volatile(self.dbgctrl_ptr())
        }
    }

    #[doc="Write the DBGCTRL register."]
    #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgctrl_mut(), f(Dbgctrl(0)));
        }
        self
    }

    #[doc="Modify the DBGCTRL register."]
    #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgctrl_mut(), f(self.dbgctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        (self.0 + 0xa) as *mut Evctrl
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
           self.evctrl_mut()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        unsafe {
            read_volatile(self.evctrl_ptr())
        }
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evctrl_mut(), f(Evctrl(0)));
        }
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evctrl_mut(), f(self.evctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        (self.0 + 0xc) as *mut Intenclr
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
           self.intenclr_mut()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        unsafe {
            read_volatile(self.intenclr_ptr())
        }
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(Intenclr(0)));
        }
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(self.intenclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        (self.0 + 0xd) as *mut Intenset
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
           self.intenset_mut()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        unsafe {
            read_volatile(self.intenset_ptr())
        }
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(Intenset(0)));
        }
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(self.intenset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        (self.0 + 0xe) as *mut Intflag
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
           self.intflag_mut()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        unsafe {
            read_volatile(self.intflag_ptr())
        }
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(Intflag(0)));
        }
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(self.intflag()));
        }
        self
    }

    #[doc="Get the *mut pointer for the READREQ register."]
    #[inline] pub fn readreq_mut(&self) -> *mut Readreq { 
        (self.0 + 0x2) as *mut Readreq
    }

    #[doc="Get the *const pointer for the READREQ register."]
    #[inline] pub fn readreq_ptr(&self) -> *const Readreq { 
           self.readreq_mut()
    }

    #[doc="Read the READREQ register."]
    #[inline] pub fn readreq(&self) -> Readreq { 
        unsafe {
            read_volatile(self.readreq_ptr())
        }
    }

    #[doc="Write the READREQ register."]
    #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.readreq_mut(), f(Readreq(0)));
        }
        self
    }

    #[doc="Modify the READREQ register."]
    #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.readreq_mut(), f(self.readreq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0xf) as *mut Status
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
           self.status_mut()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            read_volatile(self.status_ptr())
        }
    }

}

#[doc="COUNT16 Compare/Capture"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u16);
impl Cc {
    #[doc="Compare/Capture Value"]
    #[inline] pub fn cc(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc(&self) -> bool {
        self.cc() != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Cc {
    #[inline]
    fn from(other: u16) -> Self {
         Cc(other)
    }
}

impl ::core::fmt::Display for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cc() != 0 { try!(write!(f, " cc=0x{:x}", self.cc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="COUNT16 Counter Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u16);
impl Count {
    #[doc="Count Value"]
    #[inline] pub fn count(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Count {
    #[inline]
    fn from(other: u16) -> Self {
         Count(other)
    }
}

impl ::core::fmt::Display for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TC Mode"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Waveform Generation Operation"]
    #[inline] pub fn wavegen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if WAVEGEN != 0"]
    #[inline] pub fn test_wavegen(&self) -> bool {
        self.wavegen() != 0
    }

    #[doc="Sets the WAVEGEN field."]
    #[inline] pub fn set_wavegen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Prescaler"]
    #[inline] pub fn prescaler(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Prescaler and Counter Synchronization"]
    #[inline] pub fn prescsync(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if PRESCSYNC != 0"]
    #[inline] pub fn test_prescsync(&self) -> bool {
        self.prescsync() != 0
    }

    #[doc="Sets the PRESCSYNC field."]
    #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Ctrla {
    #[inline]
    fn from(other: u16) -> Self {
         Ctrla(other)
    }
}

impl ::core::fmt::Display for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command"]
    #[inline] pub fn cmd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Ctrlbclr {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbclr(other)
    }
}

impl ::core::fmt::Display for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command"]
    #[inline] pub fn cmd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Ctrlbset {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbset(other)
    }
}

impl ::core::fmt::Display for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
    #[doc="Output Waveform 0 Invert Enable"]
    #[inline] pub fn inven0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INVEN0 != 0"]
    #[inline] pub fn test_inven0(&self) -> bool {
        self.inven0() != 0
    }

    #[doc="Sets the INVEN0 field."]
    #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Output Waveform 1 Invert Enable"]
    #[inline] pub fn inven1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INVEN1 != 0"]
    #[inline] pub fn test_inven1(&self) -> bool {
        self.inven1() != 0
    }

    #[doc="Sets the INVEN1 field."]
    #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capture Channel 0 Enable"]
    #[inline] pub fn cpten0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CPTEN0 != 0"]
    #[inline] pub fn test_cpten0(&self) -> bool {
        self.cpten0() != 0
    }

    #[doc="Sets the CPTEN0 field."]
    #[inline] pub fn set_cpten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Capture Channel 1 Enable"]
    #[inline] pub fn cpten1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CPTEN1 != 0"]
    #[inline] pub fn test_cpten1(&self) -> bool {
        self.cpten1() != 0
    }

    #[doc="Sets the CPTEN1 field."]
    #[inline] pub fn set_cpten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Ctrlc {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlc(other)
    }
}

impl ::core::fmt::Display for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inven0() != 0 { try!(write!(f, " inven0"))}
        if self.inven1() != 0 { try!(write!(f, " inven1"))}
        if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
        if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Run Mode"]
    #[inline] pub fn dbgrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGRUN != 0"]
    #[inline] pub fn test_dbgrun(&self) -> bool {
        self.dbgrun() != 0
    }

    #[doc="Sets the DBGRUN field."]
    #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Dbgctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Dbgctrl(other)
    }
}

impl ::core::fmt::Display for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
    #[doc="Event Action"]
    #[inline] pub fn evact(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EVACT != 0"]
    #[inline] pub fn test_evact(&self) -> bool {
        self.evact() != 0
    }

    #[doc="Sets the EVACT field."]
    #[inline] pub fn set_evact<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TC Inverted Event Input"]
    #[inline] pub fn tcinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCINV != 0"]
    #[inline] pub fn test_tcinv(&self) -> bool {
        self.tcinv() != 0
    }

    #[doc="Sets the TCINV field."]
    #[inline] pub fn set_tcinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TC Event Input"]
    #[inline] pub fn tcei(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TCEI != 0"]
    #[inline] pub fn test_tcei(&self) -> bool {
        self.tcei() != 0
    }

    #[doc="Sets the TCEI field."]
    #[inline] pub fn set_tcei<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Overflow/Underflow Event Output Enable"]
    #[inline] pub fn ovfeo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVFEO != 0"]
    #[inline] pub fn test_ovfeo(&self) -> bool {
        self.ovfeo() != 0
    }

    #[doc="Sets the OVFEO field."]
    #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Match or Capture Channel 0 Event Output Enable"]
    #[inline] pub fn mceo0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MCEO0 != 0"]
    #[inline] pub fn test_mceo0(&self) -> bool {
        self.mceo0() != 0
    }

    #[doc="Sets the MCEO0 field."]
    #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Match or Capture Channel 1 Event Output Enable"]
    #[inline] pub fn mceo1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MCEO1 != 0"]
    #[inline] pub fn test_mceo1(&self) -> bool {
        self.mceo1() != 0
    }

    #[doc="Sets the MCEO1 field."]
    #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Evctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
        if self.tcinv() != 0 { try!(write!(f, " tcinv"))}
        if self.tcei() != 0 { try!(write!(f, " tcei"))}
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
        if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Match or Capture Channel 0 Interrupt Enable"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Match or Capture Channel 1 Interrupt Enable"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Intenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Intenclr(other)
    }
}

impl ::core::fmt::Display for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Match or Capture Channel 0 Interrupt Enable"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Match or Capture Channel 1 Interrupt Enable"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Intenset {
    #[inline]
    fn from(other: u8) -> Self {
         Intenset(other)
    }
}

impl ::core::fmt::Display for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Match or Capture Channel 0"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Match or Capture Channel 1"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Intflag {
    #[inline]
    fn from(other: u8) -> Self {
         Intflag(other)
    }
}

impl ::core::fmt::Display for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Read Request"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
    #[doc="Address"]
    #[inline] pub fn addr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Read Continuously"]
    #[inline] pub fn rcont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Read Request"]
    #[inline] pub fn rreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RREQ != 0"]
    #[inline] pub fn test_rreq(&self) -> bool {
        self.rreq() != 0
    }

    #[doc="Sets the RREQ field."]
    #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Readreq {
    #[inline]
    fn from(other: u16) -> Self {
         Readreq(other)
    }
}

impl ::core::fmt::Display for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rreq() != 0 { try!(write!(f, " rreq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Stop"]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Slave"]
    #[inline] pub fn slave(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SLAVE != 0"]
    #[inline] pub fn test_slave(&self) -> bool {
        self.slave() != 0
    }

    #[doc="Sets the SLAVE field."]
    #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization Busy"]
    #[inline] pub fn syncbusy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNCBUSY != 0"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy() != 0
    }

    #[doc="Sets the SYNCBUSY field."]
    #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.slave() != 0 { try!(write!(f, " slave"))}
        if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of count16

#[doc="32-bit Counter Mode Cluster"]
pub mod count32 {
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[doc="32-bit Counter Mode Peripheral"]
    pub struct Count32(pub usize);
impl Count32 {
    #[doc="Get the *mut pointer for the CC register."]
    #[inline] pub fn cc_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Cc { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x18 + (index << 2)) as *mut Cc
    }

    #[doc="Get the *const pointer for the CC register."]
    #[inline] pub fn cc_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Cc { 
           self.cc_mut(index)
    }

    #[doc="Read the CC register."]
    #[inline] pub fn cc<I: Into<bits::R2>>(&self, index: I) -> Cc { 
        unsafe {
            read_volatile(self.cc_ptr(index))
        }
    }

    #[doc="Write the CC register."]
    #[inline] pub fn set_cc<I: Into<bits::R2>, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(index), f(Cc(0)));
        }
        self
    }

    #[doc="Modify the CC register."]
    #[inline] pub fn with_cc<I: Into<bits::R2> + Copy, F: FnOnce(Cc) -> Cc>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(index), f(self.cc(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the COUNT register."]
    #[inline] pub fn count_mut(&self) -> *mut Count { 
        (self.0 + 0x10) as *mut Count
    }

    #[doc="Get the *const pointer for the COUNT register."]
    #[inline] pub fn count_ptr(&self) -> *const Count { 
           self.count_mut()
    }

    #[doc="Read the COUNT register."]
    #[inline] pub fn count(&self) -> Count { 
        unsafe {
            read_volatile(self.count_ptr())
        }
    }

    #[doc="Write the COUNT register."]
    #[inline] pub fn set_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(Count(0)));
        }
        self
    }

    #[doc="Modify the COUNT register."]
    #[inline] pub fn with_count<F: FnOnce(Count) -> Count>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.count_mut(), f(self.count()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLA register."]
    #[inline] pub fn ctrla_mut(&self) -> *mut Ctrla { 
        (self.0 + 0x0) as *mut Ctrla
    }

    #[doc="Get the *const pointer for the CTRLA register."]
    #[inline] pub fn ctrla_ptr(&self) -> *const Ctrla { 
           self.ctrla_mut()
    }

    #[doc="Read the CTRLA register."]
    #[inline] pub fn ctrla(&self) -> Ctrla { 
        unsafe {
            read_volatile(self.ctrla_ptr())
        }
    }

    #[doc="Write the CTRLA register."]
    #[inline] pub fn set_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrla_mut(), f(Ctrla(0)));
        }
        self
    }

    #[doc="Modify the CTRLA register."]
    #[inline] pub fn with_ctrla<F: FnOnce(Ctrla) -> Ctrla>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrla_mut(), f(self.ctrla()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_mut(&self) -> *mut Ctrlbclr { 
        (self.0 + 0x4) as *mut Ctrlbclr
    }

    #[doc="Get the *const pointer for the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr_ptr(&self) -> *const Ctrlbclr { 
           self.ctrlbclr_mut()
    }

    #[doc="Read the CTRLBCLR register."]
    #[inline] pub fn ctrlbclr(&self) -> Ctrlbclr { 
        unsafe {
            read_volatile(self.ctrlbclr_ptr())
        }
    }

    #[doc="Write the CTRLBCLR register."]
    #[inline] pub fn set_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbclr_mut(), f(Ctrlbclr(0)));
        }
        self
    }

    #[doc="Modify the CTRLBCLR register."]
    #[inline] pub fn with_ctrlbclr<F: FnOnce(Ctrlbclr) -> Ctrlbclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbclr_mut(), f(self.ctrlbclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_mut(&self) -> *mut Ctrlbset { 
        (self.0 + 0x5) as *mut Ctrlbset
    }

    #[doc="Get the *const pointer for the CTRLBSET register."]
    #[inline] pub fn ctrlbset_ptr(&self) -> *const Ctrlbset { 
           self.ctrlbset_mut()
    }

    #[doc="Read the CTRLBSET register."]
    #[inline] pub fn ctrlbset(&self) -> Ctrlbset { 
        unsafe {
            read_volatile(self.ctrlbset_ptr())
        }
    }

    #[doc="Write the CTRLBSET register."]
    #[inline] pub fn set_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbset_mut(), f(Ctrlbset(0)));
        }
        self
    }

    #[doc="Modify the CTRLBSET register."]
    #[inline] pub fn with_ctrlbset<F: FnOnce(Ctrlbset) -> Ctrlbset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlbset_mut(), f(self.ctrlbset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_mut(&self) -> *mut Ctrlc { 
        (self.0 + 0x6) as *mut Ctrlc
    }

    #[doc="Get the *const pointer for the CTRLC register."]
    #[inline] pub fn ctrlc_ptr(&self) -> *const Ctrlc { 
           self.ctrlc_mut()
    }

    #[doc="Read the CTRLC register."]
    #[inline] pub fn ctrlc(&self) -> Ctrlc { 
        unsafe {
            read_volatile(self.ctrlc_ptr())
        }
    }

    #[doc="Write the CTRLC register."]
    #[inline] pub fn set_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlc_mut(), f(Ctrlc(0)));
        }
        self
    }

    #[doc="Modify the CTRLC register."]
    #[inline] pub fn with_ctrlc<F: FnOnce(Ctrlc) -> Ctrlc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctrlc_mut(), f(self.ctrlc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_mut(&self) -> *mut Dbgctrl { 
        (self.0 + 0x8) as *mut Dbgctrl
    }

    #[doc="Get the *const pointer for the DBGCTRL register."]
    #[inline] pub fn dbgctrl_ptr(&self) -> *const Dbgctrl { 
           self.dbgctrl_mut()
    }

    #[doc="Read the DBGCTRL register."]
    #[inline] pub fn dbgctrl(&self) -> Dbgctrl { 
        unsafe {
            read_volatile(self.dbgctrl_ptr())
        }
    }

    #[doc="Write the DBGCTRL register."]
    #[inline] pub fn set_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgctrl_mut(), f(Dbgctrl(0)));
        }
        self
    }

    #[doc="Modify the DBGCTRL register."]
    #[inline] pub fn with_dbgctrl<F: FnOnce(Dbgctrl) -> Dbgctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dbgctrl_mut(), f(self.dbgctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_mut(&self) -> *mut Evctrl { 
        (self.0 + 0xa) as *mut Evctrl
    }

    #[doc="Get the *const pointer for the EVCTRL register."]
    #[inline] pub fn evctrl_ptr(&self) -> *const Evctrl { 
           self.evctrl_mut()
    }

    #[doc="Read the EVCTRL register."]
    #[inline] pub fn evctrl(&self) -> Evctrl { 
        unsafe {
            read_volatile(self.evctrl_ptr())
        }
    }

    #[doc="Write the EVCTRL register."]
    #[inline] pub fn set_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evctrl_mut(), f(Evctrl(0)));
        }
        self
    }

    #[doc="Modify the EVCTRL register."]
    #[inline] pub fn with_evctrl<F: FnOnce(Evctrl) -> Evctrl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.evctrl_mut(), f(self.evctrl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_mut(&self) -> *mut Intenclr { 
        (self.0 + 0xc) as *mut Intenclr
    }

    #[doc="Get the *const pointer for the INTENCLR register."]
    #[inline] pub fn intenclr_ptr(&self) -> *const Intenclr { 
           self.intenclr_mut()
    }

    #[doc="Read the INTENCLR register."]
    #[inline] pub fn intenclr(&self) -> Intenclr { 
        unsafe {
            read_volatile(self.intenclr_ptr())
        }
    }

    #[doc="Write the INTENCLR register."]
    #[inline] pub fn set_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(Intenclr(0)));
        }
        self
    }

    #[doc="Modify the INTENCLR register."]
    #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenclr_mut(), f(self.intenclr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTENSET register."]
    #[inline] pub fn intenset_mut(&self) -> *mut Intenset { 
        (self.0 + 0xd) as *mut Intenset
    }

    #[doc="Get the *const pointer for the INTENSET register."]
    #[inline] pub fn intenset_ptr(&self) -> *const Intenset { 
           self.intenset_mut()
    }

    #[doc="Read the INTENSET register."]
    #[inline] pub fn intenset(&self) -> Intenset { 
        unsafe {
            read_volatile(self.intenset_ptr())
        }
    }

    #[doc="Write the INTENSET register."]
    #[inline] pub fn set_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(Intenset(0)));
        }
        self
    }

    #[doc="Modify the INTENSET register."]
    #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intenset_mut(), f(self.intenset()));
        }
        self
    }

    #[doc="Get the *mut pointer for the INTFLAG register."]
    #[inline] pub fn intflag_mut(&self) -> *mut Intflag { 
        (self.0 + 0xe) as *mut Intflag
    }

    #[doc="Get the *const pointer for the INTFLAG register."]
    #[inline] pub fn intflag_ptr(&self) -> *const Intflag { 
           self.intflag_mut()
    }

    #[doc="Read the INTFLAG register."]
    #[inline] pub fn intflag(&self) -> Intflag { 
        unsafe {
            read_volatile(self.intflag_ptr())
        }
    }

    #[doc="Write the INTFLAG register."]
    #[inline] pub fn set_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(Intflag(0)));
        }
        self
    }

    #[doc="Modify the INTFLAG register."]
    #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.intflag_mut(), f(self.intflag()));
        }
        self
    }

    #[doc="Get the *mut pointer for the READREQ register."]
    #[inline] pub fn readreq_mut(&self) -> *mut Readreq { 
        (self.0 + 0x2) as *mut Readreq
    }

    #[doc="Get the *const pointer for the READREQ register."]
    #[inline] pub fn readreq_ptr(&self) -> *const Readreq { 
           self.readreq_mut()
    }

    #[doc="Read the READREQ register."]
    #[inline] pub fn readreq(&self) -> Readreq { 
        unsafe {
            read_volatile(self.readreq_ptr())
        }
    }

    #[doc="Write the READREQ register."]
    #[inline] pub fn set_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.readreq_mut(), f(Readreq(0)));
        }
        self
    }

    #[doc="Modify the READREQ register."]
    #[inline] pub fn with_readreq<F: FnOnce(Readreq) -> Readreq>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.readreq_mut(), f(self.readreq()));
        }
        self
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        (self.0 + 0xf) as *mut Status
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
           self.status_mut()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        unsafe {
            read_volatile(self.status_ptr())
        }
    }

}

#[doc="COUNT32 Compare/Capture"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc="Compare/Capture Value"]
    #[inline] pub fn cc(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if CC != 0"]
    #[inline] pub fn test_cc(&self) -> bool {
        self.cc() != 0
    }

    #[doc="Sets the CC field."]
    #[inline] pub fn set_cc<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cc {
    #[inline]
    fn from(other: u32) -> Self {
         Cc(other)
    }
}

impl ::core::fmt::Display for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="COUNT32 Counter Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Count(pub u32);
impl Count {
    #[doc="Count Value"]
    #[inline] pub fn count(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Count {
    #[inline]
    fn from(other: u32) -> Self {
         Count(other)
    }
}

impl ::core::fmt::Display for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Count {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control A"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrla(pub u16);
impl Ctrla {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ENABLE != 0"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable() != 0
    }

    #[doc="Sets the ENABLE field."]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TC Mode"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Waveform Generation Operation"]
    #[inline] pub fn wavegen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if WAVEGEN != 0"]
    #[inline] pub fn test_wavegen(&self) -> bool {
        self.wavegen() != 0
    }

    #[doc="Sets the WAVEGEN field."]
    #[inline] pub fn set_wavegen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Prescaler"]
    #[inline] pub fn prescaler(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if PRESCALER != 0"]
    #[inline] pub fn test_prescaler(&self) -> bool {
        self.prescaler() != 0
    }

    #[doc="Sets the PRESCALER field."]
    #[inline] pub fn set_prescaler<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Prescaler and Counter Synchronization"]
    #[inline] pub fn prescsync(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if PRESCSYNC != 0"]
    #[inline] pub fn test_prescsync(&self) -> bool {
        self.prescsync() != 0
    }

    #[doc="Sets the PRESCSYNC field."]
    #[inline] pub fn set_prescsync<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u16> for Ctrla {
    #[inline]
    fn from(other: u16) -> Self {
         Ctrla(other)
    }
}

impl ::core::fmt::Display for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrla {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.wavegen() != 0 { try!(write!(f, " wavegen=0x{:x}", self.wavegen()))}
        if self.prescaler() != 0 { try!(write!(f, " prescaler=0x{:x}", self.prescaler()))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        if self.prescsync() != 0 { try!(write!(f, " prescsync=0x{:x}", self.prescsync()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbclr(pub u8);
impl Ctrlbclr {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command"]
    #[inline] pub fn cmd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Ctrlbclr {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbclr(other)
    }
}

impl ::core::fmt::Display for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control B Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlbset(pub u8);
impl Ctrlbset {
    #[doc="Counter Direction"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="One-Shot"]
    #[inline] pub fn oneshot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ONESHOT != 0"]
    #[inline] pub fn test_oneshot(&self) -> bool {
        self.oneshot() != 0
    }

    #[doc="Sets the ONESHOT field."]
    #[inline] pub fn set_oneshot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Command"]
    #[inline] pub fn cmd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Ctrlbset {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlbset(other)
    }
}

impl ::core::fmt::Display for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlbset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.oneshot() != 0 { try!(write!(f, " oneshot"))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control C"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlc(pub u8);
impl Ctrlc {
    #[doc="Output Waveform 0 Invert Enable"]
    #[inline] pub fn inven0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INVEN0 != 0"]
    #[inline] pub fn test_inven0(&self) -> bool {
        self.inven0() != 0
    }

    #[doc="Sets the INVEN0 field."]
    #[inline] pub fn set_inven0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Output Waveform 1 Invert Enable"]
    #[inline] pub fn inven1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INVEN1 != 0"]
    #[inline] pub fn test_inven1(&self) -> bool {
        self.inven1() != 0
    }

    #[doc="Sets the INVEN1 field."]
    #[inline] pub fn set_inven1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Capture Channel 0 Enable"]
    #[inline] pub fn cpten0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CPTEN0 != 0"]
    #[inline] pub fn test_cpten0(&self) -> bool {
        self.cpten0() != 0
    }

    #[doc="Sets the CPTEN0 field."]
    #[inline] pub fn set_cpten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Capture Channel 1 Enable"]
    #[inline] pub fn cpten1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CPTEN1 != 0"]
    #[inline] pub fn test_cpten1(&self) -> bool {
        self.cpten1() != 0
    }

    #[doc="Sets the CPTEN1 field."]
    #[inline] pub fn set_cpten1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Ctrlc {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlc(other)
    }
}

impl ::core::fmt::Display for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inven0() != 0 { try!(write!(f, " inven0"))}
        if self.inven1() != 0 { try!(write!(f, " inven1"))}
        if self.cpten0() != 0 { try!(write!(f, " cpten0"))}
        if self.cpten1() != 0 { try!(write!(f, " cpten1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Debug Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dbgctrl(pub u8);
impl Dbgctrl {
    #[doc="Debug Run Mode"]
    #[inline] pub fn dbgrun(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DBGRUN != 0"]
    #[inline] pub fn test_dbgrun(&self) -> bool {
        self.dbgrun() != 0
    }

    #[doc="Sets the DBGRUN field."]
    #[inline] pub fn set_dbgrun<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Dbgctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Dbgctrl(other)
    }
}

impl ::core::fmt::Display for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dbgctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbgrun() != 0 { try!(write!(f, " dbgrun"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Event Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Evctrl(pub u16);
impl Evctrl {
    #[doc="Event Action"]
    #[inline] pub fn evact(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if EVACT != 0"]
    #[inline] pub fn test_evact(&self) -> bool {
        self.evact() != 0
    }

    #[doc="Sets the EVACT field."]
    #[inline] pub fn set_evact<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="TC Inverted Event Input"]
    #[inline] pub fn tcinv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TCINV != 0"]
    #[inline] pub fn test_tcinv(&self) -> bool {
        self.tcinv() != 0
    }

    #[doc="Sets the TCINV field."]
    #[inline] pub fn set_tcinv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TC Event Input"]
    #[inline] pub fn tcei(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TCEI != 0"]
    #[inline] pub fn test_tcei(&self) -> bool {
        self.tcei() != 0
    }

    #[doc="Sets the TCEI field."]
    #[inline] pub fn set_tcei<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Overflow/Underflow Event Output Enable"]
    #[inline] pub fn ovfeo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if OVFEO != 0"]
    #[inline] pub fn test_ovfeo(&self) -> bool {
        self.ovfeo() != 0
    }

    #[doc="Sets the OVFEO field."]
    #[inline] pub fn set_ovfeo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Match or Capture Channel 0 Event Output Enable"]
    #[inline] pub fn mceo0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if MCEO0 != 0"]
    #[inline] pub fn test_mceo0(&self) -> bool {
        self.mceo0() != 0
    }

    #[doc="Sets the MCEO0 field."]
    #[inline] pub fn set_mceo0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Match or Capture Channel 1 Event Output Enable"]
    #[inline] pub fn mceo1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if MCEO1 != 0"]
    #[inline] pub fn test_mceo1(&self) -> bool {
        self.mceo1() != 0
    }

    #[doc="Sets the MCEO1 field."]
    #[inline] pub fn set_mceo1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

}

impl From<u16> for Evctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Evctrl(other)
    }
}

impl ::core::fmt::Display for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Evctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.evact() != 0 { try!(write!(f, " evact=0x{:x}", self.evact()))}
        if self.tcinv() != 0 { try!(write!(f, " tcinv"))}
        if self.tcei() != 0 { try!(write!(f, " tcei"))}
        if self.ovfeo() != 0 { try!(write!(f, " ovfeo"))}
        if self.mceo0() != 0 { try!(write!(f, " mceo0"))}
        if self.mceo1() != 0 { try!(write!(f, " mceo1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Match or Capture Channel 0 Interrupt Enable"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Match or Capture Channel 1 Interrupt Enable"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Intenclr {
    #[inline]
    fn from(other: u8) -> Self {
         Intenclr(other)
    }
}

impl ::core::fmt::Display for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Set"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
    #[doc="Overflow Interrupt Enable"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error Interrupt Enable"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready Interrupt Enable"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Match or Capture Channel 0 Interrupt Enable"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Match or Capture Channel 1 Interrupt Enable"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Intenset {
    #[inline]
    fn from(other: u8) -> Self {
         Intenset(other)
    }
}

impl ::core::fmt::Display for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Flag Status and Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
    #[doc="Overflow"]
    #[inline] pub fn ovf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OVF != 0"]
    #[inline] pub fn test_ovf(&self) -> bool {
        self.ovf() != 0
    }

    #[doc="Sets the OVF field."]
    #[inline] pub fn set_ovf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Error"]
    #[inline] pub fn err(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if ERR != 0"]
    #[inline] pub fn test_err(&self) -> bool {
        self.err() != 0
    }

    #[doc="Sets the ERR field."]
    #[inline] pub fn set_err<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Synchronization Ready"]
    #[inline] pub fn syncrdy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCRDY != 0"]
    #[inline] pub fn test_syncrdy(&self) -> bool {
        self.syncrdy() != 0
    }

    #[doc="Sets the SYNCRDY field."]
    #[inline] pub fn set_syncrdy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Match or Capture Channel 0"]
    #[inline] pub fn mc0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MC0 != 0"]
    #[inline] pub fn test_mc0(&self) -> bool {
        self.mc0() != 0
    }

    #[doc="Sets the MC0 field."]
    #[inline] pub fn set_mc0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Match or Capture Channel 1"]
    #[inline] pub fn mc1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MC1 != 0"]
    #[inline] pub fn test_mc1(&self) -> bool {
        self.mc1() != 0
    }

    #[doc="Sets the MC1 field."]
    #[inline] pub fn set_mc1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u8> for Intflag {
    #[inline]
    fn from(other: u8) -> Self {
         Intflag(other)
    }
}

impl ::core::fmt::Display for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Intflag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ovf() != 0 { try!(write!(f, " ovf"))}
        if self.err() != 0 { try!(write!(f, " err"))}
        if self.syncrdy() != 0 { try!(write!(f, " syncrdy"))}
        if self.mc0() != 0 { try!(write!(f, " mc0"))}
        if self.mc1() != 0 { try!(write!(f, " mc1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Read Request"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Readreq(pub u16);
impl Readreq {
    #[doc="Address"]
    #[inline] pub fn addr(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Read Continuously"]
    #[inline] pub fn rcont(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RCONT != 0"]
    #[inline] pub fn test_rcont(&self) -> bool {
        self.rcont() != 0
    }

    #[doc="Sets the RCONT field."]
    #[inline] pub fn set_rcont<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Read Request"]
    #[inline] pub fn rreq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RREQ != 0"]
    #[inline] pub fn test_rreq(&self) -> bool {
        self.rreq() != 0
    }

    #[doc="Sets the RREQ field."]
    #[inline] pub fn set_rreq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Readreq {
    #[inline]
    fn from(other: u16) -> Self {
         Readreq(other)
    }
}

impl ::core::fmt::Display for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Readreq {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self.rcont() != 0 { try!(write!(f, " rcont"))}
        if self.rreq() != 0 { try!(write!(f, " rreq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Stop"]
    #[inline] pub fn stop(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Slave"]
    #[inline] pub fn slave(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SLAVE != 0"]
    #[inline] pub fn test_slave(&self) -> bool {
        self.slave() != 0
    }

    #[doc="Sets the SLAVE field."]
    #[inline] pub fn set_slave<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization Busy"]
    #[inline] pub fn syncbusy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNCBUSY != 0"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy() != 0
    }

    #[doc="Sets the SYNCBUSY field."]
    #[inline] pub fn set_syncbusy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
         Status(other)
    }
}

impl ::core::fmt::Display for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.slave() != 0 { try!(write!(f, " slave"))}
        if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

}
// End of count32
