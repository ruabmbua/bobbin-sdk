#[allow(unused_imports)] use bobbin_common::*;

periph!( UART0, Uart0, _UART0, UartPeriph, 0x4000c000);
periph!( UART1, Uart1, _UART1, UartPeriph, 0x4000d000);
periph!( UART2, Uart2, _UART2, UartPeriph, 0x4000e000);
periph!( UART3, Uart3, _UART3, UartPeriph, 0x4000f000);
periph!( UART4, Uart4, _UART4, UartPeriph, 0x40010000);
periph!( UART5, Uart5, _UART5, UartPeriph, 0x40011000);
periph!( UART6, Uart6, _UART6, UartPeriph, 0x40012000);
periph!( UART7, Uart7, _UART7, UartPeriph, 0x40013000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="UART Peripheral"]
pub struct UartPeriph(pub usize); 

impl super::sig::Signal<super::sig::U0rx> for Uart0 {}
impl super::sig::SignalRx<super::sig::U0rx> for Uart0 {}
impl super::sig::Signal<super::sig::U0tx> for Uart0 {}
impl super::sig::SignalTx<super::sig::U0tx> for Uart0 {}

impl super::sig::Signal<super::sig::U1rx> for Uart1 {}
impl super::sig::SignalRx<super::sig::U1rx> for Uart1 {}
impl super::sig::Signal<super::sig::U1tx> for Uart1 {}
impl super::sig::SignalTx<super::sig::U1tx> for Uart1 {}

impl super::sig::Signal<super::sig::U2rx> for Uart2 {}
impl super::sig::SignalRx<super::sig::U2rx> for Uart2 {}
impl super::sig::Signal<super::sig::U2tx> for Uart2 {}
impl super::sig::SignalTx<super::sig::U2tx> for Uart2 {}

impl super::sig::Signal<super::sig::U3rx> for Uart3 {}
impl super::sig::SignalRx<super::sig::U3rx> for Uart3 {}
impl super::sig::Signal<super::sig::U3tx> for Uart3 {}
impl super::sig::SignalTx<super::sig::U3tx> for Uart3 {}

impl super::sig::Signal<super::sig::U4rx> for Uart4 {}
impl super::sig::SignalRx<super::sig::U4rx> for Uart4 {}
impl super::sig::Signal<super::sig::U4tx> for Uart4 {}
impl super::sig::SignalTx<super::sig::U4tx> for Uart4 {}

impl super::sig::Signal<super::sig::U5rx> for Uart5 {}
impl super::sig::SignalRx<super::sig::U5rx> for Uart5 {}
impl super::sig::Signal<super::sig::U5tx> for Uart5 {}
impl super::sig::SignalTx<super::sig::U5tx> for Uart5 {}

impl super::sig::Signal<super::sig::U6rx> for Uart6 {}
impl super::sig::SignalRx<super::sig::U6rx> for Uart6 {}
impl super::sig::Signal<super::sig::U6tx> for Uart6 {}
impl super::sig::SignalTx<super::sig::U6tx> for Uart6 {}

impl super::sig::Signal<super::sig::U7rx> for Uart7 {}
impl super::sig::SignalRx<super::sig::U7rx> for Uart7 {}
impl super::sig::Signal<super::sig::U7tx> for Uart7 {}
impl super::sig::SignalTx<super::sig::U7tx> for Uart7 {}


impl UartPeriph {
    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        (self.0 + 0x0) as *mut Dr
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const Dr { 
           self.dr_mut()
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        unsafe {
            read_volatile(self.dr_ptr())
        }
    }

