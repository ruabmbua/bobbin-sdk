//! Cyclic Redundancy Check
#[allow(unused_imports)] use bobbin_common::*;

periph!(CRC, Crc, 0x40032000);

#[doc="Cyclic Redundancy Check"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Crc(pub usize);
impl Crc {
    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x0) as *const u32
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x0) as *mut u32
    }

    #[doc="Read the DATA register."]
    #[inline] pub fn data(&self) -> Data { 
        unsafe {
            Data(read_volatile((self.0 + 0x0) as *const u32))
        }
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        let value = f(Data(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the DATA register."]
    #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        let tmp = self.data();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DATAL register."]
    #[inline] pub fn datal_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0x0) as *const u16
    }

    #[doc="Get the *mut pointer for the DATAL register."]
    #[inline] pub fn datal_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0x0) as *mut u16
    }

    #[doc="Read the DATAL register."]
    #[inline] pub fn datal(&self) -> Datal { 
        unsafe {
            Datal(read_volatile((self.0 + 0x0) as *const u16))
        }
    }

    #[doc="Write the DATAL register."]
    #[inline] pub fn set_datal<F: FnOnce(Datal) -> Datal>(&self, f: F) -> &Self {
        let value = f(Datal(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u16, value.0);
        }
        self
    }

    #[doc="Modify the DATAL register."]
    #[inline] pub fn with_datal<F: FnOnce(Datal) -> Datal>(&self, f: F) -> &Self {
        let tmp = self.datal();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u16, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DATALL register."]
    #[inline] pub fn datall_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x0) as *const u8
    }

    #[doc="Get the *mut pointer for the DATALL register."]
    #[inline] pub fn datall_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x0) as *mut u8
    }

    #[doc="Read the DATALL register."]
    #[inline] pub fn datall(&self) -> Datall { 
        unsafe {
            Datall(read_volatile((self.0 + 0x0) as *const u8))
        }
    }

    #[doc="Write the DATALL register."]
    #[inline] pub fn set_datall<F: FnOnce(Datall) -> Datall>(&self, f: F) -> &Self {
        let value = f(Datall(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the DATALL register."]
    #[inline] pub fn with_datall<F: FnOnce(Datall) -> Datall>(&self, f: F) -> &Self {
        let tmp = self.datall();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DATALU register."]
    #[inline] pub fn datalu_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x1) as *const u8
    }

    #[doc="Get the *mut pointer for the DATALU register."]
    #[inline] pub fn datalu_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x1) as *mut u8
    }

    #[doc="Read the DATALU register."]
    #[inline] pub fn datalu(&self) -> Datalu { 
        unsafe {
            Datalu(read_volatile((self.0 + 0x1) as *const u8))
        }
    }

    #[doc="Write the DATALU register."]
    #[inline] pub fn set_datalu<F: FnOnce(Datalu) -> Datalu>(&self, f: F) -> &Self {
        let value = f(Datalu(0));
        unsafe {
            write_volatile((self.0 + 0x1) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the DATALU register."]
    #[inline] pub fn with_datalu<F: FnOnce(Datalu) -> Datalu>(&self, f: F) -> &Self {
        let tmp = self.datalu();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x1) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DATAH register."]
    #[inline] pub fn datah_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0x2) as *const u16
    }

    #[doc="Get the *mut pointer for the DATAH register."]
    #[inline] pub fn datah_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0x2) as *mut u16
    }

    #[doc="Read the DATAH register."]
    #[inline] pub fn datah(&self) -> Datah { 
        unsafe {
            Datah(read_volatile((self.0 + 0x2) as *const u16))
        }
    }

    #[doc="Write the DATAH register."]
    #[inline] pub fn set_datah<F: FnOnce(Datah) -> Datah>(&self, f: F) -> &Self {
        let value = f(Datah(0));
        unsafe {
            write_volatile((self.0 + 0x2) as *mut u16, value.0);
        }
        self
    }

    #[doc="Modify the DATAH register."]
    #[inline] pub fn with_datah<F: FnOnce(Datah) -> Datah>(&self, f: F) -> &Self {
        let tmp = self.datah();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x2) as *mut u16, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DATAHL register."]
    #[inline] pub fn datahl_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x2) as *const u8
    }

    #[doc="Get the *mut pointer for the DATAHL register."]
    #[inline] pub fn datahl_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x2) as *mut u8
    }

    #[doc="Read the DATAHL register."]
    #[inline] pub fn datahl(&self) -> Datahl { 
        unsafe {
            Datahl(read_volatile((self.0 + 0x2) as *const u8))
        }
    }

    #[doc="Write the DATAHL register."]
    #[inline] pub fn set_datahl<F: FnOnce(Datahl) -> Datahl>(&self, f: F) -> &Self {
        let value = f(Datahl(0));
        unsafe {
            write_volatile((self.0 + 0x2) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the DATAHL register."]
    #[inline] pub fn with_datahl<F: FnOnce(Datahl) -> Datahl>(&self, f: F) -> &Self {
        let tmp = self.datahl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x2) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DATAHU register."]
    #[inline] pub fn datahu_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0x3) as *const u8
    }

    #[doc="Get the *mut pointer for the DATAHU register."]
    #[inline] pub fn datahu_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0x3) as *mut u8
    }

    #[doc="Read the DATAHU register."]
    #[inline] pub fn datahu(&self) -> Datahu { 
        unsafe {
            Datahu(read_volatile((self.0 + 0x3) as *const u8))
        }
    }

    #[doc="Write the DATAHU register."]
    #[inline] pub fn set_datahu<F: FnOnce(Datahu) -> Datahu>(&self, f: F) -> &Self {
        let value = f(Datahu(0));
        unsafe {
            write_volatile((self.0 + 0x3) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the DATAHU register."]
    #[inline] pub fn with_datahu<F: FnOnce(Datahu) -> Datahu>(&self, f: F) -> &Self {
        let tmp = self.datahu();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x3) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the GPOLY register."]
    #[inline] pub fn gpoly_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4) as *const u32
    }

    #[doc="Get the *mut pointer for the GPOLY register."]
    #[inline] pub fn gpoly_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4) as *mut u32
    }

    #[doc="Read the GPOLY register."]
    #[inline] pub fn gpoly(&self) -> Gpoly { 
        unsafe {
            Gpoly(read_volatile((self.0 + 0x4) as *const u32))
        }
    }

    #[doc="Write the GPOLY register."]
    #[inline] pub fn set_gpoly<F: FnOnce(Gpoly) -> Gpoly>(&self, f: F) -> &Self {
        let value = f(Gpoly(0));
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the GPOLY register."]
    #[inline] pub fn with_gpoly<F: FnOnce(Gpoly) -> Gpoly>(&self, f: F) -> &Self {
        let tmp = self.gpoly();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x8) as *const u32
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x8) as *mut u32
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        unsafe {
            Ctrl(read_volatile((self.0 + 0x8) as *const u32))
        }
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        let value = f(Ctrl(0));
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        let tmp = self.ctrl();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

}

