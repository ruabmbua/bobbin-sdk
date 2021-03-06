
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SPI_V2 Peripheral"]
pub struct SpiPeriph(pub usize); 

pub struct SpiCh { pub periph: SpiPeriph, pub index: usize }

impl SpiPeriph {
    #[doc="Get the CR1 Register."]
    #[inline] pub fn cr1_reg(&self) -> ::bobbin_mcu::register::Register<Cr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr1, 0x0)
    }

    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
        self.cr1_reg().ptr()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        self.cr1_reg().read()
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn write_cr1(&self, value: Cr1) -> &Self { 
        self.cr1_reg().write(value);
        self
    }

    #[doc="Set the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().set(f);
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        self.cr1_reg().with(f);
        self
    }

    #[doc="Get the CR2 Register."]
    #[inline] pub fn cr2_reg(&self) -> ::bobbin_mcu::register::Register<Cr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr2, 0x4)
    }

    #[doc="Get the *mut pointer for the CR2 register."]
    #[inline] pub fn cr2_mut(&self) -> *mut Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR2 register."]
    #[inline] pub fn cr2_ptr(&self) -> *const Cr2 { 
        self.cr2_reg().ptr()
    }

    #[doc="Read the CR2 register."]
    #[inline] pub fn cr2(&self) -> Cr2 { 
        self.cr2_reg().read()
    }

    #[doc="Write the CR2 register."]
    #[inline] pub fn write_cr2(&self, value: Cr2) -> &Self { 
        self.cr2_reg().write(value);
        self
    }

    #[doc="Set the CR2 register."]
    #[inline] pub fn set_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().set(f);
        self
    }

    #[doc="Modify the CR2 register."]
    #[inline] pub fn with_cr2<F: FnOnce(Cr2) -> Cr2>(&self, f: F) -> &Self {
        self.cr2_reg().with(f);
        self
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x8)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

    #[doc="Get the DR Register."]
    #[inline] pub fn dr_reg(&self) -> ::bobbin_mcu::register::Register<Dr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dr, 0xc)
    }

    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const Dr { 
        self.dr_reg().ptr()
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        self.dr_reg().read()
    }

    #[doc="Write the DR register."]
    #[inline] pub fn write_dr(&self, value: Dr) -> &Self { 
        self.dr_reg().write(value);
        self
    }

    #[doc="Set the DR register."]
    #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        self.dr_reg().set(f);
        self
    }

    #[doc="Modify the DR register."]
    #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        self.dr_reg().with(f);
        self
    }

    #[doc="Get the DR8 Register."]
    #[inline] pub fn dr8_reg(&self) -> ::bobbin_mcu::register::Register<Dr8> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dr8, 0xc)
    }

    #[doc="Get the *mut pointer for the DR8 register."]
    #[inline] pub fn dr8_mut(&self) -> *mut Dr8 { 
        self.dr8_reg().ptr()
    }

    #[doc="Get the *const pointer for the DR8 register."]
    #[inline] pub fn dr8_ptr(&self) -> *const Dr8 { 
        self.dr8_reg().ptr()
    }

    #[doc="Read the DR8 register."]
    #[inline] pub fn dr8(&self) -> Dr8 { 
        self.dr8_reg().read()
    }

    #[doc="Write the DR8 register."]
    #[inline] pub fn write_dr8(&self, value: Dr8) -> &Self { 
        self.dr8_reg().write(value);
        self
    }

    #[doc="Set the DR8 register."]
    #[inline] pub fn set_dr8<F: FnOnce(Dr8) -> Dr8>(&self, f: F) -> &Self {
        self.dr8_reg().set(f);
        self
    }

    #[doc="Modify the DR8 register."]
    #[inline] pub fn with_dr8<F: FnOnce(Dr8) -> Dr8>(&self, f: F) -> &Self {
        self.dr8_reg().with(f);
        self
    }

    #[doc="Get the CRCPR Register."]
    #[inline] pub fn crcpr_reg(&self) -> ::bobbin_mcu::register::Register<Crcpr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Crcpr, 0x10)
    }

    #[doc="Get the *mut pointer for the CRCPR register."]
    #[inline] pub fn crcpr_mut(&self) -> *mut Crcpr { 
        self.crcpr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CRCPR register."]
    #[inline] pub fn crcpr_ptr(&self) -> *const Crcpr { 
        self.crcpr_reg().ptr()
    }

    #[doc="Read the CRCPR register."]
    #[inline] pub fn crcpr(&self) -> Crcpr { 
        self.crcpr_reg().read()
    }

    #[doc="Write the CRCPR register."]
    #[inline] pub fn write_crcpr(&self, value: Crcpr) -> &Self { 
        self.crcpr_reg().write(value);
        self
    }

    #[doc="Set the CRCPR register."]
    #[inline] pub fn set_crcpr<F: FnOnce(Crcpr) -> Crcpr>(&self, f: F) -> &Self {
        self.crcpr_reg().set(f);
        self
    }

    #[doc="Modify the CRCPR register."]
    #[inline] pub fn with_crcpr<F: FnOnce(Crcpr) -> Crcpr>(&self, f: F) -> &Self {
        self.crcpr_reg().with(f);
        self
    }

    #[doc="Get the RXCRCR Register."]
    #[inline] pub fn rxcrcr_reg(&self) -> ::bobbin_mcu::register::Register<Rxcrcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rxcrcr, 0x14)
    }

    #[doc="Get the *mut pointer for the RXCRCR register."]
    #[inline] pub fn rxcrcr_mut(&self) -> *mut Rxcrcr { 
        self.rxcrcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RXCRCR register."]
    #[inline] pub fn rxcrcr_ptr(&self) -> *const Rxcrcr { 
        self.rxcrcr_reg().ptr()
    }

    #[doc="Read the RXCRCR register."]
    #[inline] pub fn rxcrcr(&self) -> Rxcrcr { 
        self.rxcrcr_reg().read()
    }

    #[doc="Get the TXCRCR Register."]
    #[inline] pub fn txcrcr_reg(&self) -> ::bobbin_mcu::register::Register<Txcrcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Txcrcr, 0x18)
    }

    #[doc="Get the *mut pointer for the TXCRCR register."]
    #[inline] pub fn txcrcr_mut(&self) -> *mut Txcrcr { 
        self.txcrcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TXCRCR register."]
    #[inline] pub fn txcrcr_ptr(&self) -> *const Txcrcr { 
        self.txcrcr_reg().ptr()
    }

    #[doc="Read the TXCRCR register."]
    #[inline] pub fn txcrcr(&self) -> Txcrcr { 
        self.txcrcr_reg().read()
    }

    #[doc="Get the I2SCFGR Register."]
    #[inline] pub fn i2scfgr_reg(&self) -> ::bobbin_mcu::register::Register<I2scfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut I2scfgr, 0x1c)
    }

    #[doc="Get the *mut pointer for the I2SCFGR register."]
    #[inline] pub fn i2scfgr_mut(&self) -> *mut I2scfgr { 
        self.i2scfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the I2SCFGR register."]
    #[inline] pub fn i2scfgr_ptr(&self) -> *const I2scfgr { 
        self.i2scfgr_reg().ptr()
    }

    #[doc="Read the I2SCFGR register."]
    #[inline] pub fn i2scfgr(&self) -> I2scfgr { 
        self.i2scfgr_reg().read()
    }

    #[doc="Write the I2SCFGR register."]
    #[inline] pub fn write_i2scfgr(&self, value: I2scfgr) -> &Self { 
        self.i2scfgr_reg().write(value);
        self
    }

    #[doc="Set the I2SCFGR register."]
    #[inline] pub fn set_i2scfgr<F: FnOnce(I2scfgr) -> I2scfgr>(&self, f: F) -> &Self {
        self.i2scfgr_reg().set(f);
        self
    }

    #[doc="Modify the I2SCFGR register."]
    #[inline] pub fn with_i2scfgr<F: FnOnce(I2scfgr) -> I2scfgr>(&self, f: F) -> &Self {
        self.i2scfgr_reg().with(f);
        self
    }

    #[doc="Get the I2SPR Register."]
    #[inline] pub fn i2spr_reg(&self) -> ::bobbin_mcu::register::Register<I2spr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut I2spr, 0x20)
    }

    #[doc="Get the *mut pointer for the I2SPR register."]
    #[inline] pub fn i2spr_mut(&self) -> *mut I2spr { 
        self.i2spr_reg().ptr()
    }

    #[doc="Get the *const pointer for the I2SPR register."]
    #[inline] pub fn i2spr_ptr(&self) -> *const I2spr { 
        self.i2spr_reg().ptr()
    }

    #[doc="Read the I2SPR register."]
    #[inline] pub fn i2spr(&self) -> I2spr { 
        self.i2spr_reg().read()
    }

    #[doc="Write the I2SPR register."]
    #[inline] pub fn write_i2spr(&self, value: I2spr) -> &Self { 
        self.i2spr_reg().write(value);
        self
    }

    #[doc="Set the I2SPR register."]
    #[inline] pub fn set_i2spr<F: FnOnce(I2spr) -> I2spr>(&self, f: F) -> &Self {
        self.i2spr_reg().set(f);
        self
    }

    #[doc="Modify the I2SPR register."]
    #[inline] pub fn with_i2spr<F: FnOnce(I2spr) -> I2spr>(&self, f: F) -> &Self {
        self.i2spr_reg().with(f);
        self
    }

}