    #[doc="Write the DR register."]
    #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr_mut(), f(Dr(0)));
        }
        self
    }

    #[doc="Modify the DR register."]
    #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr_mut(), f(self.dr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RSR register."]
    #[inline] pub fn rsr_mut(&self) -> *mut Rsr { 
        (self.0 + 0x4) as *mut Rsr
    }

    #[doc="Get the *const pointer for the RSR register."]
    #[inline] pub fn rsr_ptr(&self) -> *const Rsr { 
           self.rsr_mut()
    }

    #[doc="Read the RSR register."]
    #[inline] pub fn rsr(&self) -> Rsr { 
        unsafe {
            read_volatile(self.rsr_ptr())
        }
    }

    #[doc="Write the RSR register."]
    #[inline] pub fn set_rsr<F: FnOnce(Rsr) -> Rsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rsr_mut(), f(Rsr(0)));
        }
        self
    }

    #[doc="Modify the RSR register."]
    #[inline] pub fn with_rsr<F: FnOnce(Rsr) -> Rsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rsr_mut(), f(self.rsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ECR register."]
    #[inline] pub fn ecr_mut(&self) -> *mut Ecr { 
        (self.0 + 0x4) as *mut Ecr
    }

    #[doc="Get the *const pointer for the ECR register."]
    #[inline] pub fn ecr_ptr(&self) -> *const Ecr { 
           self.ecr_mut()
    }

    #[doc="Read the ECR register."]
    #[inline] pub fn ecr(&self) -> Ecr { 
        unsafe {
            read_volatile(self.ecr_ptr())
        }
    }

    #[doc="Write the ECR register."]
    #[inline] pub fn set_ecr<F: FnOnce(Ecr) -> Ecr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ecr_mut(), f(Ecr(0)));
        }
        self
    }

    #[doc="Modify the ECR register."]
    #[inline] pub fn with_ecr<F: FnOnce(Ecr) -> Ecr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ecr_mut(), f(self.ecr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FR register."]
    #[inline] pub fn fr_mut(&self) -> *mut Fr { 
        (self.0 + 0x18) as *mut Fr
    }

    #[doc="Get the *const pointer for the FR register."]
    #[inline] pub fn fr_ptr(&self) -> *const Fr { 
           self.fr_mut()
    }

    #[doc="Read the FR register."]
    #[inline] pub fn fr(&self) -> Fr { 
        unsafe {
            read_volatile(self.fr_ptr())
        }
    }

    #[doc="Write the FR register."]
    #[inline] pub fn set_fr<F: FnOnce(Fr) -> Fr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr_mut(), f(Fr(0)));
        }
        self
    }

    #[doc="Modify the FR register."]
    #[inline] pub fn with_fr<F: FnOnce(Fr) -> Fr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr_mut(), f(self.fr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ILPR register."]
    #[inline] pub fn ilpr_mut(&self) -> *mut Ilpr { 
        (self.0 + 0x20) as *mut Ilpr
    }

    #[doc="Get the *const pointer for the ILPR register."]
    #[inline] pub fn ilpr_ptr(&self) -> *const Ilpr { 
           self.ilpr_mut()
    }

    #[doc="Read the ILPR register."]
    #[inline] pub fn ilpr(&self) -> Ilpr { 
        unsafe {
            read_volatile(self.ilpr_ptr())
        }
    }

    #[doc="Write the ILPR register."]
    #[inline] pub fn set_ilpr<F: FnOnce(Ilpr) -> Ilpr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ilpr_mut(), f(Ilpr(0)));
        }
        self
    }

    #[doc="Modify the ILPR register."]
    #[inline] pub fn with_ilpr<F: FnOnce(Ilpr) -> Ilpr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ilpr_mut(), f(self.ilpr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IBRD register."]
    #[inline] pub fn ibrd_mut(&self) -> *mut Ibrd { 
        (self.0 + 0x24) as *mut Ibrd
    }

    #[doc="Get the *const pointer for the IBRD register."]
    #[inline] pub fn ibrd_ptr(&self) -> *const Ibrd { 
           self.ibrd_mut()
    }

    #[doc="Read the IBRD register."]
    #[inline] pub fn ibrd(&self) -> Ibrd { 
        unsafe {
            read_volatile(self.ibrd_ptr())
        }
    }

    #[doc="Write the IBRD register."]
    #[inline] pub fn set_ibrd<F: FnOnce(Ibrd) -> Ibrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ibrd_mut(), f(Ibrd(0)));
        }
        self
    }

    #[doc="Modify the IBRD register."]
    #[inline] pub fn with_ibrd<F: FnOnce(Ibrd) -> Ibrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ibrd_mut(), f(self.ibrd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FBRD register."]
    #[inline] pub fn fbrd_mut(&self) -> *mut Fbrd { 
        (self.0 + 0x28) as *mut Fbrd
    }

    #[doc="Get the *const pointer for the FBRD register."]
    #[inline] pub fn fbrd_ptr(&self) -> *const Fbrd { 
           self.fbrd_mut()
    }

    #[doc="Read the FBRD register."]
    #[inline] pub fn fbrd(&self) -> Fbrd { 
        unsafe {
            read_volatile(self.fbrd_ptr())
        }
    }

    #[doc="Write the FBRD register."]
    #[inline] pub fn set_fbrd<F: FnOnce(Fbrd) -> Fbrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fbrd_mut(), f(Fbrd(0)));
        }
        self
    }

    #[doc="Modify the FBRD register."]
    #[inline] pub fn with_fbrd<F: FnOnce(Fbrd) -> Fbrd>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fbrd_mut(), f(self.fbrd()));
        }
        self
    }

    #[doc="Get the *mut pointer for the LCRH register."]
    #[inline] pub fn lcrh_mut(&self) -> *mut Lcrh { 
        (self.0 + 0x2c) as *mut Lcrh
    }

    #[doc="Get the *const pointer for the LCRH register."]
    #[inline] pub fn lcrh_ptr(&self) -> *const Lcrh { 
           self.lcrh_mut()
    }

    #[doc="Read the LCRH register."]
    #[inline] pub fn lcrh(&self) -> Lcrh { 
        unsafe {
            read_volatile(self.lcrh_ptr())
        }
    }

    #[doc="Write the LCRH register."]
    #[inline] pub fn set_lcrh<F: FnOnce(Lcrh) -> Lcrh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lcrh_mut(), f(Lcrh(0)));
        }
        self
    }

    #[doc="Modify the LCRH register."]
    #[inline] pub fn with_lcrh<F: FnOnce(Lcrh) -> Lcrh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.lcrh_mut(), f(self.lcrh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CTL register."]
    #[inline] pub fn ctl_mut(&self) -> *mut Ctl { 
        (self.0 + 0x30) as *mut Ctl
    }

    #[doc="Get the *const pointer for the CTL register."]
    #[inline] pub fn ctl_ptr(&self) -> *const Ctl { 
           self.ctl_mut()
    }

    #[doc="Read the CTL register."]
    #[inline] pub fn ctl(&self) -> Ctl { 
        unsafe {
            read_volatile(self.ctl_ptr())
        }
    }

    #[doc="Write the CTL register."]
    #[inline] pub fn set_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctl_mut(), f(Ctl(0)));
        }
        self
    }

    #[doc="Modify the CTL register."]
    #[inline] pub fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ctl_mut(), f(self.ctl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IFLS register."]
    #[inline] pub fn ifls_mut(&self) -> *mut Ifls { 
        (self.0 + 0x34) as *mut Ifls
    }

    #[doc="Get the *const pointer for the IFLS register."]
    #[inline] pub fn ifls_ptr(&self) -> *const Ifls { 
           self.ifls_mut()
    }

    #[doc="Read the IFLS register."]
    #[inline] pub fn ifls(&self) -> Ifls { 
        unsafe {
            read_volatile(self.ifls_ptr())
        }
    }

    #[doc="Write the IFLS register."]
    #[inline] pub fn set_ifls<F: FnOnce(Ifls) -> Ifls>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifls_mut(), f(Ifls(0)));
        }
        self
    }

    #[doc="Modify the IFLS register."]
    #[inline] pub fn with_ifls<F: FnOnce(Ifls) -> Ifls>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ifls_mut(), f(self.ifls()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IM register."]
    #[inline] pub fn im_mut(&self) -> *mut Im { 
        (self.0 + 0x38) as *mut Im
    }

    #[doc="Get the *const pointer for the IM register."]
    #[inline] pub fn im_ptr(&self) -> *const Im { 
           self.im_mut()
    }

    #[doc="Read the IM register."]
    #[inline] pub fn im(&self) -> Im { 
        unsafe {
            read_volatile(self.im_ptr())
        }
    }

    #[doc="Write the IM register."]
    #[inline] pub fn set_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.im_mut(), f(Im(0)));
        }
        self
    }

    #[doc="Modify the IM register."]
    #[inline] pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.im_mut(), f(self.im()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RIS register."]
    #[inline] pub fn ris_mut(&self) -> *mut Ris { 
        (self.0 + 0x3c) as *mut Ris
    }

    #[doc="Get the *const pointer for the RIS register."]
    #[inline] pub fn ris_ptr(&self) -> *const Ris { 
           self.ris_mut()
    }

    #[doc="Read the RIS register."]
    #[inline] pub fn ris(&self) -> Ris { 
        unsafe {
            read_volatile(self.ris_ptr())
        }
    }

    #[doc="Write the RIS register."]
    #[inline] pub fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ris_mut(), f(Ris(0)));
        }
        self
    }

    #[doc="Modify the RIS register."]
    #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ris_mut(), f(self.ris()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MIS register."]
    #[inline] pub fn mis_mut(&self) -> *mut Mis { 
        (self.0 + 0x40) as *mut Mis
    }

    #[doc="Get the *const pointer for the MIS register."]
    #[inline] pub fn mis_ptr(&self) -> *const Mis { 
           self.mis_mut()
    }

    #[doc="Read the MIS register."]
    #[inline] pub fn mis(&self) -> Mis { 
        unsafe {
            read_volatile(self.mis_ptr())
        }
    }

    #[doc="Write the MIS register."]
    #[inline] pub fn set_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(Mis(0)));
        }
        self
    }

    #[doc="Modify the MIS register."]
    #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(self.mis()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        (self.0 + 0x44) as *mut Icr
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
           self.icr_mut()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icr_mut(), f(Icr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DMACTL register."]
    #[inline] pub fn dmactl_mut(&self) -> *mut Dmactl { 
        (self.0 + 0x48) as *mut Dmactl
    }

    #[doc="Get the *const pointer for the DMACTL register."]
    #[inline] pub fn dmactl_ptr(&self) -> *const Dmactl { 
           self.dmactl_mut()
    }

    #[doc="Read the DMACTL register."]
    #[inline] pub fn dmactl(&self) -> Dmactl { 
        unsafe {
            read_volatile(self.dmactl_ptr())
        }
    }

    #[doc="Write the DMACTL register."]
    #[inline] pub fn set_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmactl_mut(), f(Dmactl(0)));
        }
        self
    }

    #[doc="Modify the DMACTL register."]
    #[inline] pub fn with_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmactl_mut(), f(self.dmactl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the _9BITADDR register."]
    #[inline] pub fn _9bitaddr_mut(&self) -> *mut _9bitaddr { 
        (self.0 + 0xa4) as *mut _9bitaddr
    }

    #[doc="Get the *const pointer for the _9BITADDR register."]
    #[inline] pub fn _9bitaddr_ptr(&self) -> *const _9bitaddr { 
           self._9bitaddr_mut()
    }

    #[doc="Read the _9BITADDR register."]
    #[inline] pub fn _9bitaddr(&self) -> _9bitaddr { 
        unsafe {
            read_volatile(self._9bitaddr_ptr())
        }
    }

    #[doc="Write the _9BITADDR register."]
    #[inline] pub fn set_9bitaddr<F: FnOnce(_9bitaddr) -> _9bitaddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self._9bitaddr_mut(), f(_9bitaddr(0)));
        }
        self
    }

    #[doc="Modify the _9BITADDR register."]
    #[inline] pub fn with_9bitaddr<F: FnOnce(_9bitaddr) -> _9bitaddr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self._9bitaddr_mut(), f(self._9bitaddr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the _9BITAMASK register."]
    #[inline] pub fn _9bitamask_mut(&self) -> *mut _9bitamask { 
        (self.0 + 0xa8) as *mut _9bitamask
    }

    #[doc="Get the *const pointer for the _9BITAMASK register."]
    #[inline] pub fn _9bitamask_ptr(&self) -> *const _9bitamask { 
           self._9bitamask_mut()
    }

    #[doc="Read the _9BITAMASK register."]
    #[inline] pub fn _9bitamask(&self) -> _9bitamask { 
        unsafe {
            read_volatile(self._9bitamask_ptr())
        }
    }

    #[doc="Write the _9BITAMASK register."]
    #[inline] pub fn set_9bitamask<F: FnOnce(_9bitamask) -> _9bitamask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self._9bitamask_mut(), f(_9bitamask(0)));
        }
        self
    }

    #[doc="Modify the _9BITAMASK register."]
    #[inline] pub fn with_9bitamask<F: FnOnce(_9bitamask) -> _9bitamask>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self._9bitamask_mut(), f(self._9bitamask()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PP register."]
    #[inline] pub fn pp_mut(&self) -> *mut Pp { 
        (self.0 + 0xfc0) as *mut Pp
    }

    #[doc="Get the *const pointer for the PP register."]
    #[inline] pub fn pp_ptr(&self) -> *const Pp { 
           self.pp_mut()
    }

    #[doc="Read the PP register."]
    #[inline] pub fn pp(&self) -> Pp { 
        unsafe {
            read_volatile(self.pp_ptr())
        }
    }

    #[doc="Write the PP register."]
    #[inline] pub fn set_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pp_mut(), f(Pp(0)));
        }
        self
    }

    #[doc="Modify the PP register."]
    #[inline] pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pp_mut(), f(self.pp()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CC register."]
    #[inline] pub fn cc_mut(&self) -> *mut Cc { 
        (self.0 + 0xfc8) as *mut Cc
    }

    #[doc="Get the *const pointer for the CC register."]
    #[inline] pub fn cc_ptr(&self) -> *const Cc { 
           self.cc_mut()
    }

    #[doc="Read the CC register."]
    #[inline] pub fn cc(&self) -> Cc { 
        unsafe {
            read_volatile(self.cc_ptr())
        }
    }

    #[doc="Write the CC register."]
    #[inline] pub fn set_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(), f(Cc(0)));
        }
        self
    }

    #[doc="Modify the CC register."]
    #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(), f(self.cc()));
        }
        self
    }

}