#[doc="CRC Data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
    #[doc="CRC Low Lower Byte"]
    #[inline] pub fn ll(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="CRC Low Lower Byte"]
    #[inline] pub fn test_ll(&self) -> bool {
        self.ll != 0
    }

    #[doc="CRC Low Lower Byte"]
    #[inline] pub fn set_ll<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CRC Low Upper Byte"]
    #[inline] pub fn lu(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="CRC Low Upper Byte"]
    #[inline] pub fn test_lu(&self) -> bool {
        self.lu != 0
    }

    #[doc="CRC Low Upper Byte"]
    #[inline] pub fn set_lu<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CRC High Lower Byte"]
    #[inline] pub fn hl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="CRC High Lower Byte"]
    #[inline] pub fn test_hl(&self) -> bool {
        self.hl != 0
    }

    #[doc="CRC High Lower Byte"]
    #[inline] pub fn set_hl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CRC High Upper Byte"]
    #[inline] pub fn hu(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="CRC High Upper Byte"]
    #[inline] pub fn test_hu(&self) -> bool {
        self.hu != 0
    }

    #[doc="CRC High Upper Byte"]
    #[inline] pub fn set_hu<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl ::core::fmt::Display for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ll() != 0 { try!(write!(f, " ll=0x{:x}", self.ll()))}
        if self.lu() != 0 { try!(write!(f, " lu=0x{:x}", self.lu()))}
        if self.hl() != 0 { try!(write!(f, " hl=0x{:x}", self.hl()))}
        if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATAL register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datal(pub u16);
impl Datal {
    #[doc="DATAL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline] pub fn datal(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="DATAL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline] pub fn test_datal(&self) -> bool {
        self.datal != 0
    }

    #[doc="DATAL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline] pub fn set_datal<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Datal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datal() != 0 { try!(write!(f, " datal=0x{:x}", self.datal()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATALL register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datall(pub u8);
impl Datall {
    #[doc="CRCLL stores the first 8 bits of the 32 bit DATA"]
    #[inline] pub fn datall(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="CRCLL stores the first 8 bits of the 32 bit DATA"]
    #[inline] pub fn test_datall(&self) -> bool {
        self.datall != 0
    }

    #[doc="CRCLL stores the first 8 bits of the 32 bit DATA"]
    #[inline] pub fn set_datall<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Datall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datall() != 0 { try!(write!(f, " datall=0x{:x}", self.datall()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATALU register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datalu(pub u8);
