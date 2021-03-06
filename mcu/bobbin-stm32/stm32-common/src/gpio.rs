
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 

pub struct GpioPin { pub port: GpioPeriph, pub index: usize }

impl GpioPeriph {
    #[doc="Get the MODER Register."]
    #[inline] pub fn moder_reg(&self) -> ::bobbin_mcu::register::Register<Moder> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Moder, 0x0)
    }

    #[doc="Get the *mut pointer for the MODER register."]
    #[inline] pub fn moder_mut(&self) -> *mut Moder { 
        self.moder_reg().ptr()
    }

    #[doc="Get the *const pointer for the MODER register."]
    #[inline] pub fn moder_ptr(&self) -> *const Moder { 
        self.moder_reg().ptr()
    }

    #[doc="Read the MODER register."]
    #[inline] pub fn moder(&self) -> Moder { 
        self.moder_reg().read()
    }

    #[doc="Write the MODER register."]
    #[inline] pub fn write_moder(&self, value: Moder) -> &Self { 
        self.moder_reg().write(value);
        self
    }

    #[doc="Set the MODER register."]
    #[inline] pub fn set_moder<F: FnOnce(Moder) -> Moder>(&self, f: F) -> &Self {
        self.moder_reg().set(f);
        self
    }

    #[doc="Modify the MODER register."]
    #[inline] pub fn with_moder<F: FnOnce(Moder) -> Moder>(&self, f: F) -> &Self {
        self.moder_reg().with(f);
        self
    }

    #[doc="Get the OTYPER Register."]
    #[inline] pub fn otyper_reg(&self) -> ::bobbin_mcu::register::Register<Otyper> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Otyper, 0x4)
    }

    #[doc="Get the *mut pointer for the OTYPER register."]
    #[inline] pub fn otyper_mut(&self) -> *mut Otyper { 
        self.otyper_reg().ptr()
    }

    #[doc="Get the *const pointer for the OTYPER register."]
    #[inline] pub fn otyper_ptr(&self) -> *const Otyper { 
        self.otyper_reg().ptr()
    }

    #[doc="Read the OTYPER register."]
    #[inline] pub fn otyper(&self) -> Otyper { 
        self.otyper_reg().read()
    }

    #[doc="Write the OTYPER register."]
    #[inline] pub fn write_otyper(&self, value: Otyper) -> &Self { 
        self.otyper_reg().write(value);
        self
    }

    #[doc="Set the OTYPER register."]
    #[inline] pub fn set_otyper<F: FnOnce(Otyper) -> Otyper>(&self, f: F) -> &Self {
        self.otyper_reg().set(f);
        self
    }

    #[doc="Modify the OTYPER register."]
    #[inline] pub fn with_otyper<F: FnOnce(Otyper) -> Otyper>(&self, f: F) -> &Self {
        self.otyper_reg().with(f);
        self
    }

    #[doc="Get the OSPEEDR Register."]
    #[inline] pub fn ospeedr_reg(&self) -> ::bobbin_mcu::register::Register<Ospeedr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ospeedr, 0x8)
    }

    #[doc="Get the *mut pointer for the OSPEEDR register."]
    #[inline] pub fn ospeedr_mut(&self) -> *mut Ospeedr { 
        self.ospeedr_reg().ptr()
    }

    #[doc="Get the *const pointer for the OSPEEDR register."]
    #[inline] pub fn ospeedr_ptr(&self) -> *const Ospeedr { 
        self.ospeedr_reg().ptr()
    }

    #[doc="Read the OSPEEDR register."]
    #[inline] pub fn ospeedr(&self) -> Ospeedr { 
        self.ospeedr_reg().read()
    }

    #[doc="Write the OSPEEDR register."]
    #[inline] pub fn write_ospeedr(&self, value: Ospeedr) -> &Self { 
        self.ospeedr_reg().write(value);
        self
    }

    #[doc="Set the OSPEEDR register."]
    #[inline] pub fn set_ospeedr<F: FnOnce(Ospeedr) -> Ospeedr>(&self, f: F) -> &Self {
        self.ospeedr_reg().set(f);
        self
    }

    #[doc="Modify the OSPEEDR register."]
    #[inline] pub fn with_ospeedr<F: FnOnce(Ospeedr) -> Ospeedr>(&self, f: F) -> &Self {
        self.ospeedr_reg().with(f);
        self
    }

    #[doc="Get the PUPDR Register."]
    #[inline] pub fn pupdr_reg(&self) -> ::bobbin_mcu::register::Register<Pupdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pupdr, 0xc)
    }

    #[doc="Get the *mut pointer for the PUPDR register."]
    #[inline] pub fn pupdr_mut(&self) -> *mut Pupdr { 
        self.pupdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUPDR register."]
    #[inline] pub fn pupdr_ptr(&self) -> *const Pupdr { 
        self.pupdr_reg().ptr()
    }

    #[doc="Read the PUPDR register."]
    #[inline] pub fn pupdr(&self) -> Pupdr { 
        self.pupdr_reg().read()
    }

    #[doc="Write the PUPDR register."]
    #[inline] pub fn write_pupdr(&self, value: Pupdr) -> &Self { 
        self.pupdr_reg().write(value);
        self
    }

    #[doc="Set the PUPDR register."]
    #[inline] pub fn set_pupdr<F: FnOnce(Pupdr) -> Pupdr>(&self, f: F) -> &Self {
        self.pupdr_reg().set(f);
        self
    }

    #[doc="Modify the PUPDR register."]
    #[inline] pub fn with_pupdr<F: FnOnce(Pupdr) -> Pupdr>(&self, f: F) -> &Self {
        self.pupdr_reg().with(f);
        self
    }

    #[doc="Get the IDR Register."]
    #[inline] pub fn idr_reg(&self) -> ::bobbin_mcu::register::Register<Idr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Idr, 0x10)
    }

    #[doc="Get the *mut pointer for the IDR register."]
    #[inline] pub fn idr_mut(&self) -> *mut Idr { 
        self.idr_reg().ptr()
    }

    #[doc="Get the *const pointer for the IDR register."]
    #[inline] pub fn idr_ptr(&self) -> *const Idr { 
        self.idr_reg().ptr()
    }

    #[doc="Read the IDR register."]
    #[inline] pub fn idr(&self) -> Idr { 
        self.idr_reg().read()
    }

    #[doc="Get the ODR Register."]
    #[inline] pub fn odr_reg(&self) -> ::bobbin_mcu::register::Register<Odr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Odr, 0x14)
    }

    #[doc="Get the *mut pointer for the ODR register."]
    #[inline] pub fn odr_mut(&self) -> *mut Odr { 
        self.odr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ODR register."]
    #[inline] pub fn odr_ptr(&self) -> *const Odr { 
        self.odr_reg().ptr()
    }

    #[doc="Read the ODR register."]
    #[inline] pub fn odr(&self) -> Odr { 
        self.odr_reg().read()
    }

    #[doc="Write the ODR register."]
    #[inline] pub fn write_odr(&self, value: Odr) -> &Self { 
        self.odr_reg().write(value);
        self
    }

    #[doc="Set the ODR register."]
    #[inline] pub fn set_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
        self.odr_reg().set(f);
        self
    }

    #[doc="Modify the ODR register."]
    #[inline] pub fn with_odr<F: FnOnce(Odr) -> Odr>(&self, f: F) -> &Self {
        self.odr_reg().with(f);
        self
    }

    #[doc="Get the BSRR Register."]
    #[inline] pub fn bsrr_reg(&self) -> ::bobbin_mcu::register::Register<Bsrr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bsrr, 0x18)
    }

    #[doc="Get the *mut pointer for the BSRR register."]
    #[inline] pub fn bsrr_mut(&self) -> *mut Bsrr { 
        self.bsrr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BSRR register."]
    #[inline] pub fn bsrr_ptr(&self) -> *const Bsrr { 
        self.bsrr_reg().ptr()
    }

    #[doc="Write the BSRR register."]
    #[inline] pub fn write_bsrr(&self, value: Bsrr) -> &Self { 
        self.bsrr_reg().write(value);
        self
    }

    #[doc="Set the BSRR register."]
    #[inline] pub fn set_bsrr<F: FnOnce(Bsrr) -> Bsrr>(&self, f: F) -> &Self {
        self.bsrr_reg().set(f);
        self
    }

    #[doc="Get the LCKR Register."]
    #[inline] pub fn lckr_reg(&self) -> ::bobbin_mcu::register::Register<Lckr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Lckr, 0x1c)
    }

    #[doc="Get the *mut pointer for the LCKR register."]
    #[inline] pub fn lckr_mut(&self) -> *mut Lckr { 
        self.lckr_reg().ptr()
    }

    #[doc="Get the *const pointer for the LCKR register."]
    #[inline] pub fn lckr_ptr(&self) -> *const Lckr { 
        self.lckr_reg().ptr()
    }

    #[doc="Read the LCKR register."]
    #[inline] pub fn lckr(&self) -> Lckr { 
        self.lckr_reg().read()
    }

    #[doc="Write the LCKR register."]
    #[inline] pub fn write_lckr(&self, value: Lckr) -> &Self { 
        self.lckr_reg().write(value);
        self
    }

    #[doc="Set the LCKR register."]
    #[inline] pub fn set_lckr<F: FnOnce(Lckr) -> Lckr>(&self, f: F) -> &Self {
        self.lckr_reg().set(f);
        self
    }

    #[doc="Modify the LCKR register."]
    #[inline] pub fn with_lckr<F: FnOnce(Lckr) -> Lckr>(&self, f: F) -> &Self {
        self.lckr_reg().with(f);
        self
    }

    #[doc="Get the AFRL Register."]
    #[inline] pub fn afrl_reg(&self) -> ::bobbin_mcu::register::Register<Afrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Afrl, 0x20)
    }

    #[doc="Get the *mut pointer for the AFRL register."]
    #[inline] pub fn afrl_mut(&self) -> *mut Afrl { 
        self.afrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the AFRL register."]
    #[inline] pub fn afrl_ptr(&self) -> *const Afrl { 
        self.afrl_reg().ptr()
    }

    #[doc="Read the AFRL register."]
    #[inline] pub fn afrl(&self) -> Afrl { 
        self.afrl_reg().read()
    }

    #[doc="Write the AFRL register."]
    #[inline] pub fn write_afrl(&self, value: Afrl) -> &Self { 
        self.afrl_reg().write(value);
        self
    }

    #[doc="Set the AFRL register."]
    #[inline] pub fn set_afrl<F: FnOnce(Afrl) -> Afrl>(&self, f: F) -> &Self {
        self.afrl_reg().set(f);
        self
    }

    #[doc="Modify the AFRL register."]
    #[inline] pub fn with_afrl<F: FnOnce(Afrl) -> Afrl>(&self, f: F) -> &Self {
        self.afrl_reg().with(f);
        self
    }

    #[doc="Get the AFRH Register."]
    #[inline] pub fn afrh_reg(&self) -> ::bobbin_mcu::register::Register<Afrh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Afrh, 0x24)
    }

    #[doc="Get the *mut pointer for the AFRH register."]
    #[inline] pub fn afrh_mut(&self) -> *mut Afrh { 
        self.afrh_reg().ptr()
    }

    #[doc="Get the *const pointer for the AFRH register."]
    #[inline] pub fn afrh_ptr(&self) -> *const Afrh { 
        self.afrh_reg().ptr()
    }

    #[doc="Read the AFRH register."]
    #[inline] pub fn afrh(&self) -> Afrh { 
        self.afrh_reg().read()
    }

    #[doc="Write the AFRH register."]
    #[inline] pub fn write_afrh(&self, value: Afrh) -> &Self { 
        self.afrh_reg().write(value);
        self
    }

    #[doc="Set the AFRH register."]
    #[inline] pub fn set_afrh<F: FnOnce(Afrh) -> Afrh>(&self, f: F) -> &Self {
        self.afrh_reg().set(f);
        self
    }

    #[doc="Modify the AFRH register."]
    #[inline] pub fn with_afrh<F: FnOnce(Afrh) -> Afrh>(&self, f: F) -> &Self {
        self.afrh_reg().with(f);
        self
    }

    #[doc="Get the BRR Register."]
    #[inline] pub fn brr_reg(&self) -> ::bobbin_mcu::register::Register<Brr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Brr, 0x28)
    }

    #[doc="Get the *mut pointer for the BRR register."]
    #[inline] pub fn brr_mut(&self) -> *mut Brr { 
        self.brr_reg().ptr()
    }

    #[doc="Get the *const pointer for the BRR register."]
    #[inline] pub fn brr_ptr(&self) -> *const Brr { 
        self.brr_reg().ptr()
    }

    #[doc="Write the BRR register."]
    #[inline] pub fn write_brr(&self, value: Brr) -> &Self { 
        self.brr_reg().write(value);
        self
    }

    #[doc="Set the BRR register."]
    #[inline] pub fn set_brr<F: FnOnce(Brr) -> Brr>(&self, f: F) -> &Self {
        self.brr_reg().set(f);
        self
    }

}