#[doc="UART Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="Data Transmitted or Received"]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Framing Error"]
    #[inline] pub fn fe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FE != 0"]
    #[inline] pub fn test_fe(&self) -> bool {
        self.fe() != 0
    }

    #[doc="Sets the FE field."]
    #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="UART Parity Error"]
    #[inline] pub fn pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="UART Break Error"]
    #[inline] pub fn be(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BE != 0"]
    #[inline] pub fn test_be(&self) -> bool {
        self.be() != 0
    }

    #[doc="Sets the BE field."]
    #[inline] pub fn set_be<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="UART Overrun Error"]
    #[inline] pub fn oe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if OE != 0"]
    #[inline] pub fn test_oe(&self) -> bool {
        self.oe() != 0
    }

    #[doc="Sets the OE field."]
    #[inline] pub fn set_oe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Dr {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        if self.fe() != 0 { try!(write!(f, " fe"))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        if self.be() != 0 { try!(write!(f, " be"))}
        if self.oe() != 0 { try!(write!(f, " oe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Receive Status/Error Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rsr(pub u32);
impl Rsr {
    #[doc="UART Framing Error"]
    #[inline] pub fn fe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FE != 0"]
    #[inline] pub fn test_fe(&self) -> bool {
        self.fe() != 0
    }

    #[doc="Sets the FE field."]
    #[inline] pub fn set_fe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Parity Error"]
    #[inline] pub fn pe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Break Error"]
    #[inline] pub fn be(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BE != 0"]
    #[inline] pub fn test_be(&self) -> bool {
        self.be() != 0
    }

    #[doc="Sets the BE field."]
    #[inline] pub fn set_be<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Overrun Error"]
    #[inline] pub fn oe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OE != 0"]
    #[inline] pub fn test_oe(&self) -> bool {
        self.oe() != 0
    }

    #[doc="Sets the OE field."]
    #[inline] pub fn set_oe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Rsr {
    #[inline]
    fn from(other: u32) -> Self {
         Rsr(other)
    }
}

impl ::core::fmt::Display for Rsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fe() != 0 { try!(write!(f, " fe"))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        if self.be() != 0 { try!(write!(f, " be"))}
        if self.oe() != 0 { try!(write!(f, " oe"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Receive Status/Error Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc="Error Clear"]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ecr {
    #[inline]
    fn from(other: u32) -> Self {
         Ecr(other)
    }
}

impl ::core::fmt::Display for Ecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ecr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Flag"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fr(pub u32);
impl Fr {
    #[doc="Clear To Send"]
    #[inline] pub fn cts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CTS != 0"]
    #[inline] pub fn test_cts(&self) -> bool {
        self.cts() != 0
    }

    #[doc="Sets the CTS field."]
    #[inline] pub fn set_cts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Set Ready"]
    #[inline] pub fn dsr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DSR != 0"]
    #[inline] pub fn test_dsr(&self) -> bool {
        self.dsr() != 0
    }

    #[doc="Sets the DSR field."]
    #[inline] pub fn set_dsr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Data Carrier Detect"]
    #[inline] pub fn dcd(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCD != 0"]
    #[inline] pub fn test_dcd(&self) -> bool {
        self.dcd() != 0
    }

    #[doc="Sets the DCD field."]
    #[inline] pub fn set_dcd<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Busy"]
    #[inline] pub fn busy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BUSY != 0"]
    #[inline] pub fn test_busy(&self) -> bool {
        self.busy() != 0
    }

    #[doc="Sets the BUSY field."]
    #[inline] pub fn set_busy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Receive FIFO Empty"]
    #[inline] pub fn rxfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXFE != 0"]
    #[inline] pub fn test_rxfe(&self) -> bool {
        self.rxfe() != 0
    }

    #[doc="Sets the RXFE field."]
    #[inline] pub fn set_rxfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Transmit FIFO Full"]
    #[inline] pub fn txff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXFF != 0"]
    #[inline] pub fn test_txff(&self) -> bool {
        self.txff() != 0
    }

    #[doc="Sets the TXFF field."]
    #[inline] pub fn set_txff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Receive FIFO Full"]
    #[inline] pub fn rxff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXFF != 0"]
    #[inline] pub fn test_rxff(&self) -> bool {
        self.rxff() != 0
    }

    #[doc="Sets the RXFF field."]
    #[inline] pub fn set_rxff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Transmit FIFO Empty"]
    #[inline] pub fn txfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TXFE != 0"]
    #[inline] pub fn test_txfe(&self) -> bool {
        self.txfe() != 0
    }

    #[doc="Sets the TXFE field."]
    #[inline] pub fn set_txfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Ring Indicator"]
    #[inline] pub fn ri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RI != 0"]
    #[inline] pub fn test_ri(&self) -> bool {
        self.ri() != 0
    }

    #[doc="Sets the RI field."]
    #[inline] pub fn set_ri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Fr {
    #[inline]
    fn from(other: u32) -> Self {
         Fr(other)
    }
}