#[doc="control register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="Bidirectional data mode enable"]
    #[inline] pub fn bidimode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if BIDIMODE != 0"]
    #[inline] pub fn test_bidimode(&self) -> bool {
        self.bidimode() != 0
    }

    #[doc="Sets the BIDIMODE field."]
    #[inline] pub fn set_bidimode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Output enable in bidirectional mode"]
    #[inline] pub fn bidioe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if BIDIOE != 0"]
    #[inline] pub fn test_bidioe(&self) -> bool {
        self.bidioe() != 0
    }

    #[doc="Sets the BIDIOE field."]
    #[inline] pub fn set_bidioe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Hardware CRC calculation enable"]
    #[inline] pub fn crcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CRCEN != 0"]
    #[inline] pub fn test_crcen(&self) -> bool {
        self.crcen() != 0
    }

    #[doc="Sets the CRCEN field."]
    #[inline] pub fn set_crcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="CRC transfer next"]
    #[inline] pub fn crcnext(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CRCNEXT != 0"]
    #[inline] pub fn test_crcnext(&self) -> bool {
        self.crcnext() != 0
    }

    #[doc="Sets the CRCNEXT field."]
    #[inline] pub fn set_crcnext<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="CRC Length"]
    #[inline] pub fn crcl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CRCL != 0"]
    #[inline] pub fn test_crcl(&self) -> bool {
        self.crcl() != 0
    }

    #[doc="Sets the CRCL field."]
    #[inline] pub fn set_crcl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Receive only"]
    #[inline] pub fn rxonly(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RXONLY != 0"]
    #[inline] pub fn test_rxonly(&self) -> bool {
        self.rxonly() != 0
    }

    #[doc="Sets the RXONLY field."]
    #[inline] pub fn set_rxonly<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Software slave management"]
    #[inline] pub fn ssm(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SSM != 0"]
    #[inline] pub fn test_ssm(&self) -> bool {
        self.ssm() != 0
    }

    #[doc="Sets the SSM field."]
    #[inline] pub fn set_ssm<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Internal slave select"]
    #[inline] pub fn ssi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SSI != 0"]
    #[inline] pub fn test_ssi(&self) -> bool {
        self.ssi() != 0
    }

    #[doc="Sets the SSI field."]
    #[inline] pub fn set_ssi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Frame format"]
    #[inline] pub fn lsbfirst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LSBFIRST != 0"]
    #[inline] pub fn test_lsbfirst(&self) -> bool {
        self.lsbfirst() != 0
    }

    #[doc="Sets the LSBFIRST field."]
    #[inline] pub fn set_lsbfirst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SPI enable"]
    #[inline] pub fn spe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SPE != 0"]
    #[inline] pub fn test_spe(&self) -> bool {
        self.spe() != 0
    }

    #[doc="Sets the SPE field."]
    #[inline] pub fn set_spe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Baud rate control"]
    #[inline] pub fn br(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if BR != 0"]
    #[inline] pub fn test_br(&self) -> bool {
        self.br() != 0
    }

    #[doc="Sets the BR field."]
    #[inline] pub fn set_br<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Master selection"]
    #[inline] pub fn mstr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MSTR != 0"]
    #[inline] pub fn test_mstr(&self) -> bool {
        self.mstr() != 0
    }

    #[doc="Sets the MSTR field."]
    #[inline] pub fn set_mstr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clock polarity"]
    #[inline] pub fn cpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock phase"]
    #[inline] pub fn cpha(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr1(other)
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
        if self.crcl() != 0 { try!(write!(f, " crcl"))}
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
    #[inline] pub fn rxdmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXDMAEN != 0"]
    #[inline] pub fn test_rxdmaen(&self) -> bool {
        self.rxdmaen() != 0
    }

    #[doc="Sets the RXDMAEN field."]
    #[inline] pub fn set_rxdmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Tx buffer DMA enable"]
    #[inline] pub fn txdmaen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXDMAEN != 0"]
    #[inline] pub fn test_txdmaen(&self) -> bool {
        self.txdmaen() != 0
    }

    #[doc="Sets the TXDMAEN field."]
    #[inline] pub fn set_txdmaen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SS output enable"]
    #[inline] pub fn ssoe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SSOE != 0"]
    #[inline] pub fn test_ssoe(&self) -> bool {
        self.ssoe() != 0
    }

    #[doc="Sets the SSOE field."]
    #[inline] pub fn set_ssoe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="NSS pulse management"]
    #[inline] pub fn nssp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if NSSP != 0"]
    #[inline] pub fn test_nssp(&self) -> bool {
        self.nssp() != 0
    }

    #[doc="Sets the NSSP field."]
    #[inline] pub fn set_nssp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Frame format"]
    #[inline] pub fn frf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FRF != 0"]
    #[inline] pub fn test_frf(&self) -> bool {
        self.frf() != 0
    }

    #[doc="Sets the FRF field."]
    #[inline] pub fn set_frf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Error interrupt enable"]
    #[inline] pub fn errie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RX buffer not empty interrupt enable"]
    #[inline] pub fn rxneie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXNEIE != 0"]
    #[inline] pub fn test_rxneie(&self) -> bool {
        self.rxneie() != 0
    }

    #[doc="Sets the RXNEIE field."]
    #[inline] pub fn set_rxneie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Tx buffer empty interrupt enable"]
    #[inline] pub fn txeie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXEIE != 0"]
    #[inline] pub fn test_txeie(&self) -> bool {
        self.txeie() != 0
    }

    #[doc="Sets the TXEIE field."]
    #[inline] pub fn set_txeie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Data size"]
    #[inline] pub fn ds(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if DS != 0"]
    #[inline] pub fn test_ds(&self) -> bool {
        self.ds() != 0
    }

    #[doc="Sets the DS field."]
    #[inline] pub fn set_ds<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FIFO reception threshold"]
    #[inline] pub fn frxth(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FRXTH != 0"]
    #[inline] pub fn test_frxth(&self) -> bool {
        self.frxth() != 0
    }

    #[doc="Sets the FRXTH field."]
    #[inline] pub fn set_frxth<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Last DMA transfer for reception"]
    #[inline] pub fn ldma_rx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if LDMA_RX != 0"]
    #[inline] pub fn test_ldma_rx(&self) -> bool {
        self.ldma_rx() != 0
    }

    #[doc="Sets the LDMA_RX field."]
    #[inline] pub fn set_ldma_rx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Last DMA transfer for transmission"]
    #[inline] pub fn ldma_tx(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if LDMA_TX != 0"]
    #[inline] pub fn test_ldma_tx(&self) -> bool {
        self.ldma_tx() != 0
    }

    #[doc="Sets the LDMA_TX field."]
    #[inline] pub fn set_ldma_tx<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Cr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr2(other)
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
    #[inline] pub fn rxne(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXNE != 0"]
    #[inline] pub fn test_rxne(&self) -> bool {
        self.rxne() != 0
    }

    #[doc="Sets the RXNE field."]
    #[inline] pub fn set_rxne<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit buffer empty"]
    #[inline] pub fn txe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
    #[inline] pub fn set_txe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel side"]
    #[inline] pub fn chside(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CHSIDE != 0"]
    #[inline] pub fn test_chside(&self) -> bool {
        self.chside() != 0
    }

    #[doc="Sets the CHSIDE field."]
    #[inline] pub fn set_chside<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Underrun flag"]
    #[inline] pub fn udr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if UDR != 0"]
    #[inline] pub fn test_udr(&self) -> bool {
        self.udr() != 0
    }

    #[doc="Sets the UDR field."]
    #[inline] pub fn set_udr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="CRC error flag"]
    #[inline] pub fn crcerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CRCERR != 0"]
    #[inline] pub fn test_crcerr(&self) -> bool {
        self.crcerr() != 0
    }

    #[doc="Sets the CRCERR field."]
    #[inline] pub fn set_crcerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Mode fault"]
    #[inline] pub fn modf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MODF != 0"]
    #[inline] pub fn test_modf(&self) -> bool {
        self.modf() != 0
    }

    #[doc="Sets the MODF field."]
    #[inline] pub fn set_modf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Overrun flag"]
    #[inline] pub fn ovr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if OVR != 0"]
    #[inline] pub fn test_ovr(&self) -> bool {
        self.ovr() != 0
    }

    #[doc="Sets the OVR field."]
    #[inline] pub fn set_ovr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Busy flag"]
    #[inline] pub fn bsy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BSY != 0"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Sets the BSY field."]
    #[inline] pub fn set_bsy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Frame format error"]
    #[inline] pub fn fre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FRE != 0"]
    #[inline] pub fn test_fre(&self) -> bool {
        self.fre() != 0
    }

    #[doc="Sets the FRE field."]
    #[inline] pub fn set_fre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FIFO reception level"]
    #[inline] pub fn frlvl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x3) as u8) } // [10:9]
    }

    #[doc="Returns true if FRLVL != 0"]
    #[inline] pub fn test_frlvl(&self) -> bool {
        self.frlvl() != 0
    }

    #[doc="Sets the FRLVL field."]
    #[inline] pub fn set_frlvl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="FIFO transmission level"]
    #[inline] pub fn ftlvl(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x3) as u8) } // [12:11]
    }

    #[doc="Returns true if FTLVL != 0"]
    #[inline] pub fn test_ftlvl(&self) -> bool {
        self.ftlvl() != 0
    }

    #[doc="Sets the FTLVL field."]
    #[inline] pub fn set_ftlvl<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 11);
        self.0 |= value << 11;
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
        if self.rxne() != 0 { try!(write!(f, " rxne"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        if self.chside() != 0 { try!(write!(f, " chside"))}
        if self.udr() != 0 { try!(write!(f, " udr"))}
        if self.crcerr() != 0 { try!(write!(f, " crcerr"))}
        if self.modf() != 0 { try!(write!(f, " modf"))}
        if self.ovr() != 0 { try!(write!(f, " ovr"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        if self.fre() != 0 { try!(write!(f, " fre"))}
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
    #[inline] pub fn dr(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DR != 0"]
    #[inline] pub fn test_dr(&self) -> bool {
        self.dr() != 0
    }

    #[doc="Sets the DR field."]
    #[inline] pub fn set_dr<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Dr {
    #[inline]
    fn from(other: u16) -> Self {
         Dr(other)
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
    #[inline] pub fn dr8(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DR8 != 0"]
    #[inline] pub fn test_dr8(&self) -> bool {
        self.dr8() != 0
    }

    #[doc="Sets the DR8 field."]
    #[inline] pub fn set_dr8<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Dr8 {
    #[inline]
    fn from(other: u8) -> Self {
         Dr8(other)
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
    #[inline] pub fn crcpoly(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CRCPOLY != 0"]
    #[inline] pub fn test_crcpoly(&self) -> bool {
        self.crcpoly() != 0
    }

    #[doc="Sets the CRCPOLY field."]
    #[inline] pub fn set_crcpoly<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Crcpr {
    #[inline]
    fn from(other: u32) -> Self {
         Crcpr(other)
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
    #[inline] pub fn rxcrc(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if RxCRC != 0"]
    #[inline] pub fn test_rxcrc(&self) -> bool {
        self.rxcrc() != 0
    }

    #[doc="Sets the RxCRC field."]
    #[inline] pub fn set_rxcrc<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rxcrcr {
    #[inline]
    fn from(other: u32) -> Self {
         Rxcrcr(other)
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
    #[inline] pub fn txcrc(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if TxCRC != 0"]
    #[inline] pub fn test_txcrc(&self) -> bool {
        self.txcrc() != 0
    }

    #[doc="Sets the TxCRC field."]
    #[inline] pub fn set_txcrc<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Txcrcr {
    #[inline]
    fn from(other: u32) -> Self {
         Txcrcr(other)
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
    #[inline] pub fn i2smod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if I2SMOD != 0"]
    #[inline] pub fn test_i2smod(&self) -> bool {
        self.i2smod() != 0
    }

    #[doc="Sets the I2SMOD field."]
    #[inline] pub fn set_i2smod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="I2S Enable"]
    #[inline] pub fn i2se(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if I2SE != 0"]
    #[inline] pub fn test_i2se(&self) -> bool {
        self.i2se() != 0
    }

    #[doc="Sets the I2SE field."]
    #[inline] pub fn set_i2se<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="I2S configuration mode"]
    #[inline] pub fn i2scfg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if I2SCFG != 0"]
    #[inline] pub fn test_i2scfg(&self) -> bool {
        self.i2scfg() != 0
    }

    #[doc="Sets the I2SCFG field."]
    #[inline] pub fn set_i2scfg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="PCM frame synchronization"]
    #[inline] pub fn pcmsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PCMSYNC != 0"]
    #[inline] pub fn test_pcmsync(&self) -> bool {
        self.pcmsync() != 0
    }

    #[doc="Sets the PCMSYNC field."]
    #[inline] pub fn set_pcmsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="I2S standard selection"]
    #[inline] pub fn i2sstd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if I2SSTD != 0"]
    #[inline] pub fn test_i2sstd(&self) -> bool {
        self.i2sstd() != 0
    }

    #[doc="Sets the I2SSTD field."]
    #[inline] pub fn set_i2sstd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Steady state clock polarity"]
    #[inline] pub fn ckpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CKPOL != 0"]
    #[inline] pub fn test_ckpol(&self) -> bool {
        self.ckpol() != 0
    }

    #[doc="Sets the CKPOL field."]
    #[inline] pub fn set_ckpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Data length to be transferred"]
    #[inline] pub fn datlen(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if DATLEN != 0"]
    #[inline] pub fn test_datlen(&self) -> bool {
        self.datlen() != 0
    }

    #[doc="Sets the DATLEN field."]
    #[inline] pub fn set_datlen<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel length (number of bits per audio channel)"]
    #[inline] pub fn chlen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHLEN != 0"]
    #[inline] pub fn test_chlen(&self) -> bool {
        self.chlen() != 0
    }

    #[doc="Sets the CHLEN field."]
    #[inline] pub fn set_chlen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for I2scfgr {
    #[inline]
    fn from(other: u32) -> Self {
         I2scfgr(other)
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
    #[inline] pub fn mckoe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if MCKOE != 0"]
    #[inline] pub fn test_mckoe(&self) -> bool {
        self.mckoe() != 0
    }

    #[doc="Sets the MCKOE field."]
    #[inline] pub fn set_mckoe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Odd factor for the prescaler"]
    #[inline] pub fn odd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ODD != 0"]
    #[inline] pub fn test_odd(&self) -> bool {
        self.odd() != 0
    }

    #[doc="Sets the ODD field."]
    #[inline] pub fn set_odd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="I2S Linear prescaler"]
    #[inline] pub fn i2sdiv(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if I2SDIV != 0"]
    #[inline] pub fn test_i2sdiv(&self) -> bool {
        self.i2sdiv() != 0
    }

    #[doc="Sets the I2SDIV field."]
    #[inline] pub fn set_i2sdiv<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for I2spr {
    #[inline]
    fn from(other: u32) -> Self {
         I2spr(other)
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

