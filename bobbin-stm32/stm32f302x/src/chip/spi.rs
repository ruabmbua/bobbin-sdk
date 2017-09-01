#[allow(unused_imports)] use bobbin_common::*;

periph!( SPI1, Spi1, _SPI1, SpiPeriph, 0x40013000);
periph!( SPI2, Spi2, _SPI2, SpiPeriph, 0x40003800);
periph!( SPI3, Spi3, _SPI3, SpiPeriph, 0x40003c00);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SPI Peripheral"]
pub struct SpiPeriph(pub usize); 





impl SpiPeriph {
    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x0) as *const u32
    }

    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x0) as *mut u32
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        unsafe {
            Cr1(read_volatile((self.0 + 0x0) as *const u32))
        }
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        let value = f(Cr1(0));
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        let tmp = self.cr1();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x0) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x4) as *const u32
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x4) as *mut u32
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        unsafe {
            Cr2(read_volatile((self.0 + 0x4) as *const u32))
        }
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        let value = f(Cr2(0));
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        let tmp = self.cr2();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x4) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x8) as *const u32
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x8) as *mut u32
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            Sr(read_volatile((self.0 + 0x8) as *const u32))
        }
    }

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        let value = f(Sr(0));
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        let tmp = self.sr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x8) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const u16 { 
        ((self.0 as usize) + 0xc) as *const u16
    }

    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut u16 { 
        ((self.0 as usize) + 0xc) as *mut u16
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        unsafe {
            Dr(read_volatile((self.0 + 0xc) as *const u16))
        }
    }

    #[doc="Write the DR register."]
    #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        let value = f(Dr(0));
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u16, value.0);
        }
        self
    }

    #[doc="Modify the DR register."]
    #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        let tmp = self.dr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u16, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the DR8 register."]
    #[inline] pub fn dr8_ptr(&self) -> *const u8 { 
        ((self.0 as usize) + 0xc) as *const u8
    }

    #[doc="Get the *mut pointer for the DR8 register."]
    #[inline] pub fn dr8_mut(&self) -> *mut u8 { 
        ((self.0 as usize) + 0xc) as *mut u8
    }

    #[doc="Read the DR8 register."]
    #[inline] pub fn dr8(&self) -> Dr8 { 
        unsafe {
            Dr8(read_volatile((self.0 + 0xc) as *const u8))
        }
    }

    #[doc="Write the DR8 register."]
    #[inline] pub fn set_dr8<F: FnOnce(Dr8) -> Dr8>(&self, f: F) -> &Self {
        let value = f(Dr8(0));
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u8, value.0);
        }
        self
    }

    #[doc="Modify the DR8 register."]
    #[inline] pub fn with_dr8<F: FnOnce(Dr8) -> Dr8>(&self, f: F) -> &Self {
        let tmp = self.dr8();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0xc) as *mut u8, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the CRCPR register."]
    #[inline] pub fn crcpr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x10) as *const u32
    }

    #[doc="Get the *mut pointer for the CRCPR register."]
    #[inline] pub fn crcpr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x10) as *mut u32
    }

    #[doc="Read the CRCPR register."]
    #[inline] pub fn crcpr(&self) -> Crcpr { 
        unsafe {
            Crcpr(read_volatile((self.0 + 0x10) as *const u32))
        }
    }

    #[doc="Write the CRCPR register."]
    #[inline] pub fn set_crcpr<F: FnOnce(Crcpr) -> Crcpr>(&self, f: F) -> &Self {
        let value = f(Crcpr(0));
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the CRCPR register."]
    #[inline] pub fn with_crcpr<F: FnOnce(Crcpr) -> Crcpr>(&self, f: F) -> &Self {
        let tmp = self.crcpr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x10) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the RXCRCR register."]
    #[inline] pub fn rxcrcr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x14) as *const u32
    }

    #[doc="Get the *mut pointer for the RXCRCR register."]
    #[inline] pub fn rxcrcr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x14) as *mut u32
    }

    #[doc="Read the RXCRCR register."]
    #[inline] pub fn rxcrcr(&self) -> Rxcrcr { 
        unsafe {
            Rxcrcr(read_volatile((self.0 + 0x14) as *const u32))
        }
    }

    #[doc="Get the *const pointer for the TXCRCR register."]
    #[inline] pub fn txcrcr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x18) as *const u32
    }

    #[doc="Get the *mut pointer for the TXCRCR register."]
    #[inline] pub fn txcrcr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x18) as *mut u32
    }

    #[doc="Read the TXCRCR register."]
    #[inline] pub fn txcrcr(&self) -> Txcrcr { 
        unsafe {
            Txcrcr(read_volatile((self.0 + 0x18) as *const u32))
        }
    }

    #[doc="Get the *const pointer for the I2SCFGR register."]
    #[inline] pub fn i2scfgr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x1c) as *const u32
    }

    #[doc="Get the *mut pointer for the I2SCFGR register."]
    #[inline] pub fn i2scfgr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x1c) as *mut u32
    }

    #[doc="Read the I2SCFGR register."]
    #[inline] pub fn i2scfgr(&self) -> I2scfgr { 
        unsafe {
            I2scfgr(read_volatile((self.0 + 0x1c) as *const u32))
        }
    }

    #[doc="Write the I2SCFGR register."]
    #[inline] pub fn set_i2scfgr<F: FnOnce(I2scfgr) -> I2scfgr>(&self, f: F) -> &Self {
        let value = f(I2scfgr(0));
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the I2SCFGR register."]
    #[inline] pub fn with_i2scfgr<F: FnOnce(I2scfgr) -> I2scfgr>(&self, f: F) -> &Self {
        let tmp = self.i2scfgr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x1c) as *mut u32, value.0);
        }
        self
    }

    #[doc="Get the *const pointer for the I2SPR register."]
    #[inline] pub fn i2spr_ptr(&self) -> *const u32 { 
        ((self.0 as usize) + 0x20) as *const u32
    }

    #[doc="Get the *mut pointer for the I2SPR register."]
    #[inline] pub fn i2spr_mut(&self) -> *mut u32 { 
        ((self.0 as usize) + 0x20) as *mut u32
    }

    #[doc="Read the I2SPR register."]
    #[inline] pub fn i2spr(&self) -> I2spr { 
        unsafe {
            I2spr(read_volatile((self.0 + 0x20) as *const u32))
        }
    }

    #[doc="Write the I2SPR register."]
    #[inline] pub fn set_i2spr<F: FnOnce(I2spr) -> I2spr>(&self, f: F) -> &Self {
        let value = f(I2spr(0));
        unsafe {
            write_volatile((self.0 + 0x20) as *mut u32, value.0);
        }
        self
    }

    #[doc="Modify the I2SPR register."]
    #[inline] pub fn with_i2spr<F: FnOnce(I2spr) -> I2spr>(&self, f: F) -> &Self {
        let tmp = self.i2spr();
        let value = f(tmp);
        unsafe {
            write_volatile((self.0 + 0x20) as *mut u32, value.0);
        }
        self
    }

}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Bidirectional data mode enable"]
    #[inline] pub fn bidimode(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Bidirectional data mode enable"]
    #[inline] pub fn test_bidimode(&self) -> bool {
        self.bidimode != 0
    }

    #[doc="Bidirectional data mode enable"]
    #[inline] pub fn set_bidimode<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Output enable in bidirectional mode"]
    #[inline] pub fn bidioe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Output enable in bidirectional mode"]
    #[inline] pub fn test_bidioe(&self) -> bool {
        self.bidioe != 0
    }

    #[doc="Output enable in bidirectional mode"]
    #[inline] pub fn set_bidioe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Hardware CRC calculation enable"]
    #[inline] pub fn crcen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Hardware CRC calculation enable"]
    #[inline] pub fn test_crcen(&self) -> bool {
        self.crcen != 0
    }

    #[doc="Hardware CRC calculation enable"]
    #[inline] pub fn set_crcen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CRC transfer next"]
    #[inline] pub fn crcnext(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="CRC transfer next"]
    #[inline] pub fn test_crcnext(&self) -> bool {
        self.crcnext != 0
    }

    #[doc="CRC transfer next"]
    #[inline] pub fn set_crcnext<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Data frame format"]
    #[inline] pub fn dff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Data frame format"]
    #[inline] pub fn test_dff(&self) -> bool {
        self.dff != 0
    }

    #[doc="Data frame format"]
    #[inline] pub fn set_dff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Receive only"]
    #[inline] pub fn rxonly(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Receive only"]
    #[inline] pub fn test_rxonly(&self) -> bool {
        self.rxonly != 0
    }

    #[doc="Receive only"]
    #[inline] pub fn set_rxonly<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Software slave management"]
    #[inline] pub fn ssm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Software slave management"]
    #[inline] pub fn test_ssm(&self) -> bool {
        self.ssm != 0
    }

    #[doc="Software slave management"]
    #[inline] pub fn set_ssm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Internal slave select"]
    #[inline] pub fn ssi(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Internal slave select"]
    #[inline] pub fn test_ssi(&self) -> bool {
        self.ssi != 0
    }

    #[doc="Internal slave select"]
    #[inline] pub fn set_ssi<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame format"]
    #[inline] pub fn lsbfirst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Frame format"]
    #[inline] pub fn test_lsbfirst(&self) -> bool {
        self.lsbfirst != 0
    }

    #[doc="Frame format"]
    #[inline] pub fn set_lsbfirst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SPI enable"]
    #[inline] pub fn spe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="SPI enable"]
    #[inline] pub fn test_spe(&self) -> bool {
        self.spe != 0
    }

    #[doc="SPI enable"]
    #[inline] pub fn set_spe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Baud rate control"]
    #[inline] pub fn br(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Baud rate control"]
    #[inline] pub fn test_br(&self) -> bool {
        self.br != 0
    }

    #[doc="Baud rate control"]
    #[inline] pub fn set_br<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Master selection"]
    #[inline] pub fn mstr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Master selection"]
    #[inline] pub fn test_mstr(&self) -> bool {
        self.mstr != 0
    }

    #[doc="Master selection"]
    #[inline] pub fn set_mstr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clock polarity"]
    #[inline] pub fn cpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Clock polarity"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol != 0
    }

    #[doc="Clock polarity"]
    #[inline] pub fn set_cpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock phase"]
    #[inline] pub fn cpha(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Clock phase"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha != 0
    }

    #[doc="Clock phase"]
    #[inline] pub fn set_cpha<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bidimode() != 0 { try!(write!(f, " bidimode"))}
        if self.bidioe() != 0 { try!(write!(f, " bidioe"))}
        if self.crcen() != 0 { try!(write!(f, " crcen"))}
        if self.crcnext() != 0 { try!(write!(f, " crcnext"))}
        if self.dff() != 0 { try!(write!(f, " dff"))}
        if self.rxonly() != 0 { try!(write!(f, " rxonly"))}
        if self.ssm() != 0 { try!(write!(f, " ssm"))}
        if self.ssi() != 0 { try!(write!(f, " ssi"))}
        if self.lsbfirst() != 0 { try!(write!(f, " lsbfirst"))}
        if self.spe() != 0 { try!(write!(f, " spe"))}
        if self.br() != 0 { try!(write!(f, " br=0x{:x}", self.br()))}
        if self.mstr() != 0 { try!(write!(f, " mstr"))}
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        if self.cpha() != 0 { try!(write!(f, " cpha"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="control register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc="Rx buffer DMA enable"]
    #[inline] pub fn rxdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Rx buffer DMA enable"]
    #[inline] pub fn test_rxdmaen(&self) -> bool {
        self.rxdmaen != 0
    }

    #[doc="Rx buffer DMA enable"]
    #[inline] pub fn set_rxdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Tx buffer DMA enable"]
    #[inline] pub fn txdmaen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Tx buffer DMA enable"]
    #[inline] pub fn test_txdmaen(&self) -> bool {
        self.txdmaen != 0
    }

    #[doc="Tx buffer DMA enable"]
    #[inline] pub fn set_txdmaen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SS output enable"]
    #[inline] pub fn ssoe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="SS output enable"]
    #[inline] pub fn test_ssoe(&self) -> bool {
        self.ssoe != 0
    }

    #[doc="SS output enable"]
    #[inline] pub fn set_ssoe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="NSS pulse management"]
    #[inline] pub fn nssp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="NSS pulse management"]
    #[inline] pub fn test_nssp(&self) -> bool {
        self.nssp != 0
    }

    #[doc="NSS pulse management"]
    #[inline] pub fn set_nssp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Frame format"]
    #[inline] pub fn frf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Frame format"]
    #[inline] pub fn test_frf(&self) -> bool {
        self.frf != 0
    }

    #[doc="Frame format"]
    #[inline] pub fn set_frf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie != 0
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RX buffer not empty interrupt enable"]
    #[inline] pub fn rxneie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="RX buffer not empty interrupt enable"]
    #[inline] pub fn test_rxneie(&self) -> bool {
        self.rxneie != 0
    }

    #[doc="RX buffer not empty interrupt enable"]
    #[inline] pub fn set_rxneie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Tx buffer empty interrupt enable"]
    #[inline] pub fn txeie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Tx buffer empty interrupt enable"]
    #[inline] pub fn test_txeie(&self) -> bool {
        self.txeie != 0
    }

    #[doc="Tx buffer empty interrupt enable"]
    #[inline] pub fn set_txeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Data size"]
    #[inline] pub fn ds(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Data size"]
    #[inline] pub fn test_ds(&self) -> bool {
        self.ds != 0
    }

    #[doc="Data size"]
    #[inline] pub fn set_ds<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FIFO reception threshold"]
    #[inline] pub fn frxth(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="FIFO reception threshold"]
    #[inline] pub fn test_frxth(&self) -> bool {
        self.frxth != 0
    }

    #[doc="FIFO reception threshold"]
    #[inline] pub fn set_frxth<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Last DMA transfer for reception"]
    #[inline] pub fn ldma_rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Last DMA transfer for reception"]
    #[inline] pub fn test_ldma_rx(&self) -> bool {
        self.ldma_rx != 0
    }

    #[doc="Last DMA transfer for reception"]
    #[inline] pub fn set_ldma_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Last DMA transfer for transmission"]
    #[inline] pub fn ldma_tx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Last DMA transfer for transmission"]
    #[inline] pub fn test_ldma_tx(&self) -> bool {
        self.ldma_tx != 0
    }

    #[doc="Last DMA transfer for transmission"]
    #[inline] pub fn set_ldma_tx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl ::core::fmt::Display for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdmaen() != 0 { try!(write!(f, " rxdmaen"))}
        if self.txdmaen() != 0 { try!(write!(f, " txdmaen"))}
        if self.ssoe() != 0 { try!(write!(f, " ssoe"))}
        if self.nssp() != 0 { try!(write!(f, " nssp"))}
        if self.frf() != 0 { try!(write!(f, " frf"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.rxneie() != 0 { try!(write!(f, " rxneie"))}
        if self.txeie() != 0 { try!(write!(f, " txeie"))}
        if self.ds() != 0 { try!(write!(f, " ds=0x{:x}", self.ds()))}
        if self.frxth() != 0 { try!(write!(f, " frxth"))}
        if self.ldma_rx() != 0 { try!(write!(f, " ldma_rx"))}
        if self.ldma_tx() != 0 { try!(write!(f, " ldma_tx"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Receive buffer not empty"]
    #[inline] pub fn rxne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Receive buffer not empty"]
    #[inline] pub fn test_rxne(&self) -> bool {
        self.rxne != 0
    }

    #[doc="Receive buffer not empty"]
    #[inline] pub fn set_rxne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit buffer empty"]
    #[inline] pub fn txe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Transmit buffer empty"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe != 0
    }

    #[doc="Transmit buffer empty"]
    #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel side"]
    #[inline] pub fn chside(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Channel side"]
    #[inline] pub fn test_chside(&self) -> bool {
        self.chside != 0
    }

    #[doc="Channel side"]
    #[inline] pub fn set_chside<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Underrun flag"]
    #[inline] pub fn udr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Underrun flag"]
    #[inline] pub fn test_udr(&self) -> bool {
        self.udr != 0
    }

    #[doc="Underrun flag"]
    #[inline] pub fn set_udr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="CRC error flag"]
    #[inline] pub fn crcerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="CRC error flag"]
    #[inline] pub fn test_crcerr(&self) -> bool {
        self.crcerr != 0
    }

    #[doc="CRC error flag"]
    #[inline] pub fn set_crcerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Mode fault"]
    #[inline] pub fn modf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Mode fault"]
    #[inline] pub fn test_modf(&self) -> bool {
        self.modf != 0
    }

    #[doc="Mode fault"]
    #[inline] pub fn set_modf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Overrun flag"]
    #[inline] pub fn ovr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Overrun flag"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr != 0
    }

    #[doc="Overrun flag"]
    #[inline] pub fn set_ovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Busy flag"]
    #[inline] pub fn bsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Busy flag"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy != 0
    }

    #[doc="Busy flag"]
    #[inline] pub fn set_bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TI frame format error"]
    #[inline] pub fn tifrfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="TI frame format error"]
    #[inline] pub fn test_tifrfe(&self) -> bool {
        self.tifrfe != 0
    }

    #[doc="TI frame format error"]
    #[inline] pub fn set_tifrfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FIFO reception level"]
    #[inline] pub fn frlvl(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="FIFO reception level"]
    #[inline] pub fn test_frlvl(&self) -> bool {
        self.frlvl != 0
    }

    #[doc="FIFO reception level"]
    #[inline] pub fn set_frlvl<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="FIFO transmission level"]
    #[inline] pub fn ftlvl(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="FIFO transmission level"]
    #[inline] pub fn test_ftlvl(&self) -> bool {
        self.ftlvl != 0
    }

    #[doc="FIFO transmission level"]
    #[inline] pub fn set_ftlvl<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
        self
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
        if self.rxne() != 0 { try!(write!(f, " rxne"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        if self.chside() != 0 { try!(write!(f, " chside"))}
        if self.udr() != 0 { try!(write!(f, " udr"))}
        if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
        if self.modf() != 0 { try!(write!(f, " modf"))}
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        if self.tifrfe() != 0 { try!(write!(f, " tifrfe"))}
        if self.frlvl() != 0 { try!(write!(f, " frlvl=0x{:x}", self.frlvl()))}
        if self.ftlvl() != 0 { try!(write!(f, " ftlvl=0x{:x}", self.ftlvl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u16);
impl Dr {
    #[doc="Data register"]
    #[inline] pub fn dr(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Data register"]
    #[inline] pub fn test_dr(&self) -> bool {
        self.dr != 0
    }

    #[doc="Data register"]
    #[inline] pub fn set_dr<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dr() != 0 { try!(write!(f, " dr=0x{:x}", self.dr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr8(pub u8);
impl Dr8 {
    #[doc="Data register"]
    #[inline] pub fn dr8(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Data register"]
    #[inline] pub fn test_dr8(&self) -> bool {
        self.dr8 != 0
    }

    #[doc="Data register"]
    #[inline] pub fn set_dr8<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Dr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dr8() != 0 { try!(write!(f, " dr8=0x{:x}", self.dr8()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC polynomial register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Crcpr(pub u32);
impl Crcpr {
    #[doc="CRC polynomial register"]
    #[inline] pub fn crcpoly(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="CRC polynomial register"]
    #[inline] pub fn test_crcpoly(&self) -> bool {
        self.crcpoly != 0
    }

    #[doc="CRC polynomial register"]
    #[inline] pub fn set_crcpoly<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Crcpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Crcpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.crcpoly() != 0 { try!(write!(f, " crcpoly=0x{:x}", self.crcpoly()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RX CRC register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxcrcr(pub u32);
impl Rxcrcr {
    #[doc="Rx CRC register"]
    #[inline] pub fn rxcrc(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Rx CRC register"]
    #[inline] pub fn test_rxcrc(&self) -> bool {
        self.rxcrc != 0
    }

    #[doc="Rx CRC register"]
    #[inline] pub fn set_rxcrc<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Rxcrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxcrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxcrc() != 0 { try!(write!(f, " rxcrc=0x{:x}", self.rxcrc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TX CRC register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txcrcr(pub u32);
impl Txcrcr {
    #[doc="Tx CRC register"]
    #[inline] pub fn txcrc(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Tx CRC register"]
    #[inline] pub fn test_txcrc(&self) -> bool {
        self.txcrc != 0
    }

    #[doc="Tx CRC register"]
    #[inline] pub fn set_txcrc<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for Txcrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txcrcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txcrc() != 0 { try!(write!(f, " txcrc=0x{:x}", self.txcrc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2S configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct I2scfgr(pub u32);
impl I2scfgr {
    #[doc="I2S mode selection"]
    #[inline] pub fn i2smod(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="I2S mode selection"]
    #[inline] pub fn test_i2smod(&self) -> bool {
        self.i2smod != 0
    }

    #[doc="I2S mode selection"]
    #[inline] pub fn set_i2smod<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="I2S Enable"]
    #[inline] pub fn i2se(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="I2S Enable"]
    #[inline] pub fn test_i2se(&self) -> bool {
        self.i2se != 0
    }

    #[doc="I2S Enable"]
    #[inline] pub fn set_i2se<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="I2S configuration mode"]
    #[inline] pub fn i2scfg(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="I2S configuration mode"]
    #[inline] pub fn test_i2scfg(&self) -> bool {
        self.i2scfg != 0
    }

    #[doc="I2S configuration mode"]
    #[inline] pub fn set_i2scfg<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PCM frame synchronization"]
    #[inline] pub fn pcmsync(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="PCM frame synchronization"]
    #[inline] pub fn test_pcmsync(&self) -> bool {
        self.pcmsync != 0
    }

    #[doc="PCM frame synchronization"]
    #[inline] pub fn set_pcmsync<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2S standard selection"]
    #[inline] pub fn i2sstd(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="I2S standard selection"]
    #[inline] pub fn test_i2sstd(&self) -> bool {
        self.i2sstd != 0
    }

    #[doc="I2S standard selection"]
    #[inline] pub fn set_i2sstd<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Steady state clock polarity"]
    #[inline] pub fn ckpol(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Steady state clock polarity"]
    #[inline] pub fn test_ckpol(&self) -> bool {
        self.ckpol != 0
    }

    #[doc="Steady state clock polarity"]
    #[inline] pub fn set_ckpol<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data length to be transferred"]
    #[inline] pub fn datlen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Data length to be transferred"]
    #[inline] pub fn test_datlen(&self) -> bool {
        self.datlen != 0
    }

    #[doc="Data length to be transferred"]
    #[inline] pub fn set_datlen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel length (number of bits per audio channel)"]
    #[inline] pub fn chlen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Channel length (number of bits per audio channel)"]
    #[inline] pub fn test_chlen(&self) -> bool {
        self.chlen != 0
    }

    #[doc="Channel length (number of bits per audio channel)"]
    #[inline] pub fn set_chlen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for I2scfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for I2scfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.i2smod() != 0 { try!(write!(f, " i2smod"))}
        if self.i2se() != 0 { try!(write!(f, " i2se"))}
        if self.i2scfg() != 0 { try!(write!(f, " i2scfg=0x{:x}", self.i2scfg()))}
        if self.pcmsync() != 0 { try!(write!(f, " pcmsync"))}
        if self.i2sstd() != 0 { try!(write!(f, " i2sstd=0x{:x}", self.i2sstd()))}
        if self.ckpol() != 0 { try!(write!(f, " ckpol"))}
        if self.datlen() != 0 { try!(write!(f, " datlen=0x{:x}", self.datlen()))}
        if self.chlen() != 0 { try!(write!(f, " chlen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="I2S prescaler register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct I2spr(pub u32);
impl I2spr {
    #[doc="Master clock output enable"]
    #[inline] pub fn mckoe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Master clock output enable"]
    #[inline] pub fn test_mckoe(&self) -> bool {
        self.mckoe != 0
    }

    #[doc="Master clock output enable"]
    #[inline] pub fn set_mckoe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Odd factor for the prescaler"]
    #[inline] pub fn odd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Odd factor for the prescaler"]
    #[inline] pub fn test_odd(&self) -> bool {
        self.odd != 0
    }

    #[doc="Odd factor for the prescaler"]
    #[inline] pub fn set_odd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2S Linear prescaler"]
    #[inline] pub fn i2sdiv(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="I2S Linear prescaler"]
    #[inline] pub fn test_i2sdiv(&self) -> bool {
        self.i2sdiv != 0
    }

    #[doc="I2S Linear prescaler"]
    #[inline] pub fn set_i2sdiv<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl ::core::fmt::Display for I2spr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for I2spr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mckoe() != 0 { try!(write!(f, " mckoe"))}
        if self.odd() != 0 { try!(write!(f, " odd"))}
        if self.i2sdiv() != 0 { try!(write!(f, " i2sdiv=0x{:x}", self.i2sdiv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}