impl ::core::fmt::Display for Fr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cts() != 0 { try!(write!(f, " cts"))}
        if self.dsr() != 0 { try!(write!(f, " dsr"))}
        if self.dcd() != 0 { try!(write!(f, " dcd"))}
        if self.busy() != 0 { try!(write!(f, " busy"))}
        if self.rxfe() != 0 { try!(write!(f, " rxfe"))}
        if self.txff() != 0 { try!(write!(f, " txff"))}
        if self.rxff() != 0 { try!(write!(f, " rxff"))}
        if self.txfe() != 0 { try!(write!(f, " txfe"))}
        if self.ri() != 0 { try!(write!(f, " ri"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART IrDA Low-Power Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ilpr(pub u32);
impl Ilpr {
    #[doc="IrDA Low-Power Divisor"]
    #[inline] pub fn ilpdvsr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ILPDVSR != 0"]
    #[inline] pub fn test_ilpdvsr(&self) -> bool {
        self.ilpdvsr() != 0
    }

    #[doc="Sets the ILPDVSR field."]
    #[inline] pub fn set_ilpdvsr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ilpr {
    #[inline]
    fn from(other: u32) -> Self {
         Ilpr(other)
    }
}

impl ::core::fmt::Display for Ilpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ilpr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ilpdvsr() != 0 { try!(write!(f, " ilpdvsr=0x{:x}", self.ilpdvsr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Integer Baud-Rate Divisor"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ibrd(pub u32);
impl Ibrd {
    #[doc="Integer Baud-Rate Divisor"]
    #[inline] pub fn divint(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DIVINT != 0"]
    #[inline] pub fn test_divint(&self) -> bool {
        self.divint() != 0
    }

    #[doc="Sets the DIVINT field."]
    #[inline] pub fn set_divint<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ibrd {
    #[inline]
    fn from(other: u32) -> Self {
         Ibrd(other)
    }
}

impl ::core::fmt::Display for Ibrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ibrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.divint() != 0 { try!(write!(f, " divint=0x{:x}", self.divint()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Fractional Baud-Rate Divisor"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fbrd(pub u32);
impl Fbrd {
    #[doc="Fractional Baud-Rate Divisor"]
    #[inline] pub fn divfrac(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DIVFRAC != 0"]
    #[inline] pub fn test_divfrac(&self) -> bool {
        self.divfrac() != 0
    }

    #[doc="Sets the DIVFRAC field."]
    #[inline] pub fn set_divfrac<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fbrd {
    #[inline]
    fn from(other: u32) -> Self {
         Fbrd(other)
    }
}

impl ::core::fmt::Display for Fbrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fbrd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.divfrac() != 0 { try!(write!(f, " divfrac=0x{:x}", self.divfrac()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Line Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Lcrh(pub u32);
impl Lcrh {
    #[doc="UART Send Break"]
    #[inline] pub fn brk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BRK != 0"]
    #[inline] pub fn test_brk(&self) -> bool {
        self.brk() != 0
    }

    #[doc="Sets the BRK field."]
    #[inline] pub fn set_brk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Parity Enable"]
    #[inline] pub fn pen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PEN != 0"]
    #[inline] pub fn test_pen(&self) -> bool {
        self.pen() != 0
    }

    #[doc="Sets the PEN field."]
    #[inline] pub fn set_pen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Even Parity Select"]
    #[inline] pub fn eps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EPS != 0"]
    #[inline] pub fn test_eps(&self) -> bool {
        self.eps() != 0
    }

    #[doc="Sets the EPS field."]
    #[inline] pub fn set_eps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Two Stop Bits Select"]
    #[inline] pub fn stp2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if STP2 != 0"]
    #[inline] pub fn test_stp2(&self) -> bool {
        self.stp2() != 0
    }

    #[doc="Sets the STP2 field."]
    #[inline] pub fn set_stp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Enable FIFOs"]
    #[inline] pub fn fen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FEN != 0"]
    #[inline] pub fn test_fen(&self) -> bool {
        self.fen() != 0
    }

    #[doc="Sets the FEN field."]
    #[inline] pub fn set_fen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Word Length"]
    #[inline] pub fn wlen(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if WLEN != 0"]
    #[inline] pub fn test_wlen(&self) -> bool {
        self.wlen() != 0
    }

    #[doc="Sets the WLEN field."]
    #[inline] pub fn set_wlen<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Stick Parity Select"]
    #[inline] pub fn sps(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SPS != 0"]
    #[inline] pub fn test_sps(&self) -> bool {
        self.sps() != 0
    }

    #[doc="Sets the SPS field."]
    #[inline] pub fn set_sps<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Lcrh {
    #[inline]
    fn from(other: u32) -> Self {
         Lcrh(other)
    }
}

impl ::core::fmt::Display for Lcrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Lcrh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.brk() != 0 { try!(write!(f, " brk"))}
        if self.pen() != 0 { try!(write!(f, " pen"))}
        if self.eps() != 0 { try!(write!(f, " eps"))}
        if self.stp2() != 0 { try!(write!(f, " stp2"))}
        if self.fen() != 0 { try!(write!(f, " fen"))}
        if self.wlen() != 0 { try!(write!(f, " wlen=0x{:x}", self.wlen()))}
        if self.sps() != 0 { try!(write!(f, " sps"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc="UART Enable"]
    #[inline] pub fn uarten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if UARTEN != 0"]
    #[inline] pub fn test_uarten(&self) -> bool {
        self.uarten() != 0
    }

    #[doc="Sets the UARTEN field."]
    #[inline] pub fn set_uarten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART SIR Enable"]
    #[inline] pub fn siren(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SIREN != 0"]
    #[inline] pub fn test_siren(&self) -> bool {
        self.siren() != 0
    }

    #[doc="Sets the SIREN field."]
    #[inline] pub fn set_siren<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART SIR Low-Power Mode"]
    #[inline] pub fn sirlp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if SIRLP != 0"]
    #[inline] pub fn test_sirlp(&self) -> bool {
        self.sirlp() != 0
    }

    #[doc="Sets the SIRLP field."]
    #[inline] pub fn set_sirlp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ISO 7816 Smart Card Support"]
    #[inline] pub fn smart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SMART != 0"]
    #[inline] pub fn test_smart(&self) -> bool {
        self.smart() != 0
    }

    #[doc="Sets the SMART field."]
    #[inline] pub fn set_smart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="End of Transmission"]
    #[inline] pub fn eot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EOT != 0"]
    #[inline] pub fn test_eot(&self) -> bool {
        self.eot() != 0
    }

    #[doc="Sets the EOT field."]
    #[inline] pub fn set_eot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="High-Speed Enable"]
    #[inline] pub fn hse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if HSE != 0"]
    #[inline] pub fn test_hse(&self) -> bool {
        self.hse() != 0
    }

    #[doc="Sets the HSE field."]
    #[inline] pub fn set_hse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Loop Back Enable"]
    #[inline] pub fn lbe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LBE != 0"]
    #[inline] pub fn test_lbe(&self) -> bool {
        self.lbe() != 0
    }

    #[doc="Sets the LBE field."]
    #[inline] pub fn set_lbe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="UART Transmit Enable"]
    #[inline] pub fn txe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXE != 0"]
    #[inline] pub fn test_txe(&self) -> bool {
        self.txe() != 0
    }

    #[doc="Sets the TXE field."]
    #[inline] pub fn set_txe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="UART Receive Enable"]
    #[inline] pub fn rxe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RXE != 0"]
    #[inline] pub fn test_rxe(&self) -> bool {
        self.rxe() != 0
    }

    #[doc="Sets the RXE field."]
    #[inline] pub fn set_rxe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Data Terminal Ready"]
    #[inline] pub fn dtr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DTR != 0"]
    #[inline] pub fn test_dtr(&self) -> bool {
        self.dtr() != 0
    }

    #[doc="Sets the DTR field."]
    #[inline] pub fn set_dtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Request to Send"]
    #[inline] pub fn rts(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RTS != 0"]
    #[inline] pub fn test_rts(&self) -> bool {
        self.rts() != 0
    }

    #[doc="Sets the RTS field."]
    #[inline] pub fn set_rts<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Enable Request to Send"]
    #[inline] pub fn rtsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RTSEN != 0"]
    #[inline] pub fn test_rtsen(&self) -> bool {
        self.rtsen() != 0
    }

    #[doc="Sets the RTSEN field."]
    #[inline] pub fn set_rtsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Enable Clear To Send"]
    #[inline] pub fn ctsen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CTSEN != 0"]
    #[inline] pub fn test_ctsen(&self) -> bool {
        self.ctsen() != 0
    }

    #[doc="Sets the CTSEN field."]
    #[inline] pub fn set_ctsen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Ctl {
    #[inline]
    fn from(other: u32) -> Self {
         Ctl(other)
    }
}