impl Datalu {
    #[doc="DATALL stores the second 8 bits of the 32 bit CRC"]
    #[inline] pub fn datalu(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="DATALL stores the second 8 bits of the 32 bit CRC"]
    #[inline] pub fn test_datalu(&self) -> bool {
        self.datalu != 0
    }

    #[doc="DATALL stores the second 8 bits of the 32 bit CRC"]
    #[inline] pub fn set_datalu<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Datalu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datalu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datalu() != 0 { try!(write!(f, " datalu=0x{:x}", self.datalu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATAH register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datah(pub u16);
impl Datah {
    #[doc="DATAH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline] pub fn datah(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="DATAH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline] pub fn test_datah(&self) -> bool {
        self.datah != 0
    }

    #[doc="DATAH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline] pub fn set_datah<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Datah {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datah {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datah() != 0 { try!(write!(f, " datah=0x{:x}", self.datah()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATAHL register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datahl(pub u8);
impl Datahl {
    #[doc="DATAHL stores the third 8 bits of the 32 bit CRC"]
    #[inline] pub fn datahl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="DATAHL stores the third 8 bits of the 32 bit CRC"]
    #[inline] pub fn test_datahl(&self) -> bool {
        self.datahl != 0
    }

    #[doc="DATAHL stores the third 8 bits of the 32 bit CRC"]
    #[inline] pub fn set_datahl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Datahl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datahl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datahl() != 0 { try!(write!(f, " datahl=0x{:x}", self.datahl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATAHU register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datahu(pub u8);
impl Datahu {
    #[doc="DATAHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline] pub fn datahu(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="DATAHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline] pub fn test_datahu(&self) -> bool {
        self.datahu != 0
    }

    #[doc="DATAHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline] pub fn set_datahu<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Datahu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datahu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datahu() != 0 { try!(write!(f, " datahu=0x{:x}", self.datahu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Polynomial register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpoly(pub u32);
impl Gpoly {
    #[doc="Low Polynominal Half-word"]
    #[inline] pub fn low(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Low Polynominal Half-word"]
    #[inline] pub fn test_low(&self) -> bool {
        self.low != 0
    }

    #[doc="Low Polynominal Half-word"]
    #[inline] pub fn set_low<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High Polynominal Half-word"]
    #[inline] pub fn high(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="High Polynominal Half-word"]
    #[inline] pub fn test_high(&self) -> bool {
        self.high != 0
    }

    #[doc="High Polynominal Half-word"]
    #[inline] pub fn set_high<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl ::core::fmt::Display for Gpoly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpoly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.low() != 0 { try!(write!(f, " low=0x{:x}", self.low()))}
        if self.high() != 0 { try!(write!(f, " high=0x{:x}", self.high()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="TCRC"]
    #[inline] pub fn tcrc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="TCRC"]
    #[inline] pub fn test_tcrc(&self) -> bool {
        self.tcrc != 0
    }

    #[doc="TCRC"]
    #[inline] pub fn set_tcrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Write CRC Data Register As Seed"]
    #[inline] pub fn was(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Write CRC Data Register As Seed"]
    #[inline] pub fn test_was(&self) -> bool {
        self.was != 0
    }

    #[doc="Write CRC Data Register As Seed"]
    #[inline] pub fn set_was<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Complement Read Of CRC Data Register"]
    #[inline] pub fn fxor(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Complement Read Of CRC Data Register"]
    #[inline] pub fn test_fxor(&self) -> bool {
        self.fxor != 0
    }

    #[doc="Complement Read Of CRC Data Register"]
    #[inline] pub fn set_fxor<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Type Of Transpose For Read"]
    #[inline] pub fn totr(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Type Of Transpose For Read"]
    #[inline] pub fn test_totr(&self) -> bool {
        self.totr != 0
    }

    #[doc="Type Of Transpose For Read"]
    #[inline] pub fn set_totr<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Type Of Transpose For Writes"]
    #[inline] pub fn tot(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Type Of Transpose For Writes"]
    #[inline] pub fn test_tot(&self) -> bool {
        self.tot != 0
    }

    #[doc="Type Of Transpose For Writes"]
    #[inline] pub fn set_tot<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

}

impl ::core::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcrc() != 0 { try!(write!(f, " tcrc"))}
        if self.was() != 0 { try!(write!(f, " was"))}
        if self.fxor() != 0 { try!(write!(f, " fxor"))}
        if self.totr() != 0 { try!(write!(f, " totr=0x{:x}", self.totr()))}
        if self.tot() != 0 { try!(write!(f, " tot=0x{:x}", self.tot()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


