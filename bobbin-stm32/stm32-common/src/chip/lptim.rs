#[allow(unused_imports)] use bobbin_common::*;


#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPTIM Peripheral"]
pub struct LptimPeriph(pub usize); 


impl LptimPeriph {
    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x0) as *const u32
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x0) as *mut u32
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        unsafe {
            Isr(read_volatile((self.0 + 0x0) as *const u32))
        }
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4) as *const u32
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4) as *mut u32
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        let value = f(Icr(0));
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x8) as *const u32
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x8) as *mut u32
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        unsafe {
            Ier(read_volatile((self.0 + 0x8) as *const u32))
        }
    }

    #[doc="Write the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        let value = f(Ier(0));
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        let tmp = self.ier();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CFGR register."]
    #[inline] pub fn cfgr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0xc) as *const u32
    }

    #[doc="Get the *mut pointer for the CFGR register."]
    #[inline] pub fn cfgr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0xc) as *mut u32
    }

    #[doc="Read the CFGR register."]
    #[inline] pub fn cfgr(&self) -> Cfgr { 
        unsafe {
            Cfgr(read_volatile((self.0 + 0xc) as *const u32))
        }
    }

    #[doc="Write the CFGR register."]
    #[inline] pub fn set_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        let value = f(Cfgr(0));
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CFGR register."]
    #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        let tmp = self.cfgr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10) as *const u32
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10) as *mut u32
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        unsafe {
            Cr(read_volatile((self.0 + 0x10) as *const u32))
        }
    }

    #[doc="Write the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        let value = f(Cr(0));
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        let tmp = self.cr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CMP register."]
    #[inline] pub fn cmp_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x14) as *const u32
    }

    #[doc="Get the *mut pointer for the CMP register."]
    #[inline] pub fn cmp_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x14) as *mut u32
    }

    #[doc="Read the CMP register."]
    #[inline] pub fn cmp(&self) -> Cmp { 
        unsafe {
            Cmp(read_volatile((self.0 + 0x14) as *const u32))
        }
    }

    #[doc="Write the CMP register."]
    #[inline] pub fn set_cmp<F: FnOnce(Cmp) -> Cmp>(&self, f: F) -> &Self {
        let value = f(Cmp(0));
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CMP register."]
    #[inline] pub fn with_cmp<F: FnOnce(Cmp) -> Cmp>(&self, f: F) -> &Self {
        let tmp = self.cmp();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x14) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the ARR register."]
    #[inline] pub fn arr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x18) as *const u32
    }

    #[doc="Get the *mut pointer for the ARR register."]
    #[inline] pub fn arr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x18) as *mut u32
    }

    #[doc="Read the ARR register."]
    #[inline] pub fn arr(&self) -> Arr { 
        unsafe {
            Arr(read_volatile((self.0 + 0x18) as *const u32))
        }
    }

    #[doc="Write the ARR register."]
    #[inline] pub fn set_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
        let value = f(Arr(0));
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the ARR register."]
    #[inline] pub fn with_arr<F: FnOnce(Arr) -> Arr>(&self, f: F) -> &Self {
        let tmp = self.arr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x18) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x1c) as *const u32
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x1c) as *mut u32
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        unsafe {
            Cnt(read_volatile((self.0 + 0x1c) as *const u32))
        }
    }

}