impl ::core::fmt::Display for Ctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.uarten() != 0 { try!(write!(f, " uarten"))}
        if self.siren() != 0 { try!(write!(f, " siren"))}
        if self.sirlp() != 0 { try!(write!(f, " sirlp"))}
        if self.smart() != 0 { try!(write!(f, " smart"))}
        if self.eot() != 0 { try!(write!(f, " eot"))}
        if self.hse() != 0 { try!(write!(f, " hse"))}
        if self.lbe() != 0 { try!(write!(f, " lbe"))}
        if self.txe() != 0 { try!(write!(f, " txe"))}
        if self.rxe() != 0 { try!(write!(f, " rxe"))}
        if self.dtr() != 0 { try!(write!(f, " dtr"))}
        if self.rts() != 0 { try!(write!(f, " rts"))}
        if self.rtsen() != 0 { try!(write!(f, " rtsen"))}
        if self.ctsen() != 0 { try!(write!(f, " ctsen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Interrupt FIFO Level Select"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ifls(pub u32);
impl Ifls {
    #[doc="UART Transmit Interrupt FIFO Level Select"]
    #[inline] pub fn tx(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if TX != 0"]
    #[inline] pub fn test_tx(&self) -> bool {
        self.tx() != 0
    }

    #[doc="Sets the TX field."]
    #[inline] pub fn set_tx<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Receive Interrupt FIFO Level Select"]
    #[inline] pub fn rx(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if RX != 0"]
    #[inline] pub fn test_rx(&self) -> bool {
        self.rx() != 0
    }

    #[doc="Sets the RX field."]
    #[inline] pub fn set_rx<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Ifls {
    #[inline]
    fn from(other: u32) -> Self {
         Ifls(other)
    }
}

impl ::core::fmt::Display for Ifls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ifls {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tx() != 0 { try!(write!(f, " tx=0x{:x}", self.tx()))}
        if self.rx() != 0 { try!(write!(f, " rx=0x{:x}", self.rx()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Im(pub u32);
impl Im {
    #[doc="UART Ring Indicator Modem Interrupt Mask"]
    #[inline] pub fn rimim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RIMIM != 0"]
    #[inline] pub fn test_rimim(&self) -> bool {
        self.rimim() != 0
    }

    #[doc="Sets the RIMIM field."]
    #[inline] pub fn set_rimim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Clear to Send Modem Interrupt Mask"]
    #[inline] pub fn ctsmim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTSMIM != 0"]
    #[inline] pub fn test_ctsmim(&self) -> bool {
        self.ctsmim() != 0
    }

    #[doc="Sets the CTSMIM field."]
    #[inline] pub fn set_ctsmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Data Carrier Detect Modem Interrupt Mask"]
    #[inline] pub fn dcdmim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCDMIM != 0"]
    #[inline] pub fn test_dcdmim(&self) -> bool {
        self.dcdmim() != 0
    }

    #[doc="Sets the DCDMIM field."]
    #[inline] pub fn set_dcdmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Data Set Ready Modem Interrupt Mask"]
    #[inline] pub fn dsrmim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DSRMIM != 0"]
    #[inline] pub fn test_dsrmim(&self) -> bool {
        self.dsrmim() != 0
    }

    #[doc="Sets the DSRMIM field."]
    #[inline] pub fn set_dsrmim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Receive Interrupt Mask"]
    #[inline] pub fn rxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXIM != 0"]
    #[inline] pub fn test_rxim(&self) -> bool {
        self.rxim() != 0
    }

    #[doc="Sets the RXIM field."]
    #[inline] pub fn set_rxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Transmit Interrupt Mask"]
    #[inline] pub fn txim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXIM != 0"]
    #[inline] pub fn test_txim(&self) -> bool {
        self.txim() != 0
    }

    #[doc="Sets the TXIM field."]
    #[inline] pub fn set_txim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Receive Time-Out Interrupt Mask"]
    #[inline] pub fn rtim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTIM != 0"]
    #[inline] pub fn test_rtim(&self) -> bool {
        self.rtim() != 0
    }

    #[doc="Sets the RTIM field."]
    #[inline] pub fn set_rtim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Framing Error Interrupt Mask"]
    #[inline] pub fn feim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FEIM != 0"]
    #[inline] pub fn test_feim(&self) -> bool {
        self.feim() != 0
    }

    #[doc="Sets the FEIM field."]
    #[inline] pub fn set_feim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="UART Parity Error Interrupt Mask"]
    #[inline] pub fn peim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PEIM != 0"]
    #[inline] pub fn test_peim(&self) -> bool {
        self.peim() != 0
    }

    #[doc="Sets the PEIM field."]
    #[inline] pub fn set_peim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="UART Break Error Interrupt Mask"]
    #[inline] pub fn beim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BEIM != 0"]
    #[inline] pub fn test_beim(&self) -> bool {
        self.beim() != 0
    }

    #[doc="Sets the BEIM field."]
    #[inline] pub fn set_beim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="UART Overrun Error Interrupt Mask"]
    #[inline] pub fn oeim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OEIM != 0"]
    #[inline] pub fn test_oeim(&self) -> bool {
        self.oeim() != 0
    }

    #[doc="Sets the OEIM field."]
    #[inline] pub fn set_oeim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="End of Transmission Interrupt Mask"]
    #[inline] pub fn eotim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if EOTIM != 0"]
    #[inline] pub fn test_eotim(&self) -> bool {
        self.eotim() != 0
    }

    #[doc="Sets the EOTIM field."]
    #[inline] pub fn set_eotim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="9-Bit Mode Interrupt Mask"]
    #[inline] pub fn _9bitim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if _9BITIM != 0"]
    #[inline] pub fn test_9bitim(&self) -> bool {
        self._9bitim() != 0
    }

    #[doc="Sets the _9BITIM field."]
    #[inline] pub fn set_9bitim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receive DMA Interrupt Mask"]
    #[inline] pub fn dmarxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DMARXIM != 0"]
    #[inline] pub fn test_dmarxim(&self) -> bool {
        self.dmarxim() != 0
    }

    #[doc="Sets the DMARXIM field."]
    #[inline] pub fn set_dmarxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Transmit DMA Interrupt Mask"]
    #[inline] pub fn dmatxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DMATXIM != 0"]
    #[inline] pub fn test_dmatxim(&self) -> bool {
        self.dmatxim() != 0
    }

    #[doc="Sets the DMATXIM field."]
    #[inline] pub fn set_dmatxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Im {
    #[inline]
    fn from(other: u32) -> Self {
         Im(other)
    }
}