#[doc="GPIO port mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Moder(pub u32);
impl Moder {
    #[doc="Port x configuration bits (y = 0..15)"]
    #[inline] pub fn moder<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 1);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MODER != 0"]
    #[inline] pub fn test_moder<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.moder(index) != 0
    }

    #[doc="Sets the MODER field."]
    #[inline] pub fn set_moder<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 1);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Moder {
    #[inline]
    fn from(other: u32) -> Self {
         Moder(other)
    }
}

impl ::core::fmt::Display for Moder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Moder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.moder(0) != 0 { try!(write!(f, " moder[0]=0x{:x}", self.moder(0)))}
        if self.moder(1) != 0 { try!(write!(f, " moder[1]=0x{:x}", self.moder(1)))}
        if self.moder(2) != 0 { try!(write!(f, " moder[2]=0x{:x}", self.moder(2)))}
        if self.moder(3) != 0 { try!(write!(f, " moder[3]=0x{:x}", self.moder(3)))}
        if self.moder(4) != 0 { try!(write!(f, " moder[4]=0x{:x}", self.moder(4)))}
        if self.moder(5) != 0 { try!(write!(f, " moder[5]=0x{:x}", self.moder(5)))}
        if self.moder(6) != 0 { try!(write!(f, " moder[6]=0x{:x}", self.moder(6)))}
        if self.moder(7) != 0 { try!(write!(f, " moder[7]=0x{:x}", self.moder(7)))}
        if self.moder(8) != 0 { try!(write!(f, " moder[8]=0x{:x}", self.moder(8)))}
        if self.moder(9) != 0 { try!(write!(f, " moder[9]=0x{:x}", self.moder(9)))}
        if self.moder(10) != 0 { try!(write!(f, " moder[10]=0x{:x}", self.moder(10)))}
        if self.moder(11) != 0 { try!(write!(f, " moder[11]=0x{:x}", self.moder(11)))}
        if self.moder(12) != 0 { try!(write!(f, " moder[12]=0x{:x}", self.moder(12)))}
        if self.moder(13) != 0 { try!(write!(f, " moder[13]=0x{:x}", self.moder(13)))}
        if self.moder(14) != 0 { try!(write!(f, " moder[14]=0x{:x}", self.moder(14)))}
        if self.moder(15) != 0 { try!(write!(f, " moder[15]=0x{:x}", self.moder(15)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO port output type register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Otyper(pub u32);
impl Otyper {
    #[doc="Port x configuration bits (y = 0..15)"]
    #[inline] pub fn ot<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OT != 0"]
    #[inline] pub fn test_ot<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.ot(index) != 0
    }

    #[doc="Sets the OT field."]
    #[inline] pub fn set_ot<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Otyper {
    #[inline]
    fn from(other: u32) -> Self {
         Otyper(other)
    }
}

impl ::core::fmt::Display for Otyper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Otyper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ot(0) != 0 { try!(write!(f, " ot[0]"))}
        if self.ot(1) != 0 { try!(write!(f, " ot[1]"))}
        if self.ot(2) != 0 { try!(write!(f, " ot[2]"))}
        if self.ot(3) != 0 { try!(write!(f, " ot[3]"))}
        if self.ot(4) != 0 { try!(write!(f, " ot[4]"))}
        if self.ot(5) != 0 { try!(write!(f, " ot[5]"))}
        if self.ot(6) != 0 { try!(write!(f, " ot[6]"))}
        if self.ot(7) != 0 { try!(write!(f, " ot[7]"))}
        if self.ot(8) != 0 { try!(write!(f, " ot[8]"))}
        if self.ot(9) != 0 { try!(write!(f, " ot[9]"))}
        if self.ot(10) != 0 { try!(write!(f, " ot[10]"))}
        if self.ot(11) != 0 { try!(write!(f, " ot[11]"))}
        if self.ot(12) != 0 { try!(write!(f, " ot[12]"))}
        if self.ot(13) != 0 { try!(write!(f, " ot[13]"))}
        if self.ot(14) != 0 { try!(write!(f, " ot[14]"))}
        if self.ot(15) != 0 { try!(write!(f, " ot[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO port output speed register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ospeedr(pub u32);
impl Ospeedr {
    #[doc="Port x configuration bits (y = 0..15)"]
    #[inline] pub fn ospeedr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 1);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if OSPEEDR != 0"]
    #[inline] pub fn test_ospeedr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.ospeedr(index) != 0
    }

    #[doc="Sets the OSPEEDR field."]
    #[inline] pub fn set_ospeedr<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 1);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ospeedr {
    #[inline]
    fn from(other: u32) -> Self {
         Ospeedr(other)
    }
}

impl ::core::fmt::Display for Ospeedr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ospeedr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ospeedr(0) != 0 { try!(write!(f, " ospeedr[0]=0x{:x}", self.ospeedr(0)))}
        if self.ospeedr(1) != 0 { try!(write!(f, " ospeedr[1]=0x{:x}", self.ospeedr(1)))}
        if self.ospeedr(2) != 0 { try!(write!(f, " ospeedr[2]=0x{:x}", self.ospeedr(2)))}
        if self.ospeedr(3) != 0 { try!(write!(f, " ospeedr[3]=0x{:x}", self.ospeedr(3)))}
        if self.ospeedr(4) != 0 { try!(write!(f, " ospeedr[4]=0x{:x}", self.ospeedr(4)))}
        if self.ospeedr(5) != 0 { try!(write!(f, " ospeedr[5]=0x{:x}", self.ospeedr(5)))}
        if self.ospeedr(6) != 0 { try!(write!(f, " ospeedr[6]=0x{:x}", self.ospeedr(6)))}
        if self.ospeedr(7) != 0 { try!(write!(f, " ospeedr[7]=0x{:x}", self.ospeedr(7)))}
        if self.ospeedr(8) != 0 { try!(write!(f, " ospeedr[8]=0x{:x}", self.ospeedr(8)))}
        if self.ospeedr(9) != 0 { try!(write!(f, " ospeedr[9]=0x{:x}", self.ospeedr(9)))}
        if self.ospeedr(10) != 0 { try!(write!(f, " ospeedr[10]=0x{:x}", self.ospeedr(10)))}
        if self.ospeedr(11) != 0 { try!(write!(f, " ospeedr[11]=0x{:x}", self.ospeedr(11)))}
        if self.ospeedr(12) != 0 { try!(write!(f, " ospeedr[12]=0x{:x}", self.ospeedr(12)))}
        if self.ospeedr(13) != 0 { try!(write!(f, " ospeedr[13]=0x{:x}", self.ospeedr(13)))}
        if self.ospeedr(14) != 0 { try!(write!(f, " ospeedr[14]=0x{:x}", self.ospeedr(14)))}
        if self.ospeedr(15) != 0 { try!(write!(f, " ospeedr[15]=0x{:x}", self.ospeedr(15)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO port pull-up/pull-down register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pupdr(pub u32);
impl Pupdr {
    #[doc="Port x configuration bits (y = 0..15)"]
    #[inline] pub fn pupdr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 1);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if PUPDR != 0"]
    #[inline] pub fn test_pupdr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.pupdr(index) != 0
    }

    #[doc="Sets the PUPDR field."]
    #[inline] pub fn set_pupdr<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 1);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pupdr {
    #[inline]
    fn from(other: u32) -> Self {
         Pupdr(other)
    }
}

impl ::core::fmt::Display for Pupdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pupdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pupdr(0) != 0 { try!(write!(f, " pupdr[0]=0x{:x}", self.pupdr(0)))}
        if self.pupdr(1) != 0 { try!(write!(f, " pupdr[1]=0x{:x}", self.pupdr(1)))}
        if self.pupdr(2) != 0 { try!(write!(f, " pupdr[2]=0x{:x}", self.pupdr(2)))}
        if self.pupdr(3) != 0 { try!(write!(f, " pupdr[3]=0x{:x}", self.pupdr(3)))}
        if self.pupdr(4) != 0 { try!(write!(f, " pupdr[4]=0x{:x}", self.pupdr(4)))}
        if self.pupdr(5) != 0 { try!(write!(f, " pupdr[5]=0x{:x}", self.pupdr(5)))}
        if self.pupdr(6) != 0 { try!(write!(f, " pupdr[6]=0x{:x}", self.pupdr(6)))}
        if self.pupdr(7) != 0 { try!(write!(f, " pupdr[7]=0x{:x}", self.pupdr(7)))}
        if self.pupdr(8) != 0 { try!(write!(f, " pupdr[8]=0x{:x}", self.pupdr(8)))}
        if self.pupdr(9) != 0 { try!(write!(f, " pupdr[9]=0x{:x}", self.pupdr(9)))}
        if self.pupdr(10) != 0 { try!(write!(f, " pupdr[10]=0x{:x}", self.pupdr(10)))}
        if self.pupdr(11) != 0 { try!(write!(f, " pupdr[11]=0x{:x}", self.pupdr(11)))}
        if self.pupdr(12) != 0 { try!(write!(f, " pupdr[12]=0x{:x}", self.pupdr(12)))}
        if self.pupdr(13) != 0 { try!(write!(f, " pupdr[13]=0x{:x}", self.pupdr(13)))}
        if self.pupdr(14) != 0 { try!(write!(f, " pupdr[14]=0x{:x}", self.pupdr(14)))}
        if self.pupdr(15) != 0 { try!(write!(f, " pupdr[15]=0x{:x}", self.pupdr(15)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO port input data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Idr(pub u32);
impl Idr {
    #[doc="Port input data (y = 0..15)"]
    #[inline] pub fn idr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IDR != 0"]
    #[inline] pub fn test_idr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.idr(index) != 0
    }

    #[doc="Sets the IDR field."]
    #[inline] pub fn set_idr<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Idr {
    #[inline]
    fn from(other: u32) -> Self {
         Idr(other)
    }
}

impl ::core::fmt::Display for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Idr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.idr(0) != 0 { try!(write!(f, " idr[0]"))}
        if self.idr(1) != 0 { try!(write!(f, " idr[1]"))}
        if self.idr(2) != 0 { try!(write!(f, " idr[2]"))}
        if self.idr(3) != 0 { try!(write!(f, " idr[3]"))}
        if self.idr(4) != 0 { try!(write!(f, " idr[4]"))}
        if self.idr(5) != 0 { try!(write!(f, " idr[5]"))}
        if self.idr(6) != 0 { try!(write!(f, " idr[6]"))}
        if self.idr(7) != 0 { try!(write!(f, " idr[7]"))}
        if self.idr(8) != 0 { try!(write!(f, " idr[8]"))}
        if self.idr(9) != 0 { try!(write!(f, " idr[9]"))}
        if self.idr(10) != 0 { try!(write!(f, " idr[10]"))}
        if self.idr(11) != 0 { try!(write!(f, " idr[11]"))}
        if self.idr(12) != 0 { try!(write!(f, " idr[12]"))}
        if self.idr(13) != 0 { try!(write!(f, " idr[13]"))}
        if self.idr(14) != 0 { try!(write!(f, " idr[14]"))}
        if self.idr(15) != 0 { try!(write!(f, " idr[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO port output data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Odr(pub u32);
impl Odr {
    #[doc="Port output data (y = 0..15)"]
    #[inline] pub fn odr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ODR != 0"]
    #[inline] pub fn test_odr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.odr(index) != 0
    }

    #[doc="Sets the ODR field."]
    #[inline] pub fn set_odr<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Odr {
    #[inline]
    fn from(other: u32) -> Self {
         Odr(other)
    }
}

impl ::core::fmt::Display for Odr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Odr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.odr(0) != 0 { try!(write!(f, " odr[0]"))}
        if self.odr(1) != 0 { try!(write!(f, " odr[1]"))}
        if self.odr(2) != 0 { try!(write!(f, " odr[2]"))}
        if self.odr(3) != 0 { try!(write!(f, " odr[3]"))}
        if self.odr(4) != 0 { try!(write!(f, " odr[4]"))}
        if self.odr(5) != 0 { try!(write!(f, " odr[5]"))}
        if self.odr(6) != 0 { try!(write!(f, " odr[6]"))}
        if self.odr(7) != 0 { try!(write!(f, " odr[7]"))}
        if self.odr(8) != 0 { try!(write!(f, " odr[8]"))}
        if self.odr(9) != 0 { try!(write!(f, " odr[9]"))}
        if self.odr(10) != 0 { try!(write!(f, " odr[10]"))}
        if self.odr(11) != 0 { try!(write!(f, " odr[11]"))}
        if self.odr(12) != 0 { try!(write!(f, " odr[12]"))}
        if self.odr(13) != 0 { try!(write!(f, " odr[13]"))}
        if self.odr(14) != 0 { try!(write!(f, " odr[14]"))}
        if self.odr(15) != 0 { try!(write!(f, " odr[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO port bit set/reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bsrr(pub u32);
impl Bsrr {
    #[doc="Port x reset bit y (y = 0..15)"]
    #[inline] pub fn br<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if BR != 0"]
    #[inline] pub fn test_br<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.br(index) != 0
    }

    #[doc="Sets the BR field."]
    #[inline] pub fn set_br<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Port x set bit y (y= 0..15)"]
    #[inline] pub fn bs<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BS != 0"]
    #[inline] pub fn test_bs<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.bs(index) != 0
    }

    #[doc="Sets the BS field."]
    #[inline] pub fn set_bs<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Bsrr {
    #[inline]
    fn from(other: u32) -> Self {
         Bsrr(other)
    }
}

impl ::core::fmt::Display for Bsrr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bsrr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.br(0) != 0 { try!(write!(f, " br[0]"))}
        if self.br(1) != 0 { try!(write!(f, " br[1]"))}
        if self.br(2) != 0 { try!(write!(f, " br[2]"))}
        if self.br(3) != 0 { try!(write!(f, " br[3]"))}
        if self.br(4) != 0 { try!(write!(f, " br[4]"))}
        if self.br(5) != 0 { try!(write!(f, " br[5]"))}
        if self.br(6) != 0 { try!(write!(f, " br[6]"))}
        if self.br(7) != 0 { try!(write!(f, " br[7]"))}
        if self.br(8) != 0 { try!(write!(f, " br[8]"))}
        if self.br(9) != 0 { try!(write!(f, " br[9]"))}
        if self.br(10) != 0 { try!(write!(f, " br[10]"))}
        if self.br(11) != 0 { try!(write!(f, " br[11]"))}
        if self.br(12) != 0 { try!(write!(f, " br[12]"))}
        if self.br(13) != 0 { try!(write!(f, " br[13]"))}
        if self.br(14) != 0 { try!(write!(f, " br[14]"))}
        if self.br(15) != 0 { try!(write!(f, " br[15]"))}
        if self.bs(0) != 0 { try!(write!(f, " bs[0]"))}
        if self.bs(1) != 0 { try!(write!(f, " bs[1]"))}
        if self.bs(2) != 0 { try!(write!(f, " bs[2]"))}
        if self.bs(3) != 0 { try!(write!(f, " bs[3]"))}
        if self.bs(4) != 0 { try!(write!(f, " bs[4]"))}
        if self.bs(5) != 0 { try!(write!(f, " bs[5]"))}
        if self.bs(6) != 0 { try!(write!(f, " bs[6]"))}
        if self.bs(7) != 0 { try!(write!(f, " bs[7]"))}
        if self.bs(8) != 0 { try!(write!(f, " bs[8]"))}
        if self.bs(9) != 0 { try!(write!(f, " bs[9]"))}
        if self.bs(10) != 0 { try!(write!(f, " bs[10]"))}
        if self.bs(11) != 0 { try!(write!(f, " bs[11]"))}
        if self.bs(12) != 0 { try!(write!(f, " bs[12]"))}
        if self.bs(13) != 0 { try!(write!(f, " bs[13]"))}
        if self.bs(14) != 0 { try!(write!(f, " bs[14]"))}
        if self.bs(15) != 0 { try!(write!(f, " bs[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO port configuration lock register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lckr(pub u32);
impl Lckr {
    #[doc="Lock Key"]
    #[inline] pub fn lckk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if LCKK != 0"]
    #[inline] pub fn test_lckk(&self) -> bool {
        self.lckk() != 0
    }

    #[doc="Sets the LCKK field."]
    #[inline] pub fn set_lckk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Port x lock bit y (y= 0..15)"]
    #[inline] pub fn lck<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LCK != 0"]
    #[inline] pub fn test_lck<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.lck(index) != 0
    }

    #[doc="Sets the LCK field."]
    #[inline] pub fn set_lck<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Lckr {
    #[inline]
    fn from(other: u32) -> Self {
         Lckr(other)
    }
}

impl ::core::fmt::Display for Lckr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lckr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lckk() != 0 { try!(write!(f, " lckk"))}
        if self.lck(0) != 0 { try!(write!(f, " lck[0]"))}
        if self.lck(1) != 0 { try!(write!(f, " lck[1]"))}
        if self.lck(2) != 0 { try!(write!(f, " lck[2]"))}
        if self.lck(3) != 0 { try!(write!(f, " lck[3]"))}
        if self.lck(4) != 0 { try!(write!(f, " lck[4]"))}
        if self.lck(5) != 0 { try!(write!(f, " lck[5]"))}
        if self.lck(6) != 0 { try!(write!(f, " lck[6]"))}
        if self.lck(7) != 0 { try!(write!(f, " lck[7]"))}
        if self.lck(8) != 0 { try!(write!(f, " lck[8]"))}
        if self.lck(9) != 0 { try!(write!(f, " lck[9]"))}
        if self.lck(10) != 0 { try!(write!(f, " lck[10]"))}
        if self.lck(11) != 0 { try!(write!(f, " lck[11]"))}
        if self.lck(12) != 0 { try!(write!(f, " lck[12]"))}
        if self.lck(13) != 0 { try!(write!(f, " lck[13]"))}
        if self.lck(14) != 0 { try!(write!(f, " lck[14]"))}
        if self.lck(15) != 0 { try!(write!(f, " lck[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO alternate function low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afrl(pub u32);
impl Afrl {
    #[doc="Alternate function selection for port x bit y (y = 0..7)"]
    #[inline] pub fn afrl<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if AFRL != 0"]
    #[inline] pub fn test_afrl<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.afrl(index) != 0
    }

    #[doc="Sets the AFRL field."]
    #[inline] pub fn set_afrl<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Afrl {
    #[inline]
    fn from(other: u32) -> Self {
         Afrl(other)
    }
}

impl ::core::fmt::Display for Afrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.afrl(0) != 0 { try!(write!(f, " afrl[0]=0x{:x}", self.afrl(0)))}
        if self.afrl(1) != 0 { try!(write!(f, " afrl[1]=0x{:x}", self.afrl(1)))}
        if self.afrl(2) != 0 { try!(write!(f, " afrl[2]=0x{:x}", self.afrl(2)))}
        if self.afrl(3) != 0 { try!(write!(f, " afrl[3]=0x{:x}", self.afrl(3)))}
        if self.afrl(4) != 0 { try!(write!(f, " afrl[4]=0x{:x}", self.afrl(4)))}
        if self.afrl(5) != 0 { try!(write!(f, " afrl[5]=0x{:x}", self.afrl(5)))}
        if self.afrl(6) != 0 { try!(write!(f, " afrl[6]=0x{:x}", self.afrl(6)))}
        if self.afrl(7) != 0 { try!(write!(f, " afrl[7]=0x{:x}", self.afrl(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="GPIO alternate function high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Afrh(pub u32);
impl Afrh {
    #[doc="Alternate function selection for port x bit y (y = 8..15)"]
    #[inline] pub fn afrh<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if AFRH != 0"]
    #[inline] pub fn test_afrh<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.afrh(index) != 0
    }

    #[doc="Sets the AFRH field."]
    #[inline] pub fn set_afrh<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Afrh {
    #[inline]
    fn from(other: u32) -> Self {
         Afrh(other)
    }
}

impl ::core::fmt::Display for Afrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Afrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.afrh(0) != 0 { try!(write!(f, " afrh[0]=0x{:x}", self.afrh(0)))}
        if self.afrh(1) != 0 { try!(write!(f, " afrh[1]=0x{:x}", self.afrh(1)))}
        if self.afrh(2) != 0 { try!(write!(f, " afrh[2]=0x{:x}", self.afrh(2)))}
        if self.afrh(3) != 0 { try!(write!(f, " afrh[3]=0x{:x}", self.afrh(3)))}
        if self.afrh(4) != 0 { try!(write!(f, " afrh[4]=0x{:x}", self.afrh(4)))}
        if self.afrh(5) != 0 { try!(write!(f, " afrh[5]=0x{:x}", self.afrh(5)))}
        if self.afrh(6) != 0 { try!(write!(f, " afrh[6]=0x{:x}", self.afrh(6)))}
        if self.afrh(7) != 0 { try!(write!(f, " afrh[7]=0x{:x}", self.afrh(7)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port bit reset register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc="Port x reset bit y"]
    #[inline] pub fn br<I: Into<::bobbin_bits::R16>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BR != 0"]
    #[inline] pub fn test_br<I: Into<::bobbin_bits::R16>>(&self, index: I) -> bool{
        self.br(index) != 0
    }

    #[doc="Sets the BR field."]
    #[inline] pub fn set_br<I: Into<::bobbin_bits::R16>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Brr {
    #[inline]
    fn from(other: u32) -> Self {
         Brr(other)
    }
}

impl ::core::fmt::Display for Brr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Brr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.br(0) != 0 { try!(write!(f, " br[0]"))}
        if self.br(1) != 0 { try!(write!(f, " br[1]"))}
        if self.br(2) != 0 { try!(write!(f, " br[2]"))}
        if self.br(3) != 0 { try!(write!(f, " br[3]"))}
        if self.br(4) != 0 { try!(write!(f, " br[4]"))}
        if self.br(5) != 0 { try!(write!(f, " br[5]"))}
        if self.br(6) != 0 { try!(write!(f, " br[6]"))}
        if self.br(7) != 0 { try!(write!(f, " br[7]"))}
        if self.br(8) != 0 { try!(write!(f, " br[8]"))}
        if self.br(9) != 0 { try!(write!(f, " br[9]"))}
        if self.br(10) != 0 { try!(write!(f, " br[10]"))}
        if self.br(11) != 0 { try!(write!(f, " br[11]"))}
        if self.br(12) != 0 { try!(write!(f, " br[12]"))}
        if self.br(13) != 0 { try!(write!(f, " br[13]"))}
        if self.br(14) != 0 { try!(write!(f, " br[14]"))}
        if self.br(15) != 0 { try!(write!(f, " br[15]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