#[doc="Interrupt and Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Counter direction change up to down"]
    #[inline] pub fn down(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Counter direction change up to down"]
    #[inline] pub fn test_down(&self) -> bool {
        self.down != 0
    }

    #[doc="Counter direction change up to down"]
    #[inline] pub fn set_down<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Counter direction change down to up"]
    #[inline] pub fn up(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Counter direction change down to up"]
    #[inline] pub fn test_up(&self) -> bool {
        self.up != 0
    }

    #[doc="Counter direction change down to up"]
    #[inline] pub fn set_up<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Autoreload register update OK"]
    #[inline] pub fn arrok(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Autoreload register update OK"]
    #[inline] pub fn test_arrok(&self) -> bool {
        self.arrok != 0
    }

    #[doc="Autoreload register update OK"]
    #[inline] pub fn set_arrok<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Compare register update OK"]
    #[inline] pub fn cmpok(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Compare register update OK"]
    #[inline] pub fn test_cmpok(&self) -> bool {
        self.cmpok != 0
    }

    #[doc="Compare register update OK"]
    #[inline] pub fn set_cmpok<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="External trigger edge event"]
    #[inline] pub fn exttrig(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="External trigger edge event"]
    #[inline] pub fn test_exttrig(&self) -> bool {
        self.exttrig != 0
    }

    #[doc="External trigger edge event"]
    #[inline] pub fn set_exttrig<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autoreload match"]
    #[inline] pub fn arrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Autoreload match"]
    #[inline] pub fn test_arrm(&self) -> bool {
        self.arrm != 0
    }

    #[doc="Autoreload match"]
    #[inline] pub fn set_arrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Compare match"]
    #[inline] pub fn cmpm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Compare match"]
    #[inline] pub fn test_cmpm(&self) -> bool {
        self.cmpm != 0
    }

    #[doc="Compare match"]
    #[inline] pub fn set_cmpm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.down() != 0 { try!(write!(f, " down"))}
        if self.up() != 0 { try!(write!(f, " up"))}
        if self.arrok() != 0 { try!(write!(f, " arrok"))}
        if self.cmpok() != 0 { try!(write!(f, " cmpok"))}
        if self.exttrig() != 0 { try!(write!(f, " exttrig"))}
        if self.arrm() != 0 { try!(write!(f, " arrm"))}
        if self.cmpm() != 0 { try!(write!(f, " cmpm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Clear Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="Direction change to down Clear Flag"]
    #[inline] pub fn downcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Direction change to down Clear Flag"]
    #[inline] pub fn test_downcf(&self) -> bool {
        self.downcf != 0
    }

    #[doc="Direction change to down Clear Flag"]
    #[inline] pub fn set_downcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Direction change to UP Clear Flag"]
    #[inline] pub fn upcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Direction change to UP Clear Flag"]
    #[inline] pub fn test_upcf(&self) -> bool {
        self.upcf != 0
    }

    #[doc="Direction change to UP Clear Flag"]
    #[inline] pub fn set_upcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Autoreload register update OK Clear Flag"]
    #[inline] pub fn arrokcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Autoreload register update OK Clear Flag"]
    #[inline] pub fn test_arrokcf(&self) -> bool {
        self.arrokcf != 0
    }

    #[doc="Autoreload register update OK Clear Flag"]
    #[inline] pub fn set_arrokcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Compare register update OK Clear Flag"]
    #[inline] pub fn cmpokcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Compare register update OK Clear Flag"]
    #[inline] pub fn test_cmpokcf(&self) -> bool {
        self.cmpokcf != 0
    }

    #[doc="Compare register update OK Clear Flag"]
    #[inline] pub fn set_cmpokcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="External trigger valid edge Clear Flag"]
    #[inline] pub fn exttrigcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="External trigger valid edge Clear Flag"]
    #[inline] pub fn test_exttrigcf(&self) -> bool {
        self.exttrigcf != 0
    }

    #[doc="External trigger valid edge Clear Flag"]
    #[inline] pub fn set_exttrigcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autoreload match Clear Flag"]
    #[inline] pub fn arrmcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Autoreload match Clear Flag"]
    #[inline] pub fn test_arrmcf(&self) -> bool {
        self.arrmcf != 0
    }

    #[doc="Autoreload match Clear Flag"]
    #[inline] pub fn set_arrmcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="compare match Clear Flag"]
    #[inline] pub fn cmpmcf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="compare match Clear Flag"]
    #[inline] pub fn test_cmpmcf(&self) -> bool {
        self.cmpmcf != 0
    }

    #[doc="compare match Clear Flag"]
    #[inline] pub fn set_cmpmcf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.downcf() != 0 { try!(write!(f, " downcf"))}
        if self.upcf() != 0 { try!(write!(f, " upcf"))}
        if self.arrokcf() != 0 { try!(write!(f, " arrokcf"))}
        if self.cmpokcf() != 0 { try!(write!(f, " cmpokcf"))}
        if self.exttrigcf() != 0 { try!(write!(f, " exttrigcf"))}
        if self.arrmcf() != 0 { try!(write!(f, " arrmcf"))}
        if self.cmpmcf() != 0 { try!(write!(f, " cmpmcf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Direction change to down Interrupt Enable"]
    #[inline] pub fn downie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Direction change to down Interrupt Enable"]
    #[inline] pub fn test_downie(&self) -> bool {
        self.downie != 0
    }

    #[doc="Direction change to down Interrupt Enable"]
    #[inline] pub fn set_downie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Direction change to UP Interrupt Enable"]
    #[inline] pub fn upie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Direction change to UP Interrupt Enable"]
    #[inline] pub fn test_upie(&self) -> bool {
        self.upie != 0
    }

    #[doc="Direction change to UP Interrupt Enable"]
    #[inline] pub fn set_upie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Autoreload register update OK Interrupt Enable"]
    #[inline] pub fn arrokie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Autoreload register update OK Interrupt Enable"]
    #[inline] pub fn test_arrokie(&self) -> bool {
        self.arrokie != 0
    }

    #[doc="Autoreload register update OK Interrupt Enable"]
    #[inline] pub fn set_arrokie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Compare register update OK Interrupt Enable"]
    #[inline] pub fn cmpokie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Compare register update OK Interrupt Enable"]
    #[inline] pub fn test_cmpokie(&self) -> bool {
        self.cmpokie != 0
    }

    #[doc="Compare register update OK Interrupt Enable"]
    #[inline] pub fn set_cmpokie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="External trigger valid edge Interrupt Enable"]
    #[inline] pub fn exttrigie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="External trigger valid edge Interrupt Enable"]
    #[inline] pub fn test_exttrigie(&self) -> bool {
        self.exttrigie != 0
    }

    #[doc="External trigger valid edge Interrupt Enable"]
    #[inline] pub fn set_exttrigie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Autoreload match Interrupt Enable"]
    #[inline] pub fn arrmie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Autoreload match Interrupt Enable"]
    #[inline] pub fn test_arrmie(&self) -> bool {
        self.arrmie != 0
    }

    #[doc="Autoreload match Interrupt Enable"]
    #[inline] pub fn set_arrmie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Compare match Interrupt Enable"]
    #[inline] pub fn cmpmie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Compare match Interrupt Enable"]
    #[inline] pub fn test_cmpmie(&self) -> bool {
        self.cmpmie != 0
    }

    #[doc="Compare match Interrupt Enable"]
    #[inline] pub fn set_cmpmie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.downie() != 0 { try!(write!(f, " downie"))}
        if self.upie() != 0 { try!(write!(f, " upie"))}
        if self.arrokie() != 0 { try!(write!(f, " arrokie"))}
        if self.cmpokie() != 0 { try!(write!(f, " cmpokie"))}
        if self.exttrigie() != 0 { try!(write!(f, " exttrigie"))}
        if self.arrmie() != 0 { try!(write!(f, " arrmie"))}
        if self.cmpmie() != 0 { try!(write!(f, " cmpmie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc="Encoder mode enable"]
    #[inline] pub fn enc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Encoder mode enable"]
    #[inline] pub fn test_enc(&self) -> bool {
        self.enc != 0
    }

    #[doc="Encoder mode enable"]
    #[inline] pub fn set_enc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="counter mode enabled"]
    #[inline] pub fn countmode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="counter mode enabled"]
    #[inline] pub fn test_countmode(&self) -> bool {
        self.countmode != 0
    }

    #[doc="counter mode enabled"]
    #[inline] pub fn set_countmode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Registers update mode"]
    #[inline] pub fn preload(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Registers update mode"]
    #[inline] pub fn test_preload(&self) -> bool {
        self.preload != 0
    }

    #[doc="Registers update mode"]
    #[inline] pub fn set_preload<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Waveform shape polarity"]
    #[inline] pub fn wavpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Waveform shape polarity"]
    #[inline] pub fn test_wavpol(&self) -> bool {
        self.wavpol != 0
    }

    #[doc="Waveform shape polarity"]
    #[inline] pub fn set_wavpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Waveform shape"]
    #[inline] pub fn wave(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Waveform shape"]
    #[inline] pub fn test_wave(&self) -> bool {
        self.wave != 0
    }

    #[doc="Waveform shape"]
    #[inline] pub fn set_wave<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Timeout enable"]
    #[inline] pub fn timout(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Timeout enable"]
    #[inline] pub fn test_timout(&self) -> bool {
        self.timout != 0
    }

    #[doc="Timeout enable"]
    #[inline] pub fn set_timout<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Trigger enable and polarity"]
    #[inline] pub fn trigen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3) as u8) } // [18:17]
    }

    #[doc="Trigger enable and polarity"]
    #[inline] pub fn test_trigen(&self) -> bool {
        self.trigen != 0
    }

    #[doc="Trigger enable and polarity"]
    #[inline] pub fn set_trigen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Trigger selector"]
    #[inline] pub fn trigsel(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x7) as u8) } // [15:13]
    }

    #[doc="Trigger selector"]
    #[inline] pub fn test_trigsel(&self) -> bool {
        self.trigsel != 0
    }

    #[doc="Trigger selector"]
    #[inline] pub fn set_trigsel<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Clock prescaler"]
    #[inline] pub fn presc(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x7) as u8) } // [11:9]
    }

    #[doc="Clock prescaler"]
    #[inline] pub fn test_presc(&self) -> bool {
        self.presc != 0
    }

    #[doc="Clock prescaler"]
    #[inline] pub fn set_presc<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Configurable digital filter for trigger"]
    #[inline] pub fn trgflt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Configurable digital filter for trigger"]
    #[inline] pub fn test_trgflt(&self) -> bool {
        self.trgflt != 0
    }

    #[doc="Configurable digital filter for trigger"]
    #[inline] pub fn set_trgflt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Configurable digital filter for external clock"]
    #[inline] pub fn ckflt(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Configurable digital filter for external clock"]
    #[inline] pub fn test_ckflt(&self) -> bool {
        self.ckflt != 0
    }

    #[doc="Configurable digital filter for external clock"]
    #[inline] pub fn set_ckflt<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn ckpol(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn test_ckpol(&self) -> bool {
        self.ckpol != 0
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn set_ckpol<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock selector"]
    #[inline] pub fn cksel(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Clock selector"]
    #[inline] pub fn test_cksel(&self) -> bool {
        self.cksel != 0
    }

    #[doc="Clock selector"]
    #[inline] pub fn set_cksel<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.enc() != 0 { try!(write!(f, " enc"))}
        if self.countmode() != 0 { try!(write!(f, " countmode"))}
        if self.preload() != 0 { try!(write!(f, " preload"))}
        if self.wavpol() != 0 { try!(write!(f, " wavpol"))}
        if self.wave() != 0 { try!(write!(f, " wave"))}
        if self.timout() != 0 { try!(write!(f, " timout"))}
        if self.trigen() != 0 { try!(write!(f, " trigen=0x{:x}", self.trigen()))}
        if self.trigsel() != 0 { try!(write!(f, " trigsel=0x{:x}", self.trigsel()))}
        if self.presc() != 0 { try!(write!(f, " presc=0x{:x}", self.presc()))}
        if self.trgflt() != 0 { try!(write!(f, " trgflt=0x{:x}", self.trgflt()))}
        if self.ckflt() != 0 { try!(write!(f, " ckflt=0x{:x}", self.ckflt()))}
        if self.ckpol() != 0 { try!(write!(f, " ckpol=0x{:x}", self.ckpol()))}
        if self.cksel() != 0 { try!(write!(f, " cksel"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Timer start in continuous mode"]
    #[inline] pub fn cntstrt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Timer start in continuous mode"]
    #[inline] pub fn test_cntstrt(&self) -> bool {
        self.cntstrt != 0
    }

    #[doc="Timer start in continuous mode"]
    #[inline] pub fn set_cntstrt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="LPTIM start in single mode"]
    #[inline] pub fn sngstrt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="LPTIM start in single mode"]
    #[inline] pub fn test_sngstrt(&self) -> bool {
        self.sngstrt != 0
    }

    #[doc="LPTIM start in single mode"]
    #[inline] pub fn set_sngstrt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="LPTIM Enable"]
    #[inline] pub fn enable(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="LPTIM Enable"]
    #[inline] pub fn test_enable(&self) -> bool {
        self.enable != 0
    }

    #[doc="LPTIM Enable"]
    #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
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
        if self.cntstrt() != 0 { try!(write!(f, " cntstrt"))}
        if self.sngstrt() != 0 { try!(write!(f, " sngstrt"))}
        if self.enable() != 0 { try!(write!(f, " enable"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cmp(pub u32);
impl Cmp {
    #[doc="Compare value."]
    #[inline] pub fn cmp(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Compare value."]
    #[inline] pub fn test_cmp(&self) -> bool {
        self.cmp != 0
    }

    #[doc="Compare value."]
    #[inline] pub fn set_cmp<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Cmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cmp() != 0 { try!(write!(f, " cmp=0x{:x}", self.cmp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Autoreload Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc="Auto reload value."]
    #[inline] pub fn arr(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Auto reload value."]
    #[inline] pub fn test_arr(&self) -> bool {
        self.arr != 0
    }

    #[doc="Auto reload value."]
    #[inline] pub fn set_arr<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Arr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Arr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.arr() != 0 { try!(write!(f, " arr=0x{:x}", self.arr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="Counter value."]
    #[inline] pub fn cnt(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Counter value."]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt != 0
    }

    #[doc="Counter value."]
    #[inline] pub fn set_cnt<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