impl ::core::fmt::Display for Im {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Im {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rimim() != 0 { try!(write!(f, " rimim"))}
        if self.ctsmim() != 0 { try!(write!(f, " ctsmim"))}
        if self.dcdmim() != 0 { try!(write!(f, " dcdmim"))}
        if self.dsrmim() != 0 { try!(write!(f, " dsrmim"))}
        if self.rxim() != 0 { try!(write!(f, " rxim"))}
        if self.txim() != 0 { try!(write!(f, " txim"))}
        if self.rtim() != 0 { try!(write!(f, " rtim"))}
        if self.feim() != 0 { try!(write!(f, " feim"))}
        if self.peim() != 0 { try!(write!(f, " peim"))}
        if self.beim() != 0 { try!(write!(f, " beim"))}
        if self.oeim() != 0 { try!(write!(f, " oeim"))}
        if self.eotim() != 0 { try!(write!(f, " eotim"))}
        if self._9bitim() != 0 { try!(write!(f, " _9bitim"))}
        if self.dmarxim() != 0 { try!(write!(f, " dmarxim"))}
        if self.dmatxim() != 0 { try!(write!(f, " dmatxim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc="UART Ring Indicator Modem Raw Interrupt Status"]
    #[inline] pub fn riris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RIRIS != 0"]
    #[inline] pub fn test_riris(&self) -> bool {
        self.riris() != 0
    }

    #[doc="Sets the RIRIS field."]
    #[inline] pub fn set_riris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Clear to Send Modem Raw Interrupt Status"]
    #[inline] pub fn ctsris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTSRIS != 0"]
    #[inline] pub fn test_ctsris(&self) -> bool {
        self.ctsris() != 0
    }

    #[doc="Sets the CTSRIS field."]
    #[inline] pub fn set_ctsris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Data Carrier Detect Modem Raw Interrupt Status"]
    #[inline] pub fn dcdris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCDRIS != 0"]
    #[inline] pub fn test_dcdris(&self) -> bool {
        self.dcdris() != 0
    }

    #[doc="Sets the DCDRIS field."]
    #[inline] pub fn set_dcdris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Data Set Ready Modem Raw Interrupt Status"]
    #[inline] pub fn dsrris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DSRRIS != 0"]
    #[inline] pub fn test_dsrris(&self) -> bool {
        self.dsrris() != 0
    }

    #[doc="Sets the DSRRIS field."]
    #[inline] pub fn set_dsrris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Receive Raw Interrupt Status"]
    #[inline] pub fn rxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXRIS != 0"]
    #[inline] pub fn test_rxris(&self) -> bool {
        self.rxris() != 0
    }

    #[doc="Sets the RXRIS field."]
    #[inline] pub fn set_rxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Transmit Raw Interrupt Status"]
    #[inline] pub fn txris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXRIS != 0"]
    #[inline] pub fn test_txris(&self) -> bool {
        self.txris() != 0
    }

    #[doc="Sets the TXRIS field."]
    #[inline] pub fn set_txris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Receive Time-Out Raw Interrupt Status"]
    #[inline] pub fn rtris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTRIS != 0"]
    #[inline] pub fn test_rtris(&self) -> bool {
        self.rtris() != 0
    }

    #[doc="Sets the RTRIS field."]
    #[inline] pub fn set_rtris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Framing Error Raw Interrupt Status"]
    #[inline] pub fn feris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FERIS != 0"]
    #[inline] pub fn test_feris(&self) -> bool {
        self.feris() != 0
    }

    #[doc="Sets the FERIS field."]
    #[inline] pub fn set_feris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="UART Parity Error Raw Interrupt Status"]
    #[inline] pub fn peris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PERIS != 0"]
    #[inline] pub fn test_peris(&self) -> bool {
        self.peris() != 0
    }

    #[doc="Sets the PERIS field."]
    #[inline] pub fn set_peris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="UART Break Error Raw Interrupt Status"]
    #[inline] pub fn beris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BERIS != 0"]
    #[inline] pub fn test_beris(&self) -> bool {
        self.beris() != 0
    }

    #[doc="Sets the BERIS field."]
    #[inline] pub fn set_beris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="UART Overrun Error Raw Interrupt Status"]
    #[inline] pub fn oeris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OERIS != 0"]
    #[inline] pub fn test_oeris(&self) -> bool {
        self.oeris() != 0
    }

    #[doc="Sets the OERIS field."]
    #[inline] pub fn set_oeris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="End of Transmission Raw Interrupt Status"]
    #[inline] pub fn eotris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if EOTRIS != 0"]
    #[inline] pub fn test_eotris(&self) -> bool {
        self.eotris() != 0
    }

    #[doc="Sets the EOTRIS field."]
    #[inline] pub fn set_eotris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="9-Bit Mode Raw Interrupt Status"]
    #[inline] pub fn _9bitris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if _9BITRIS != 0"]
    #[inline] pub fn test_9bitris(&self) -> bool {
        self._9bitris() != 0
    }

    #[doc="Sets the _9BITRIS field."]
    #[inline] pub fn set_9bitris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receive DMA Raw Interrupt Status"]
    #[inline] pub fn dmarxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DMARXRIS != 0"]
    #[inline] pub fn test_dmarxris(&self) -> bool {
        self.dmarxris() != 0
    }

    #[doc="Sets the DMARXRIS field."]
    #[inline] pub fn set_dmarxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn dmatxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DMATXRIS != 0"]
    #[inline] pub fn test_dmatxris(&self) -> bool {
        self.dmatxris() != 0
    }

    #[doc="Sets the DMATXRIS field."]
    #[inline] pub fn set_dmatxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Ris {
    #[inline]
    fn from(other: u32) -> Self {
         Ris(other)
    }
}

impl ::core::fmt::Display for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.riris() != 0 { try!(write!(f, " riris"))}
        if self.ctsris() != 0 { try!(write!(f, " ctsris"))}
        if self.dcdris() != 0 { try!(write!(f, " dcdris"))}
        if self.dsrris() != 0 { try!(write!(f, " dsrris"))}
        if self.rxris() != 0 { try!(write!(f, " rxris"))}
        if self.txris() != 0 { try!(write!(f, " txris"))}
        if self.rtris() != 0 { try!(write!(f, " rtris"))}
        if self.feris() != 0 { try!(write!(f, " feris"))}
        if self.peris() != 0 { try!(write!(f, " peris"))}
        if self.beris() != 0 { try!(write!(f, " beris"))}
        if self.oeris() != 0 { try!(write!(f, " oeris"))}
        if self.eotris() != 0 { try!(write!(f, " eotris"))}
        if self._9bitris() != 0 { try!(write!(f, " _9bitris"))}
        if self.dmarxris() != 0 { try!(write!(f, " dmarxris"))}
        if self.dmatxris() != 0 { try!(write!(f, " dmatxris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Masked Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
    #[doc="UART Ring Indicator Modem Masked Interrupt Status"]
    #[inline] pub fn rimis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RIMIS != 0"]
    #[inline] pub fn test_rimis(&self) -> bool {
        self.rimis() != 0
    }

    #[doc="Sets the RIMIS field."]
    #[inline] pub fn set_rimis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Clear to Send Modem Masked Interrupt Status"]
    #[inline] pub fn ctsmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTSMIS != 0"]
    #[inline] pub fn test_ctsmis(&self) -> bool {
        self.ctsmis() != 0
    }

    #[doc="Sets the CTSMIS field."]
    #[inline] pub fn set_ctsmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Data Carrier Detect Modem Masked Interrupt Status"]
    #[inline] pub fn dcdmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCDMIS != 0"]
    #[inline] pub fn test_dcdmis(&self) -> bool {
        self.dcdmis() != 0
    }

    #[doc="Sets the DCDMIS field."]
    #[inline] pub fn set_dcdmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Data Set Ready Modem Masked Interrupt Status"]
    #[inline] pub fn dsrmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DSRMIS != 0"]
    #[inline] pub fn test_dsrmis(&self) -> bool {
        self.dsrmis() != 0
    }

    #[doc="Sets the DSRMIS field."]
    #[inline] pub fn set_dsrmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="UART Receive Masked Interrupt Status"]
    #[inline] pub fn rxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXMIS != 0"]
    #[inline] pub fn test_rxmis(&self) -> bool {
        self.rxmis() != 0
    }

    #[doc="Sets the RXMIS field."]
    #[inline] pub fn set_rxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="UART Transmit Masked Interrupt Status"]
    #[inline] pub fn txmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXMIS != 0"]
    #[inline] pub fn test_txmis(&self) -> bool {
        self.txmis() != 0
    }

    #[doc="Sets the TXMIS field."]
    #[inline] pub fn set_txmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Receive Time-Out Masked Interrupt Status"]
    #[inline] pub fn rtmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTMIS != 0"]
    #[inline] pub fn test_rtmis(&self) -> bool {
        self.rtmis() != 0
    }

    #[doc="Sets the RTMIS field."]
    #[inline] pub fn set_rtmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART Framing Error Masked Interrupt Status"]
    #[inline] pub fn femis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FEMIS != 0"]
    #[inline] pub fn test_femis(&self) -> bool {
        self.femis() != 0
    }

    #[doc="Sets the FEMIS field."]
    #[inline] pub fn set_femis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="UART Parity Error Masked Interrupt Status"]
    #[inline] pub fn pemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PEMIS != 0"]
    #[inline] pub fn test_pemis(&self) -> bool {
        self.pemis() != 0
    }

    #[doc="Sets the PEMIS field."]
    #[inline] pub fn set_pemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="UART Break Error Masked Interrupt Status"]
    #[inline] pub fn bemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BEMIS != 0"]
    #[inline] pub fn test_bemis(&self) -> bool {
        self.bemis() != 0
    }

    #[doc="Sets the BEMIS field."]
    #[inline] pub fn set_bemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="UART Overrun Error Masked Interrupt Status"]
    #[inline] pub fn oemis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OEMIS != 0"]
    #[inline] pub fn test_oemis(&self) -> bool {
        self.oemis() != 0
    }

    #[doc="Sets the OEMIS field."]
    #[inline] pub fn set_oemis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="End of Transmission Masked Interrupt Status"]
    #[inline] pub fn eotmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if EOTMIS != 0"]
    #[inline] pub fn test_eotmis(&self) -> bool {
        self.eotmis() != 0
    }

    #[doc="Sets the EOTMIS field."]
    #[inline] pub fn set_eotmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="9-Bit Mode Masked Interrupt Status"]
    #[inline] pub fn _9bitmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if _9BITMIS != 0"]
    #[inline] pub fn test_9bitmis(&self) -> bool {
        self._9bitmis() != 0
    }

    #[doc="Sets the _9BITMIS field."]
    #[inline] pub fn set_9bitmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receive DMA Masked Interrupt Status"]
    #[inline] pub fn dmarxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DMARXMIS != 0"]
    #[inline] pub fn test_dmarxmis(&self) -> bool {
        self.dmarxmis() != 0
    }

    #[doc="Sets the DMARXMIS field."]
    #[inline] pub fn set_dmarxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Transmit DMA Masked Interrupt Status"]
    #[inline] pub fn dmatxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DMATXMIS != 0"]
    #[inline] pub fn test_dmatxmis(&self) -> bool {
        self.dmatxmis() != 0
    }

    #[doc="Sets the DMATXMIS field."]
    #[inline] pub fn set_dmatxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Mis {
    #[inline]
    fn from(other: u32) -> Self {
         Mis(other)
    }
}

impl ::core::fmt::Display for Mis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rimis() != 0 { try!(write!(f, " rimis"))}
        if self.ctsmis() != 0 { try!(write!(f, " ctsmis"))}
        if self.dcdmis() != 0 { try!(write!(f, " dcdmis"))}
        if self.dsrmis() != 0 { try!(write!(f, " dsrmis"))}
        if self.rxmis() != 0 { try!(write!(f, " rxmis"))}
        if self.txmis() != 0 { try!(write!(f, " txmis"))}
        if self.rtmis() != 0 { try!(write!(f, " rtmis"))}
        if self.femis() != 0 { try!(write!(f, " femis"))}
        if self.pemis() != 0 { try!(write!(f, " pemis"))}
        if self.bemis() != 0 { try!(write!(f, " bemis"))}
        if self.oemis() != 0 { try!(write!(f, " oemis"))}
        if self.eotmis() != 0 { try!(write!(f, " eotmis"))}
        if self._9bitmis() != 0 { try!(write!(f, " _9bitmis"))}
        if self.dmarxmis() != 0 { try!(write!(f, " dmarxmis"))}
        if self.dmatxmis() != 0 { try!(write!(f, " dmatxmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Interrupt Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="UART Ring Indicator Modem Interrupt Clear"]
    #[inline] pub fn rimic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RIMIC != 0"]
    #[inline] pub fn test_rimic(&self) -> bool {
        self.rimic() != 0
    }

    #[doc="Sets the RIMIC field."]
    #[inline] pub fn set_rimic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="UART Clear to Send Modem Interrupt Clear"]
    #[inline] pub fn ctsmic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CTSMIC != 0"]
    #[inline] pub fn test_ctsmic(&self) -> bool {
        self.ctsmic() != 0
    }

    #[doc="Sets the CTSMIC field."]
    #[inline] pub fn set_ctsmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="UART Data Carrier Detect Modem Interrupt Clear"]
    #[inline] pub fn dcdmic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DCDMIC != 0"]
    #[inline] pub fn test_dcdmic(&self) -> bool {
        self.dcdmic() != 0
    }

    #[doc="Sets the DCDMIC field."]
    #[inline] pub fn set_dcdmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="UART Data Set Ready Modem Interrupt Clear"]
    #[inline] pub fn dsrmic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DSRMIC != 0"]
    #[inline] pub fn test_dsrmic(&self) -> bool {
        self.dsrmic() != 0
    }

    #[doc="Sets the DSRMIC field."]
    #[inline] pub fn set_dsrmic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receive Interrupt Clear"]
    #[inline] pub fn rxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXIC != 0"]
    #[inline] pub fn test_rxic(&self) -> bool {
        self.rxic() != 0
    }

    #[doc="Sets the RXIC field."]
    #[inline] pub fn set_rxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Transmit Interrupt Clear"]
    #[inline] pub fn txic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXIC != 0"]
    #[inline] pub fn test_txic(&self) -> bool {
        self.txic() != 0
    }

    #[doc="Sets the TXIC field."]
    #[inline] pub fn set_txic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Receive Time-Out Interrupt Clear"]
    #[inline] pub fn rtic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTIC != 0"]
    #[inline] pub fn test_rtic(&self) -> bool {
        self.rtic() != 0
    }

    #[doc="Sets the RTIC field."]
    #[inline] pub fn set_rtic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Framing Error Interrupt Clear"]
    #[inline] pub fn feic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FEIC != 0"]
    #[inline] pub fn test_feic(&self) -> bool {
        self.feic() != 0
    }

    #[doc="Sets the FEIC field."]
    #[inline] pub fn set_feic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Parity Error Interrupt Clear"]
    #[inline] pub fn peic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PEIC != 0"]
    #[inline] pub fn test_peic(&self) -> bool {
        self.peic() != 0
    }

    #[doc="Sets the PEIC field."]
    #[inline] pub fn set_peic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Break Error Interrupt Clear"]
    #[inline] pub fn beic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if BEIC != 0"]
    #[inline] pub fn test_beic(&self) -> bool {
        self.beic() != 0
    }

    #[doc="Sets the BEIC field."]
    #[inline] pub fn set_beic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Overrun Error Interrupt Clear"]
    #[inline] pub fn oeic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if OEIC != 0"]
    #[inline] pub fn test_oeic(&self) -> bool {
        self.oeic() != 0
    }

    #[doc="Sets the OEIC field."]
    #[inline] pub fn set_oeic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="End of Transmission Interrupt Clear"]
    #[inline] pub fn eotic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if EOTIC != 0"]
    #[inline] pub fn test_eotic(&self) -> bool {
        self.eotic() != 0
    }

    #[doc="Sets the EOTIC field."]
    #[inline] pub fn set_eotic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="9-Bit Mode Interrupt Clear"]
    #[inline] pub fn _9bitic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if _9BITIC != 0"]
    #[inline] pub fn test_9bitic(&self) -> bool {
        self._9bitic() != 0
    }

    #[doc="Sets the _9BITIC field."]
    #[inline] pub fn set_9bitic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Receive DMA Interrupt Clear"]
    #[inline] pub fn dmarxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DMARXIC != 0"]
    #[inline] pub fn test_dmarxic(&self) -> bool {
        self.dmarxic() != 0
    }

    #[doc="Sets the DMARXIC field."]
    #[inline] pub fn set_dmarxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Transmit DMA Interrupt Clear"]
    #[inline] pub fn dmatxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if DMATXIC != 0"]
    #[inline] pub fn test_dmatxic(&self) -> bool {
        self.dmatxic() != 0
    }

    #[doc="Sets the DMATXIC field."]
    #[inline] pub fn set_dmatxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
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
        if self.rimic() != 0 { try!(write!(f, " rimic"))}
        if self.ctsmic() != 0 { try!(write!(f, " ctsmic"))}
        if self.dcdmic() != 0 { try!(write!(f, " dcdmic"))}
        if self.dsrmic() != 0 { try!(write!(f, " dsrmic"))}
        if self.rxic() != 0 { try!(write!(f, " rxic"))}
        if self.txic() != 0 { try!(write!(f, " txic"))}
        if self.rtic() != 0 { try!(write!(f, " rtic"))}
        if self.feic() != 0 { try!(write!(f, " feic"))}
        if self.peic() != 0 { try!(write!(f, " peic"))}
        if self.beic() != 0 { try!(write!(f, " beic"))}
        if self.oeic() != 0 { try!(write!(f, " oeic"))}
        if self.eotic() != 0 { try!(write!(f, " eotic"))}
        if self._9bitic() != 0 { try!(write!(f, " _9bitic"))}
        if self.dmarxic() != 0 { try!(write!(f, " dmarxic"))}
        if self.dmatxic() != 0 { try!(write!(f, " dmatxic"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART DMA Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmactl(pub u32);
impl Dmactl {
    #[doc="Receive DMA Enable"]
    #[inline] pub fn rxdmae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXDMAE != 0"]
    #[inline] pub fn test_rxdmae(&self) -> bool {
        self.rxdmae() != 0
    }

    #[doc="Sets the RXDMAE field."]
    #[inline] pub fn set_rxdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit DMA Enable"]
    #[inline] pub fn txdmae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXDMAE != 0"]
    #[inline] pub fn test_txdmae(&self) -> bool {
        self.txdmae() != 0
    }

    #[doc="Sets the TXDMAE field."]
    #[inline] pub fn set_txdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DMA on Error"]
    #[inline] pub fn dmaerr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DMAERR != 0"]
    #[inline] pub fn test_dmaerr(&self) -> bool {
        self.dmaerr() != 0
    }

    #[doc="Sets the DMAERR field."]
    #[inline] pub fn set_dmaerr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Dmactl {
    #[inline]
    fn from(other: u32) -> Self {
         Dmactl(other)
    }
}

impl ::core::fmt::Display for Dmactl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmactl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdmae() != 0 { try!(write!(f, " rxdmae"))}
        if self.txdmae() != 0 { try!(write!(f, " txdmae"))}
        if self.dmaerr() != 0 { try!(write!(f, " dmaerr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 9-Bit Self Address"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct _9bitaddr(pub u32);
impl _9bitaddr {
    #[doc="Self Address for 9-Bit Mode"]
    #[inline] pub fn addr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ADDR != 0"]
    #[inline] pub fn test_addr(&self) -> bool {
        self.addr() != 0
    }

    #[doc="Sets the ADDR field."]
    #[inline] pub fn set_addr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Enable 9-Bit Mode"]
    #[inline] pub fn _9biten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if _9BITEN != 0"]
    #[inline] pub fn test_9biten(&self) -> bool {
        self._9biten() != 0
    }

    #[doc="Sets the _9BITEN field."]
    #[inline] pub fn set_9biten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for _9bitaddr {
    #[inline]
    fn from(other: u32) -> Self {
         _9bitaddr(other)
    }
}

impl ::core::fmt::Display for _9bitaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for _9bitaddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
        if self._9biten() != 0 { try!(write!(f, " _9biten"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART 9-Bit Self Address Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct _9bitamask(pub u32);
impl _9bitamask {
    #[doc="Self Address Mask for 9-Bit Mode"]
    #[inline] pub fn mask(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MASK != 0"]
    #[inline] pub fn test_mask(&self) -> bool {
        self.mask() != 0
    }

    #[doc="Sets the MASK field."]
    #[inline] pub fn set_mask<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for _9bitamask {
    #[inline]
    fn from(other: u32) -> Self {
         _9bitamask(other)
    }
}

impl ::core::fmt::Display for _9bitamask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for _9bitamask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mask() != 0 { try!(write!(f, " mask=0x{:x}", self.mask()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Peripheral Properties"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
    #[doc="Smart Card Support"]
    #[inline] pub fn sc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SC != 0"]
    #[inline] pub fn test_sc(&self) -> bool {
        self.sc() != 0
    }

    #[doc="Sets the SC field."]
    #[inline] pub fn set_sc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="9-Bit Support"]
    #[inline] pub fn nb(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if NB != 0"]
    #[inline] pub fn test_nb(&self) -> bool {
        self.nb() != 0
    }

    #[doc="Sets the NB field."]
    #[inline] pub fn set_nb<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Modem Support"]
    #[inline] pub fn ms(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MS != 0"]
    #[inline] pub fn test_ms(&self) -> bool {
        self.ms() != 0
    }

    #[doc="Sets the MS field."]
    #[inline] pub fn set_ms<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Modem Support Extended"]
    #[inline] pub fn mse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MSE != 0"]
    #[inline] pub fn test_mse(&self) -> bool {
        self.mse() != 0
    }

    #[doc="Sets the MSE field."]
    #[inline] pub fn set_mse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Pp {
    #[inline]
    fn from(other: u32) -> Self {
         Pp(other)
    }
}

impl ::core::fmt::Display for Pp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sc() != 0 { try!(write!(f, " sc"))}
        if self.nb() != 0 { try!(write!(f, " nb"))}
        if self.ms() != 0 { try!(write!(f, " ms"))}
        if self.mse() != 0 { try!(write!(f, " mse"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Clock Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc="UART Baud Clock Source"]
    #[inline] pub fn cs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CS != 0"]
    #[inline] pub fn test_cs(&self) -> bool {
        self.cs() != 0
    }

    #[doc="Sets the CS field."]
    #[inline] pub fn set_cs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
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
        if self.cs() != 0 { try!(write!(f, " cs=0x{:x}", self.cs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

