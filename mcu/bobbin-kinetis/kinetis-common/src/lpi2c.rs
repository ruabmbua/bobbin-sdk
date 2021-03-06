
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="LPI2C Peripheral"]
pub struct Lpi2cPeriph(pub usize); 

impl Lpi2cPeriph {
    #[doc="Get the VERID Register."]
    #[inline] pub fn verid_reg(&self) -> ::bobbin_mcu::register::Register<Verid> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Verid, 0x0)
    }

    #[doc="Get the *mut pointer for the VERID register."]
    #[inline] pub fn verid_mut(&self) -> *mut Verid { 
        self.verid_reg().ptr()
    }

    #[doc="Get the *const pointer for the VERID register."]
    #[inline] pub fn verid_ptr(&self) -> *const Verid { 
        self.verid_reg().ptr()
    }

    #[doc="Read the VERID register."]
    #[inline] pub fn verid(&self) -> Verid { 
        self.verid_reg().read()
    }

    #[doc="Get the PARAM Register."]
    #[inline] pub fn param_reg(&self) -> ::bobbin_mcu::register::Register<Param> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Param, 0x4)
    }

    #[doc="Get the *mut pointer for the PARAM register."]
    #[inline] pub fn param_mut(&self) -> *mut Param { 
        self.param_reg().ptr()
    }

    #[doc="Get the *const pointer for the PARAM register."]
    #[inline] pub fn param_ptr(&self) -> *const Param { 
        self.param_reg().ptr()
    }

    #[doc="Read the PARAM register."]
    #[inline] pub fn param(&self) -> Param { 
        self.param_reg().read()
    }

    #[doc="Get the MCR Register."]
    #[inline] pub fn mcr_reg(&self) -> ::bobbin_mcu::register::Register<Mcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mcr, 0x10)
    }

    #[doc="Get the *mut pointer for the MCR register."]
    #[inline] pub fn mcr_mut(&self) -> *mut Mcr { 
        self.mcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCR register."]
    #[inline] pub fn mcr_ptr(&self) -> *const Mcr { 
        self.mcr_reg().ptr()
    }

    #[doc="Read the MCR register."]
    #[inline] pub fn mcr(&self) -> Mcr { 
        self.mcr_reg().read()
    }

    #[doc="Write the MCR register."]
    #[inline] pub fn write_mcr(&self, value: Mcr) -> &Self { 
        self.mcr_reg().write(value);
        self
    }

    #[doc="Set the MCR register."]
    #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        self.mcr_reg().set(f);
        self
    }

    #[doc="Modify the MCR register."]
    #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        self.mcr_reg().with(f);
        self
    }

    #[doc="Get the MSR Register."]
    #[inline] pub fn msr_reg(&self) -> ::bobbin_mcu::register::Register<Msr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Msr, 0x14)
    }

    #[doc="Get the *mut pointer for the MSR register."]
    #[inline] pub fn msr_mut(&self) -> *mut Msr { 
        self.msr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MSR register."]
    #[inline] pub fn msr_ptr(&self) -> *const Msr { 
        self.msr_reg().ptr()
    }

    #[doc="Read the MSR register."]
    #[inline] pub fn msr(&self) -> Msr { 
        self.msr_reg().read()
    }

    #[doc="Write the MSR register."]
    #[inline] pub fn write_msr(&self, value: Msr) -> &Self { 
        self.msr_reg().write(value);
        self
    }

    #[doc="Set the MSR register."]
    #[inline] pub fn set_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
        self.msr_reg().set(f);
        self
    }

    #[doc="Modify the MSR register."]
    #[inline] pub fn with_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
        self.msr_reg().with(f);
        self
    }

    #[doc="Get the MIER Register."]
    #[inline] pub fn mier_reg(&self) -> ::bobbin_mcu::register::Register<Mier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mier, 0x18)
    }

    #[doc="Get the *mut pointer for the MIER register."]
    #[inline] pub fn mier_mut(&self) -> *mut Mier { 
        self.mier_reg().ptr()
    }

    #[doc="Get the *const pointer for the MIER register."]
    #[inline] pub fn mier_ptr(&self) -> *const Mier { 
        self.mier_reg().ptr()
    }

    #[doc="Read the MIER register."]
    #[inline] pub fn mier(&self) -> Mier { 
        self.mier_reg().read()
    }

    #[doc="Write the MIER register."]
    #[inline] pub fn write_mier(&self, value: Mier) -> &Self { 
        self.mier_reg().write(value);
        self
    }

    #[doc="Set the MIER register."]
    #[inline] pub fn set_mier<F: FnOnce(Mier) -> Mier>(&self, f: F) -> &Self {
        self.mier_reg().set(f);
        self
    }

    #[doc="Modify the MIER register."]
    #[inline] pub fn with_mier<F: FnOnce(Mier) -> Mier>(&self, f: F) -> &Self {
        self.mier_reg().with(f);
        self
    }

    #[doc="Get the MDER Register."]
    #[inline] pub fn mder_reg(&self) -> ::bobbin_mcu::register::Register<Mder> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mder, 0x1c)
    }

    #[doc="Get the *mut pointer for the MDER register."]
    #[inline] pub fn mder_mut(&self) -> *mut Mder { 
        self.mder_reg().ptr()
    }

    #[doc="Get the *const pointer for the MDER register."]
    #[inline] pub fn mder_ptr(&self) -> *const Mder { 
        self.mder_reg().ptr()
    }

    #[doc="Read the MDER register."]
    #[inline] pub fn mder(&self) -> Mder { 
        self.mder_reg().read()
    }

    #[doc="Write the MDER register."]
    #[inline] pub fn write_mder(&self, value: Mder) -> &Self { 
        self.mder_reg().write(value);
        self
    }

    #[doc="Set the MDER register."]
    #[inline] pub fn set_mder<F: FnOnce(Mder) -> Mder>(&self, f: F) -> &Self {
        self.mder_reg().set(f);
        self
    }

    #[doc="Modify the MDER register."]
    #[inline] pub fn with_mder<F: FnOnce(Mder) -> Mder>(&self, f: F) -> &Self {
        self.mder_reg().with(f);
        self
    }

    #[doc="Get the MCFGR0 Register."]
    #[inline] pub fn mcfgr0_reg(&self) -> ::bobbin_mcu::register::Register<Mcfgr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mcfgr0, 0x20)
    }

    #[doc="Get the *mut pointer for the MCFGR0 register."]
    #[inline] pub fn mcfgr0_mut(&self) -> *mut Mcfgr0 { 
        self.mcfgr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCFGR0 register."]
    #[inline] pub fn mcfgr0_ptr(&self) -> *const Mcfgr0 { 
        self.mcfgr0_reg().ptr()
    }

    #[doc="Read the MCFGR0 register."]
    #[inline] pub fn mcfgr0(&self) -> Mcfgr0 { 
        self.mcfgr0_reg().read()
    }

    #[doc="Write the MCFGR0 register."]
    #[inline] pub fn write_mcfgr0(&self, value: Mcfgr0) -> &Self { 
        self.mcfgr0_reg().write(value);
        self
    }

    #[doc="Set the MCFGR0 register."]
    #[inline] pub fn set_mcfgr0<F: FnOnce(Mcfgr0) -> Mcfgr0>(&self, f: F) -> &Self {
        self.mcfgr0_reg().set(f);
        self
    }

    #[doc="Modify the MCFGR0 register."]
    #[inline] pub fn with_mcfgr0<F: FnOnce(Mcfgr0) -> Mcfgr0>(&self, f: F) -> &Self {
        self.mcfgr0_reg().with(f);
        self
    }

    #[doc="Get the MCFGR1 Register."]
    #[inline] pub fn mcfgr1_reg(&self) -> ::bobbin_mcu::register::Register<Mcfgr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mcfgr1, 0x24)
    }

    #[doc="Get the *mut pointer for the MCFGR1 register."]
    #[inline] pub fn mcfgr1_mut(&self) -> *mut Mcfgr1 { 
        self.mcfgr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCFGR1 register."]
    #[inline] pub fn mcfgr1_ptr(&self) -> *const Mcfgr1 { 
        self.mcfgr1_reg().ptr()
    }

    #[doc="Read the MCFGR1 register."]
    #[inline] pub fn mcfgr1(&self) -> Mcfgr1 { 
        self.mcfgr1_reg().read()
    }

    #[doc="Write the MCFGR1 register."]
    #[inline] pub fn write_mcfgr1(&self, value: Mcfgr1) -> &Self { 
        self.mcfgr1_reg().write(value);
        self
    }

    #[doc="Set the MCFGR1 register."]
    #[inline] pub fn set_mcfgr1<F: FnOnce(Mcfgr1) -> Mcfgr1>(&self, f: F) -> &Self {
        self.mcfgr1_reg().set(f);
        self
    }

    #[doc="Modify the MCFGR1 register."]
    #[inline] pub fn with_mcfgr1<F: FnOnce(Mcfgr1) -> Mcfgr1>(&self, f: F) -> &Self {
        self.mcfgr1_reg().with(f);
        self
    }

    #[doc="Get the MCFGR2 Register."]
    #[inline] pub fn mcfgr2_reg(&self) -> ::bobbin_mcu::register::Register<Mcfgr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mcfgr2, 0x28)
    }

    #[doc="Get the *mut pointer for the MCFGR2 register."]
    #[inline] pub fn mcfgr2_mut(&self) -> *mut Mcfgr2 { 
        self.mcfgr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCFGR2 register."]
    #[inline] pub fn mcfgr2_ptr(&self) -> *const Mcfgr2 { 
        self.mcfgr2_reg().ptr()
    }

    #[doc="Read the MCFGR2 register."]
    #[inline] pub fn mcfgr2(&self) -> Mcfgr2 { 
        self.mcfgr2_reg().read()
    }

    #[doc="Write the MCFGR2 register."]
    #[inline] pub fn write_mcfgr2(&self, value: Mcfgr2) -> &Self { 
        self.mcfgr2_reg().write(value);
        self
    }

    #[doc="Set the MCFGR2 register."]
    #[inline] pub fn set_mcfgr2<F: FnOnce(Mcfgr2) -> Mcfgr2>(&self, f: F) -> &Self {
        self.mcfgr2_reg().set(f);
        self
    }

    #[doc="Modify the MCFGR2 register."]
    #[inline] pub fn with_mcfgr2<F: FnOnce(Mcfgr2) -> Mcfgr2>(&self, f: F) -> &Self {
        self.mcfgr2_reg().with(f);
        self
    }

    #[doc="Get the MCFGR3 Register."]
    #[inline] pub fn mcfgr3_reg(&self) -> ::bobbin_mcu::register::Register<Mcfgr3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mcfgr3, 0x2c)
    }

    #[doc="Get the *mut pointer for the MCFGR3 register."]
    #[inline] pub fn mcfgr3_mut(&self) -> *mut Mcfgr3 { 
        self.mcfgr3_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCFGR3 register."]
    #[inline] pub fn mcfgr3_ptr(&self) -> *const Mcfgr3 { 
        self.mcfgr3_reg().ptr()
    }

    #[doc="Read the MCFGR3 register."]
    #[inline] pub fn mcfgr3(&self) -> Mcfgr3 { 
        self.mcfgr3_reg().read()
    }

    #[doc="Write the MCFGR3 register."]
    #[inline] pub fn write_mcfgr3(&self, value: Mcfgr3) -> &Self { 
        self.mcfgr3_reg().write(value);
        self
    }

    #[doc="Set the MCFGR3 register."]
    #[inline] pub fn set_mcfgr3<F: FnOnce(Mcfgr3) -> Mcfgr3>(&self, f: F) -> &Self {
        self.mcfgr3_reg().set(f);
        self
    }

    #[doc="Modify the MCFGR3 register."]
    #[inline] pub fn with_mcfgr3<F: FnOnce(Mcfgr3) -> Mcfgr3>(&self, f: F) -> &Self {
        self.mcfgr3_reg().with(f);
        self
    }

    #[doc="Get the MDMR Register."]
    #[inline] pub fn mdmr_reg(&self) -> ::bobbin_mcu::register::Register<Mdmr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mdmr, 0x40)
    }

    #[doc="Get the *mut pointer for the MDMR register."]
    #[inline] pub fn mdmr_mut(&self) -> *mut Mdmr { 
        self.mdmr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MDMR register."]
    #[inline] pub fn mdmr_ptr(&self) -> *const Mdmr { 
        self.mdmr_reg().ptr()
    }

    #[doc="Read the MDMR register."]
    #[inline] pub fn mdmr(&self) -> Mdmr { 
        self.mdmr_reg().read()
    }

    #[doc="Write the MDMR register."]
    #[inline] pub fn write_mdmr(&self, value: Mdmr) -> &Self { 
        self.mdmr_reg().write(value);
        self
    }

    #[doc="Set the MDMR register."]
    #[inline] pub fn set_mdmr<F: FnOnce(Mdmr) -> Mdmr>(&self, f: F) -> &Self {
        self.mdmr_reg().set(f);
        self
    }

    #[doc="Modify the MDMR register."]
    #[inline] pub fn with_mdmr<F: FnOnce(Mdmr) -> Mdmr>(&self, f: F) -> &Self {
        self.mdmr_reg().with(f);
        self
    }

    #[doc="Get the MCCR0 Register."]
    #[inline] pub fn mccr0_reg(&self) -> ::bobbin_mcu::register::Register<Mccr0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mccr0, 0x48)
    }

    #[doc="Get the *mut pointer for the MCCR0 register."]
    #[inline] pub fn mccr0_mut(&self) -> *mut Mccr0 { 
        self.mccr0_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCCR0 register."]
    #[inline] pub fn mccr0_ptr(&self) -> *const Mccr0 { 
        self.mccr0_reg().ptr()
    }

    #[doc="Read the MCCR0 register."]
    #[inline] pub fn mccr0(&self) -> Mccr0 { 
        self.mccr0_reg().read()
    }

    #[doc="Write the MCCR0 register."]
    #[inline] pub fn write_mccr0(&self, value: Mccr0) -> &Self { 
        self.mccr0_reg().write(value);
        self
    }

    #[doc="Set the MCCR0 register."]
    #[inline] pub fn set_mccr0<F: FnOnce(Mccr0) -> Mccr0>(&self, f: F) -> &Self {
        self.mccr0_reg().set(f);
        self
    }

    #[doc="Modify the MCCR0 register."]
    #[inline] pub fn with_mccr0<F: FnOnce(Mccr0) -> Mccr0>(&self, f: F) -> &Self {
        self.mccr0_reg().with(f);
        self
    }

    #[doc="Get the MCCR1 Register."]
    #[inline] pub fn mccr1_reg(&self) -> ::bobbin_mcu::register::Register<Mccr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mccr1, 0x50)
    }

    #[doc="Get the *mut pointer for the MCCR1 register."]
    #[inline] pub fn mccr1_mut(&self) -> *mut Mccr1 { 
        self.mccr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the MCCR1 register."]
    #[inline] pub fn mccr1_ptr(&self) -> *const Mccr1 { 
        self.mccr1_reg().ptr()
    }

    #[doc="Read the MCCR1 register."]
    #[inline] pub fn mccr1(&self) -> Mccr1 { 
        self.mccr1_reg().read()
    }

    #[doc="Write the MCCR1 register."]
    #[inline] pub fn write_mccr1(&self, value: Mccr1) -> &Self { 
        self.mccr1_reg().write(value);
        self
    }

    #[doc="Set the MCCR1 register."]
    #[inline] pub fn set_mccr1<F: FnOnce(Mccr1) -> Mccr1>(&self, f: F) -> &Self {
        self.mccr1_reg().set(f);
        self
    }

    #[doc="Modify the MCCR1 register."]
    #[inline] pub fn with_mccr1<F: FnOnce(Mccr1) -> Mccr1>(&self, f: F) -> &Self {
        self.mccr1_reg().with(f);
        self
    }

    #[doc="Get the MFCR Register."]
    #[inline] pub fn mfcr_reg(&self) -> ::bobbin_mcu::register::Register<Mfcr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mfcr, 0x58)
    }

    #[doc="Get the *mut pointer for the MFCR register."]
    #[inline] pub fn mfcr_mut(&self) -> *mut Mfcr { 
        self.mfcr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MFCR register."]
    #[inline] pub fn mfcr_ptr(&self) -> *const Mfcr { 
        self.mfcr_reg().ptr()
    }

    #[doc="Read the MFCR register."]
    #[inline] pub fn mfcr(&self) -> Mfcr { 
        self.mfcr_reg().read()
    }

    #[doc="Write the MFCR register."]
    #[inline] pub fn write_mfcr(&self, value: Mfcr) -> &Self { 
        self.mfcr_reg().write(value);
        self
    }

    #[doc="Set the MFCR register."]
    #[inline] pub fn set_mfcr<F: FnOnce(Mfcr) -> Mfcr>(&self, f: F) -> &Self {
        self.mfcr_reg().set(f);
        self
    }

    #[doc="Modify the MFCR register."]
    #[inline] pub fn with_mfcr<F: FnOnce(Mfcr) -> Mfcr>(&self, f: F) -> &Self {
        self.mfcr_reg().with(f);
        self
    }

    #[doc="Get the MFSR Register."]
    #[inline] pub fn mfsr_reg(&self) -> ::bobbin_mcu::register::Register<Mfsr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mfsr, 0x5c)
    }

    #[doc="Get the *mut pointer for the MFSR register."]
    #[inline] pub fn mfsr_mut(&self) -> *mut Mfsr { 
        self.mfsr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MFSR register."]
    #[inline] pub fn mfsr_ptr(&self) -> *const Mfsr { 
        self.mfsr_reg().ptr()
    }

    #[doc="Read the MFSR register."]
    #[inline] pub fn mfsr(&self) -> Mfsr { 
        self.mfsr_reg().read()
    }

    #[doc="Get the MTDR Register."]
    #[inline] pub fn mtdr_reg(&self) -> ::bobbin_mcu::register::Register<Mtdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mtdr, 0x60)
    }

    #[doc="Get the *mut pointer for the MTDR register."]
    #[inline] pub fn mtdr_mut(&self) -> *mut Mtdr { 
        self.mtdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MTDR register."]
    #[inline] pub fn mtdr_ptr(&self) -> *const Mtdr { 
        self.mtdr_reg().ptr()
    }

    #[doc="Read the MTDR register."]
    #[inline] pub fn mtdr(&self) -> Mtdr { 
        self.mtdr_reg().read()
    }

    #[doc="Write the MTDR register."]
    #[inline] pub fn write_mtdr(&self, value: Mtdr) -> &Self { 
        self.mtdr_reg().write(value);
        self
    }

    #[doc="Set the MTDR register."]
    #[inline] pub fn set_mtdr<F: FnOnce(Mtdr) -> Mtdr>(&self, f: F) -> &Self {
        self.mtdr_reg().set(f);
        self
    }

    #[doc="Modify the MTDR register."]
    #[inline] pub fn with_mtdr<F: FnOnce(Mtdr) -> Mtdr>(&self, f: F) -> &Self {
        self.mtdr_reg().with(f);
        self
    }

    #[doc="Get the MRDR Register."]
    #[inline] pub fn mrdr_reg(&self) -> ::bobbin_mcu::register::Register<Mrdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mrdr, 0x70)
    }

    #[doc="Get the *mut pointer for the MRDR register."]
    #[inline] pub fn mrdr_mut(&self) -> *mut Mrdr { 
        self.mrdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the MRDR register."]
    #[inline] pub fn mrdr_ptr(&self) -> *const Mrdr { 
        self.mrdr_reg().ptr()
    }

    #[doc="Read the MRDR register."]
    #[inline] pub fn mrdr(&self) -> Mrdr { 
        self.mrdr_reg().read()
    }

    #[doc="Get the SCR Register."]
    #[inline] pub fn scr_reg(&self) -> ::bobbin_mcu::register::Register<Scr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Scr, 0x110)
    }

    #[doc="Get the *mut pointer for the SCR register."]
    #[inline] pub fn scr_mut(&self) -> *mut Scr { 
        self.scr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SCR register."]
    #[inline] pub fn scr_ptr(&self) -> *const Scr { 
        self.scr_reg().ptr()
    }

    #[doc="Read the SCR register."]
    #[inline] pub fn scr(&self) -> Scr { 
        self.scr_reg().read()
    }

    #[doc="Write the SCR register."]
    #[inline] pub fn write_scr(&self, value: Scr) -> &Self { 
        self.scr_reg().write(value);
        self
    }

    #[doc="Set the SCR register."]
    #[inline] pub fn set_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
        self.scr_reg().set(f);
        self
    }

    #[doc="Modify the SCR register."]
    #[inline] pub fn with_scr<F: FnOnce(Scr) -> Scr>(&self, f: F) -> &Self {
        self.scr_reg().with(f);
        self
    }

    #[doc="Get the SSR Register."]
    #[inline] pub fn ssr_reg(&self) -> ::bobbin_mcu::register::Register<Ssr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ssr, 0x114)
    }

    #[doc="Get the *mut pointer for the SSR register."]
    #[inline] pub fn ssr_mut(&self) -> *mut Ssr { 
        self.ssr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SSR register."]
    #[inline] pub fn ssr_ptr(&self) -> *const Ssr { 
        self.ssr_reg().ptr()
    }

    #[doc="Read the SSR register."]
    #[inline] pub fn ssr(&self) -> Ssr { 
        self.ssr_reg().read()
    }

    #[doc="Write the SSR register."]
    #[inline] pub fn write_ssr(&self, value: Ssr) -> &Self { 
        self.ssr_reg().write(value);
        self
    }

    #[doc="Set the SSR register."]
    #[inline] pub fn set_ssr<F: FnOnce(Ssr) -> Ssr>(&self, f: F) -> &Self {
        self.ssr_reg().set(f);
        self
    }

    #[doc="Modify the SSR register."]
    #[inline] pub fn with_ssr<F: FnOnce(Ssr) -> Ssr>(&self, f: F) -> &Self {
        self.ssr_reg().with(f);
        self
    }

    #[doc="Get the SIER Register."]
    #[inline] pub fn sier_reg(&self) -> ::bobbin_mcu::register::Register<Sier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sier, 0x118)
    }

    #[doc="Get the *mut pointer for the SIER register."]
    #[inline] pub fn sier_mut(&self) -> *mut Sier { 
        self.sier_reg().ptr()
    }

    #[doc="Get the *const pointer for the SIER register."]
    #[inline] pub fn sier_ptr(&self) -> *const Sier { 
        self.sier_reg().ptr()
    }

    #[doc="Read the SIER register."]
    #[inline] pub fn sier(&self) -> Sier { 
        self.sier_reg().read()
    }

    #[doc="Write the SIER register."]
    #[inline] pub fn write_sier(&self, value: Sier) -> &Self { 
        self.sier_reg().write(value);
        self
    }

    #[doc="Set the SIER register."]
    #[inline] pub fn set_sier<F: FnOnce(Sier) -> Sier>(&self, f: F) -> &Self {
        self.sier_reg().set(f);
        self
    }

    #[doc="Modify the SIER register."]
    #[inline] pub fn with_sier<F: FnOnce(Sier) -> Sier>(&self, f: F) -> &Self {
        self.sier_reg().with(f);
        self
    }

    #[doc="Get the SDER Register."]
    #[inline] pub fn sder_reg(&self) -> ::bobbin_mcu::register::Register<Sder> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sder, 0x11c)
    }

    #[doc="Get the *mut pointer for the SDER register."]
    #[inline] pub fn sder_mut(&self) -> *mut Sder { 
        self.sder_reg().ptr()
    }

    #[doc="Get the *const pointer for the SDER register."]
    #[inline] pub fn sder_ptr(&self) -> *const Sder { 
        self.sder_reg().ptr()
    }

    #[doc="Read the SDER register."]
    #[inline] pub fn sder(&self) -> Sder { 
        self.sder_reg().read()
    }

    #[doc="Write the SDER register."]
    #[inline] pub fn write_sder(&self, value: Sder) -> &Self { 
        self.sder_reg().write(value);
        self
    }

    #[doc="Set the SDER register."]
    #[inline] pub fn set_sder<F: FnOnce(Sder) -> Sder>(&self, f: F) -> &Self {
        self.sder_reg().set(f);
        self
    }

    #[doc="Modify the SDER register."]
    #[inline] pub fn with_sder<F: FnOnce(Sder) -> Sder>(&self, f: F) -> &Self {
        self.sder_reg().with(f);
        self
    }

    #[doc="Get the SCFGR1 Register."]
    #[inline] pub fn scfgr1_reg(&self) -> ::bobbin_mcu::register::Register<Scfgr1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Scfgr1, 0x124)
    }

    #[doc="Get the *mut pointer for the SCFGR1 register."]
    #[inline] pub fn scfgr1_mut(&self) -> *mut Scfgr1 { 
        self.scfgr1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SCFGR1 register."]
    #[inline] pub fn scfgr1_ptr(&self) -> *const Scfgr1 { 
        self.scfgr1_reg().ptr()
    }

    #[doc="Read the SCFGR1 register."]
    #[inline] pub fn scfgr1(&self) -> Scfgr1 { 
        self.scfgr1_reg().read()
    }

    #[doc="Write the SCFGR1 register."]
    #[inline] pub fn write_scfgr1(&self, value: Scfgr1) -> &Self { 
        self.scfgr1_reg().write(value);
        self
    }

    #[doc="Set the SCFGR1 register."]
    #[inline] pub fn set_scfgr1<F: FnOnce(Scfgr1) -> Scfgr1>(&self, f: F) -> &Self {
        self.scfgr1_reg().set(f);
        self
    }

    #[doc="Modify the SCFGR1 register."]
    #[inline] pub fn with_scfgr1<F: FnOnce(Scfgr1) -> Scfgr1>(&self, f: F) -> &Self {
        self.scfgr1_reg().with(f);
        self
    }

    #[doc="Get the SCFGR2 Register."]
    #[inline] pub fn scfgr2_reg(&self) -> ::bobbin_mcu::register::Register<Scfgr2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Scfgr2, 0x128)
    }

    #[doc="Get the *mut pointer for the SCFGR2 register."]
    #[inline] pub fn scfgr2_mut(&self) -> *mut Scfgr2 { 
        self.scfgr2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SCFGR2 register."]
    #[inline] pub fn scfgr2_ptr(&self) -> *const Scfgr2 { 
        self.scfgr2_reg().ptr()
    }

    #[doc="Read the SCFGR2 register."]
    #[inline] pub fn scfgr2(&self) -> Scfgr2 { 
        self.scfgr2_reg().read()
    }

    #[doc="Write the SCFGR2 register."]
    #[inline] pub fn write_scfgr2(&self, value: Scfgr2) -> &Self { 
        self.scfgr2_reg().write(value);
        self
    }

    #[doc="Set the SCFGR2 register."]
    #[inline] pub fn set_scfgr2<F: FnOnce(Scfgr2) -> Scfgr2>(&self, f: F) -> &Self {
        self.scfgr2_reg().set(f);
        self
    }

    #[doc="Modify the SCFGR2 register."]
    #[inline] pub fn with_scfgr2<F: FnOnce(Scfgr2) -> Scfgr2>(&self, f: F) -> &Self {
        self.scfgr2_reg().with(f);
        self
    }

    #[doc="Get the SAMR Register."]
    #[inline] pub fn samr_reg(&self) -> ::bobbin_mcu::register::Register<Samr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Samr, 0x140)
    }

    #[doc="Get the *mut pointer for the SAMR register."]
    #[inline] pub fn samr_mut(&self) -> *mut Samr { 
        self.samr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SAMR register."]
    #[inline] pub fn samr_ptr(&self) -> *const Samr { 
        self.samr_reg().ptr()
    }

    #[doc="Read the SAMR register."]
    #[inline] pub fn samr(&self) -> Samr { 
        self.samr_reg().read()
    }

    #[doc="Write the SAMR register."]
    #[inline] pub fn write_samr(&self, value: Samr) -> &Self { 
        self.samr_reg().write(value);
        self
    }

    #[doc="Set the SAMR register."]
    #[inline] pub fn set_samr<F: FnOnce(Samr) -> Samr>(&self, f: F) -> &Self {
        self.samr_reg().set(f);
        self
    }

    #[doc="Modify the SAMR register."]
    #[inline] pub fn with_samr<F: FnOnce(Samr) -> Samr>(&self, f: F) -> &Self {
        self.samr_reg().with(f);
        self
    }

    #[doc="Get the SASR Register."]
    #[inline] pub fn sasr_reg(&self) -> ::bobbin_mcu::register::Register<Sasr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sasr, 0x150)
    }

    #[doc="Get the *mut pointer for the SASR register."]
    #[inline] pub fn sasr_mut(&self) -> *mut Sasr { 
        self.sasr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SASR register."]
    #[inline] pub fn sasr_ptr(&self) -> *const Sasr { 
        self.sasr_reg().ptr()
    }

    #[doc="Read the SASR register."]
    #[inline] pub fn sasr(&self) -> Sasr { 
        self.sasr_reg().read()
    }

    #[doc="Get the STAR Register."]
    #[inline] pub fn star_reg(&self) -> ::bobbin_mcu::register::Register<Star> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Star, 0x154)
    }

    #[doc="Get the *mut pointer for the STAR register."]
    #[inline] pub fn star_mut(&self) -> *mut Star { 
        self.star_reg().ptr()
    }

    #[doc="Get the *const pointer for the STAR register."]
    #[inline] pub fn star_ptr(&self) -> *const Star { 
        self.star_reg().ptr()
    }

    #[doc="Read the STAR register."]
    #[inline] pub fn star(&self) -> Star { 
        self.star_reg().read()
    }

    #[doc="Write the STAR register."]
    #[inline] pub fn write_star(&self, value: Star) -> &Self { 
        self.star_reg().write(value);
        self
    }

    #[doc="Set the STAR register."]
    #[inline] pub fn set_star<F: FnOnce(Star) -> Star>(&self, f: F) -> &Self {
        self.star_reg().set(f);
        self
    }

    #[doc="Modify the STAR register."]
    #[inline] pub fn with_star<F: FnOnce(Star) -> Star>(&self, f: F) -> &Self {
        self.star_reg().with(f);
        self
    }

    #[doc="Get the STDR Register."]
    #[inline] pub fn stdr_reg(&self) -> ::bobbin_mcu::register::Register<Stdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Stdr, 0x160)
    }

    #[doc="Get the *mut pointer for the STDR register."]
    #[inline] pub fn stdr_mut(&self) -> *mut Stdr { 
        self.stdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the STDR register."]
    #[inline] pub fn stdr_ptr(&self) -> *const Stdr { 
        self.stdr_reg().ptr()
    }

    #[doc="Read the STDR register."]
    #[inline] pub fn stdr(&self) -> Stdr { 
        self.stdr_reg().read()
    }

    #[doc="Write the STDR register."]
    #[inline] pub fn write_stdr(&self, value: Stdr) -> &Self { 
        self.stdr_reg().write(value);
        self
    }

    #[doc="Set the STDR register."]
    #[inline] pub fn set_stdr<F: FnOnce(Stdr) -> Stdr>(&self, f: F) -> &Self {
        self.stdr_reg().set(f);
        self
    }

    #[doc="Modify the STDR register."]
    #[inline] pub fn with_stdr<F: FnOnce(Stdr) -> Stdr>(&self, f: F) -> &Self {
        self.stdr_reg().with(f);
        self
    }

    #[doc="Get the SRDR Register."]
    #[inline] pub fn srdr_reg(&self) -> ::bobbin_mcu::register::Register<Srdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Srdr, 0x170)
    }

    #[doc="Get the *mut pointer for the SRDR register."]
    #[inline] pub fn srdr_mut(&self) -> *mut Srdr { 
        self.srdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SRDR register."]
    #[inline] pub fn srdr_ptr(&self) -> *const Srdr { 
        self.srdr_reg().ptr()
    }

    #[doc="Read the SRDR register."]
    #[inline] pub fn srdr(&self) -> Srdr { 
        self.srdr_reg().read()
    }

}

#[doc="Version ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc="Feature Specification Number"]
    #[inline] pub fn feature(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if FEATURE != 0"]
    #[inline] pub fn test_feature(&self) -> bool {
        self.feature() != 0
    }

    #[doc="Sets the FEATURE field."]
    #[inline] pub fn set_feature<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Minor Version Number"]
    #[inline] pub fn minor(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MINOR != 0"]
    #[inline] pub fn test_minor(&self) -> bool {
        self.minor() != 0
    }

    #[doc="Sets the MINOR field."]
    #[inline] pub fn set_minor<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Major Version Number"]
    #[inline] pub fn major(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if MAJOR != 0"]
    #[inline] pub fn test_major(&self) -> bool {
        self.major() != 0
    }

    #[doc="Sets the MAJOR field."]
    #[inline] pub fn set_major<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Verid {
    #[inline]
    fn from(other: u32) -> Self {
         Verid(other)
    }
}

impl ::core::fmt::Display for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.feature() != 0 { try!(write!(f, " feature=0x{:x}", self.feature()))}
        if self.minor() != 0 { try!(write!(f, " minor=0x{:x}", self.minor()))}
        if self.major() != 0 { try!(write!(f, " major=0x{:x}", self.major()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Parameter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Param(pub u32);
impl Param {
    #[doc="Master Transmit FIFO Size"]
    #[inline] pub fn mtxfifo(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if MTXFIFO != 0"]
    #[inline] pub fn test_mtxfifo(&self) -> bool {
        self.mtxfifo() != 0
    }

    #[doc="Sets the MTXFIFO field."]
    #[inline] pub fn set_mtxfifo<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Master Receive FIFO Size"]
    #[inline] pub fn mrxfifo(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MRXFIFO != 0"]
    #[inline] pub fn test_mrxfifo(&self) -> bool {
        self.mrxfifo() != 0
    }

    #[doc="Sets the MRXFIFO field."]
    #[inline] pub fn set_mrxfifo<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Param {
    #[inline]
    fn from(other: u32) -> Self {
         Param(other)
    }
}

impl ::core::fmt::Display for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Param {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mtxfifo() != 0 { try!(write!(f, " mtxfifo=0x{:x}", self.mtxfifo()))}
        if self.mrxfifo() != 0 { try!(write!(f, " mrxfifo=0x{:x}", self.mrxfifo()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc="Master Enable"]
    #[inline] pub fn men(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if MEN != 0"]
    #[inline] pub fn test_men(&self) -> bool {
        self.men() != 0
    }

    #[doc="Sets the MEN field."]
    #[inline] pub fn set_men<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Software Reset"]
    #[inline] pub fn rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RST != 0"]
    #[inline] pub fn test_rst(&self) -> bool {
        self.rst() != 0
    }

    #[doc="Sets the RST field."]
    #[inline] pub fn set_rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Doze mode enable"]
    #[inline] pub fn dozen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DOZEN != 0"]
    #[inline] pub fn test_dozen(&self) -> bool {
        self.dozen() != 0
    }

    #[doc="Sets the DOZEN field."]
    #[inline] pub fn set_dozen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Debug Enable"]
    #[inline] pub fn dbgen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DBGEN != 0"]
    #[inline] pub fn test_dbgen(&self) -> bool {
        self.dbgen() != 0
    }

    #[doc="Sets the DBGEN field."]
    #[inline] pub fn set_dbgen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Reset Transmit FIFO"]
    #[inline] pub fn rtf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RTF != 0"]
    #[inline] pub fn test_rtf(&self) -> bool {
        self.rtf() != 0
    }

    #[doc="Sets the RTF field."]
    #[inline] pub fn set_rtf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Reset Receive FIFO"]
    #[inline] pub fn rrf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RRF != 0"]
    #[inline] pub fn test_rrf(&self) -> bool {
        self.rrf() != 0
    }

    #[doc="Sets the RRF field."]
    #[inline] pub fn set_rrf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Mcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mcr(other)
    }
}

impl ::core::fmt::Display for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.men() != 0 { try!(write!(f, " men"))}
        if self.rst() != 0 { try!(write!(f, " rst"))}
        if self.dozen() != 0 { try!(write!(f, " dozen"))}
        if self.dbgen() != 0 { try!(write!(f, " dbgen"))}
        if self.rtf() != 0 { try!(write!(f, " rtf"))}
        if self.rrf() != 0 { try!(write!(f, " rrf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc="Transmit Data Flag"]
    #[inline] pub fn tdf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TDF != 0"]
    #[inline] pub fn test_tdf(&self) -> bool {
        self.tdf() != 0
    }

    #[doc="Sets the TDF field."]
    #[inline] pub fn set_tdf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Flag"]
    #[inline] pub fn rdf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDF != 0"]
    #[inline] pub fn test_rdf(&self) -> bool {
        self.rdf() != 0
    }

    #[doc="Sets the RDF field."]
    #[inline] pub fn set_rdf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End Packet Flag"]
    #[inline] pub fn epf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EPF != 0"]
    #[inline] pub fn test_epf(&self) -> bool {
        self.epf() != 0
    }

    #[doc="Sets the EPF field."]
    #[inline] pub fn set_epf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="STOP Detect Flag"]
    #[inline] pub fn sdf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SDF != 0"]
    #[inline] pub fn test_sdf(&self) -> bool {
        self.sdf() != 0
    }

    #[doc="Sets the SDF field."]
    #[inline] pub fn set_sdf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="NACK Detect Flag"]
    #[inline] pub fn ndf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if NDF != 0"]
    #[inline] pub fn test_ndf(&self) -> bool {
        self.ndf() != 0
    }

    #[doc="Sets the NDF field."]
    #[inline] pub fn set_ndf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Arbitration Lost Flag"]
    #[inline] pub fn alf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ALF != 0"]
    #[inline] pub fn test_alf(&self) -> bool {
        self.alf() != 0
    }

    #[doc="Sets the ALF field."]
    #[inline] pub fn set_alf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="FIFO Error Flag"]
    #[inline] pub fn fef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FEF != 0"]
    #[inline] pub fn test_fef(&self) -> bool {
        self.fef() != 0
    }

    #[doc="Sets the FEF field."]
    #[inline] pub fn set_fef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pin Low Timeout Flag"]
    #[inline] pub fn pltf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PLTF != 0"]
    #[inline] pub fn test_pltf(&self) -> bool {
        self.pltf() != 0
    }

    #[doc="Sets the PLTF field."]
    #[inline] pub fn set_pltf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Data Match Flag"]
    #[inline] pub fn dmf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DMF != 0"]
    #[inline] pub fn test_dmf(&self) -> bool {
        self.dmf() != 0
    }

    #[doc="Sets the DMF field."]
    #[inline] pub fn set_dmf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Master Busy Flag"]
    #[inline] pub fn mbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if MBF != 0"]
    #[inline] pub fn test_mbf(&self) -> bool {
        self.mbf() != 0
    }

    #[doc="Sets the MBF field."]
    #[inline] pub fn set_mbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Bus Busy Flag"]
    #[inline] pub fn bbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if BBF != 0"]
    #[inline] pub fn test_bbf(&self) -> bool {
        self.bbf() != 0
    }

    #[doc="Sets the BBF field."]
    #[inline] pub fn set_bbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

}

impl From<u32> for Msr {
    #[inline]
    fn from(other: u32) -> Self {
         Msr(other)
    }
}

impl ::core::fmt::Display for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdf() != 0 { try!(write!(f, " tdf"))}
        if self.rdf() != 0 { try!(write!(f, " rdf"))}
        if self.epf() != 0 { try!(write!(f, " epf"))}
        if self.sdf() != 0 { try!(write!(f, " sdf"))}
        if self.ndf() != 0 { try!(write!(f, " ndf"))}
        if self.alf() != 0 { try!(write!(f, " alf"))}
        if self.fef() != 0 { try!(write!(f, " fef"))}
        if self.pltf() != 0 { try!(write!(f, " pltf"))}
        if self.dmf() != 0 { try!(write!(f, " dmf"))}
        if self.mbf() != 0 { try!(write!(f, " mbf"))}
        if self.bbf() != 0 { try!(write!(f, " bbf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mier(pub u32);
impl Mier {
    #[doc="Transmit Data Interrupt Enable"]
    #[inline] pub fn tdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TDIE != 0"]
    #[inline] pub fn test_tdie(&self) -> bool {
        self.tdie() != 0
    }

    #[doc="Sets the TDIE field."]
    #[inline] pub fn set_tdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Interrupt Enable"]
    #[inline] pub fn rdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDIE != 0"]
    #[inline] pub fn test_rdie(&self) -> bool {
        self.rdie() != 0
    }

    #[doc="Sets the RDIE field."]
    #[inline] pub fn set_rdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="End Packet Interrupt Enable"]
    #[inline] pub fn epie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EPIE != 0"]
    #[inline] pub fn test_epie(&self) -> bool {
        self.epie() != 0
    }

    #[doc="Sets the EPIE field."]
    #[inline] pub fn set_epie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="STOP Detect Interrupt Enable"]
    #[inline] pub fn sdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SDIE != 0"]
    #[inline] pub fn test_sdie(&self) -> bool {
        self.sdie() != 0
    }

    #[doc="Sets the SDIE field."]
    #[inline] pub fn set_sdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="NACK Detect Interrupt Enable"]
    #[inline] pub fn ndie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if NDIE != 0"]
    #[inline] pub fn test_ndie(&self) -> bool {
        self.ndie() != 0
    }

    #[doc="Sets the NDIE field."]
    #[inline] pub fn set_ndie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Arbitration Lost Interrupt Enable"]
    #[inline] pub fn alie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ALIE != 0"]
    #[inline] pub fn test_alie(&self) -> bool {
        self.alie() != 0
    }

    #[doc="Sets the ALIE field."]
    #[inline] pub fn set_alie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="FIFO Error Interrupt Enable"]
    #[inline] pub fn feie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FEIE != 0"]
    #[inline] pub fn test_feie(&self) -> bool {
        self.feie() != 0
    }

    #[doc="Sets the FEIE field."]
    #[inline] pub fn set_feie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pin Low Timeout Interrupt Enable"]
    #[inline] pub fn pltie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PLTIE != 0"]
    #[inline] pub fn test_pltie(&self) -> bool {
        self.pltie() != 0
    }

    #[doc="Sets the PLTIE field."]
    #[inline] pub fn set_pltie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Data Match Interrupt Enable"]
    #[inline] pub fn dmie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if DMIE != 0"]
    #[inline] pub fn test_dmie(&self) -> bool {
        self.dmie() != 0
    }

    #[doc="Sets the DMIE field."]
    #[inline] pub fn set_dmie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Mier {
    #[inline]
    fn from(other: u32) -> Self {
         Mier(other)
    }
}

impl ::core::fmt::Display for Mier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdie() != 0 { try!(write!(f, " tdie"))}
        if self.rdie() != 0 { try!(write!(f, " rdie"))}
        if self.epie() != 0 { try!(write!(f, " epie"))}
        if self.sdie() != 0 { try!(write!(f, " sdie"))}
        if self.ndie() != 0 { try!(write!(f, " ndie"))}
        if self.alie() != 0 { try!(write!(f, " alie"))}
        if self.feie() != 0 { try!(write!(f, " feie"))}
        if self.pltie() != 0 { try!(write!(f, " pltie"))}
        if self.dmie() != 0 { try!(write!(f, " dmie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master DMA Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mder(pub u32);
impl Mder {
    #[doc="Transmit Data DMA Enable"]
    #[inline] pub fn tdde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TDDE != 0"]
    #[inline] pub fn test_tdde(&self) -> bool {
        self.tdde() != 0
    }

    #[doc="Sets the TDDE field."]
    #[inline] pub fn set_tdde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data DMA Enable"]
    #[inline] pub fn rdde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDDE != 0"]
    #[inline] pub fn test_rdde(&self) -> bool {
        self.rdde() != 0
    }

    #[doc="Sets the RDDE field."]
    #[inline] pub fn set_rdde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Mder {
    #[inline]
    fn from(other: u32) -> Self {
         Mder(other)
    }
}

impl ::core::fmt::Display for Mder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdde() != 0 { try!(write!(f, " tdde"))}
        if self.rdde() != 0 { try!(write!(f, " rdde"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Configuration Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcfgr0(pub u32);
impl Mcfgr0 {
    #[doc="Host Request Enable"]
    #[inline] pub fn hren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HREN != 0"]
    #[inline] pub fn test_hren(&self) -> bool {
        self.hren() != 0
    }

    #[doc="Sets the HREN field."]
    #[inline] pub fn set_hren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Host Request Polarity"]
    #[inline] pub fn hrpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HRPOL != 0"]
    #[inline] pub fn test_hrpol(&self) -> bool {
        self.hrpol() != 0
    }

    #[doc="Sets the HRPOL field."]
    #[inline] pub fn set_hrpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Host Request Select"]
    #[inline] pub fn hrsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if HRSEL != 0"]
    #[inline] pub fn test_hrsel(&self) -> bool {
        self.hrsel() != 0
    }

    #[doc="Sets the HRSEL field."]
    #[inline] pub fn set_hrsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Circular FIFO Enable"]
    #[inline] pub fn cirfifo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CIRFIFO != 0"]
    #[inline] pub fn test_cirfifo(&self) -> bool {
        self.cirfifo() != 0
    }

    #[doc="Sets the CIRFIFO field."]
    #[inline] pub fn set_cirfifo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Receive Data Match Only"]
    #[inline] pub fn rdmo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RDMO != 0"]
    #[inline] pub fn test_rdmo(&self) -> bool {
        self.rdmo() != 0
    }

    #[doc="Sets the RDMO field."]
    #[inline] pub fn set_rdmo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Mcfgr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Mcfgr0(other)
    }
}

impl ::core::fmt::Display for Mcfgr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcfgr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hren() != 0 { try!(write!(f, " hren"))}
        if self.hrpol() != 0 { try!(write!(f, " hrpol"))}
        if self.hrsel() != 0 { try!(write!(f, " hrsel"))}
        if self.cirfifo() != 0 { try!(write!(f, " cirfifo"))}
        if self.rdmo() != 0 { try!(write!(f, " rdmo"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Configuration Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcfgr1(pub u32);
impl Mcfgr1 {
    #[doc="Prescaler"]
    #[inline] pub fn prescale(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PRESCALE != 0"]
    #[inline] pub fn test_prescale(&self) -> bool {
        self.prescale() != 0
    }

    #[doc="Sets the PRESCALE field."]
    #[inline] pub fn set_prescale<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Automatic STOP Generation"]
    #[inline] pub fn autostop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if AUTOSTOP != 0"]
    #[inline] pub fn test_autostop(&self) -> bool {
        self.autostop() != 0
    }

    #[doc="Sets the AUTOSTOP field."]
    #[inline] pub fn set_autostop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="IGNACK"]
    #[inline] pub fn ignack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if IGNACK != 0"]
    #[inline] pub fn test_ignack(&self) -> bool {
        self.ignack() != 0
    }

    #[doc="Sets the IGNACK field."]
    #[inline] pub fn set_ignack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Timeout Configuration"]
    #[inline] pub fn timecfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TIMECFG != 0"]
    #[inline] pub fn test_timecfg(&self) -> bool {
        self.timecfg() != 0
    }

    #[doc="Sets the TIMECFG field."]
    #[inline] pub fn set_timecfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Match Configuration"]
    #[inline] pub fn matcfg(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if MATCFG != 0"]
    #[inline] pub fn test_matcfg(&self) -> bool {
        self.matcfg() != 0
    }

    #[doc="Sets the MATCFG field."]
    #[inline] pub fn set_matcfg<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pin Configuration"]
    #[inline] pub fn pincfg(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x7) as u8) } // [26:24]
    }

    #[doc="Returns true if PINCFG != 0"]
    #[inline] pub fn test_pincfg(&self) -> bool {
        self.pincfg() != 0
    }

    #[doc="Sets the PINCFG field."]
    #[inline] pub fn set_pincfg<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Mcfgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Mcfgr1(other)
    }
}

impl ::core::fmt::Display for Mcfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prescale() != 0 { try!(write!(f, " prescale=0x{:x}", self.prescale()))}
        if self.autostop() != 0 { try!(write!(f, " autostop"))}
        if self.ignack() != 0 { try!(write!(f, " ignack"))}
        if self.timecfg() != 0 { try!(write!(f, " timecfg"))}
        if self.matcfg() != 0 { try!(write!(f, " matcfg=0x{:x}", self.matcfg()))}
        if self.pincfg() != 0 { try!(write!(f, " pincfg=0x{:x}", self.pincfg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Configuration Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcfgr2(pub u32);
impl Mcfgr2 {
    #[doc="Bus Idle Timeout"]
    #[inline] pub fn busidle(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if BUSIDLE != 0"]
    #[inline] pub fn test_busidle(&self) -> bool {
        self.busidle() != 0
    }

    #[doc="Sets the BUSIDLE field."]
    #[inline] pub fn set_busidle<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Glitch Filter SCL"]
    #[inline] pub fn filtscl(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if FILTSCL != 0"]
    #[inline] pub fn test_filtscl(&self) -> bool {
        self.filtscl() != 0
    }

    #[doc="Sets the FILTSCL field."]
    #[inline] pub fn set_filtscl<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Glitch Filter SDA"]
    #[inline] pub fn filtsda(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FILTSDA != 0"]
    #[inline] pub fn test_filtsda(&self) -> bool {
        self.filtsda() != 0
    }

    #[doc="Sets the FILTSDA field."]
    #[inline] pub fn set_filtsda<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Mcfgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Mcfgr2(other)
    }
}

impl ::core::fmt::Display for Mcfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.busidle() != 0 { try!(write!(f, " busidle=0x{:x}", self.busidle()))}
        if self.filtscl() != 0 { try!(write!(f, " filtscl=0x{:x}", self.filtscl()))}
        if self.filtsda() != 0 { try!(write!(f, " filtsda=0x{:x}", self.filtsda()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Configuration Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcfgr3(pub u32);
impl Mcfgr3 {
    #[doc="Pin Low Timeout"]
    #[inline] pub fn pinlow(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xfff) as u16) } // [19:8]
    }

    #[doc="Returns true if PINLOW != 0"]
    #[inline] pub fn test_pinlow(&self) -> bool {
        self.pinlow() != 0
    }

    #[doc="Sets the PINLOW field."]
    #[inline] pub fn set_pinlow<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Mcfgr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Mcfgr3(other)
    }
}

impl ::core::fmt::Display for Mcfgr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcfgr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pinlow() != 0 { try!(write!(f, " pinlow=0x{:x}", self.pinlow()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Data Match Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mdmr(pub u32);
impl Mdmr {
    #[doc="Match 0 Value"]
    #[inline] pub fn match0(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if MATCH0 != 0"]
    #[inline] pub fn test_match0(&self) -> bool {
        self.match0() != 0
    }

    #[doc="Sets the MATCH0 field."]
    #[inline] pub fn set_match0<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Match 1 Value"]
    #[inline] pub fn match1(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if MATCH1 != 0"]
    #[inline] pub fn test_match1(&self) -> bool {
        self.match1() != 0
    }

    #[doc="Sets the MATCH1 field."]
    #[inline] pub fn set_match1<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Mdmr {
    #[inline]
    fn from(other: u32) -> Self {
         Mdmr(other)
    }
}

impl ::core::fmt::Display for Mdmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mdmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.match0() != 0 { try!(write!(f, " match0=0x{:x}", self.match0()))}
        if self.match1() != 0 { try!(write!(f, " match1=0x{:x}", self.match1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Clock Configuration Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mccr0(pub u32);
impl Mccr0 {
    #[doc="Clock Low Period"]
    #[inline] pub fn clklo(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLKLO != 0"]
    #[inline] pub fn test_clklo(&self) -> bool {
        self.clklo() != 0
    }

    #[doc="Sets the CLKLO field."]
    #[inline] pub fn set_clklo<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock High Period"]
    #[inline] pub fn clkhi(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if CLKHI != 0"]
    #[inline] pub fn test_clkhi(&self) -> bool {
        self.clkhi() != 0
    }

    #[doc="Sets the CLKHI field."]
    #[inline] pub fn set_clkhi<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Setup Hold Delay"]
    #[inline] pub fn sethold(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if SETHOLD != 0"]
    #[inline] pub fn test_sethold(&self) -> bool {
        self.sethold() != 0
    }

    #[doc="Sets the SETHOLD field."]
    #[inline] pub fn set_sethold<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data Valid Delay"]
    #[inline] pub fn datavd(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if DATAVD != 0"]
    #[inline] pub fn test_datavd(&self) -> bool {
        self.datavd() != 0
    }

    #[doc="Sets the DATAVD field."]
    #[inline] pub fn set_datavd<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Mccr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Mccr0(other)
    }
}

impl ::core::fmt::Display for Mccr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mccr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clklo() != 0 { try!(write!(f, " clklo=0x{:x}", self.clklo()))}
        if self.clkhi() != 0 { try!(write!(f, " clkhi=0x{:x}", self.clkhi()))}
        if self.sethold() != 0 { try!(write!(f, " sethold=0x{:x}", self.sethold()))}
        if self.datavd() != 0 { try!(write!(f, " datavd=0x{:x}", self.datavd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Clock Configuration Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mccr1(pub u32);
impl Mccr1 {
    #[doc="Clock Low Period"]
    #[inline] pub fn clklo(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if CLKLO != 0"]
    #[inline] pub fn test_clklo(&self) -> bool {
        self.clklo() != 0
    }

    #[doc="Sets the CLKLO field."]
    #[inline] pub fn set_clklo<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock High Period"]
    #[inline] pub fn clkhi(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if CLKHI != 0"]
    #[inline] pub fn test_clkhi(&self) -> bool {
        self.clkhi() != 0
    }

    #[doc="Sets the CLKHI field."]
    #[inline] pub fn set_clkhi<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Setup Hold Delay"]
    #[inline] pub fn sethold(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3f) as u8) } // [21:16]
    }

    #[doc="Returns true if SETHOLD != 0"]
    #[inline] pub fn test_sethold(&self) -> bool {
        self.sethold() != 0
    }

    #[doc="Sets the SETHOLD field."]
    #[inline] pub fn set_sethold<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Data Valid Delay"]
    #[inline] pub fn datavd(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3f) as u8) } // [29:24]
    }

    #[doc="Returns true if DATAVD != 0"]
    #[inline] pub fn test_datavd(&self) -> bool {
        self.datavd() != 0
    }

    #[doc="Sets the DATAVD field."]
    #[inline] pub fn set_datavd<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Mccr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Mccr1(other)
    }
}

impl ::core::fmt::Display for Mccr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mccr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clklo() != 0 { try!(write!(f, " clklo=0x{:x}", self.clklo()))}
        if self.clkhi() != 0 { try!(write!(f, " clkhi=0x{:x}", self.clkhi()))}
        if self.sethold() != 0 { try!(write!(f, " sethold=0x{:x}", self.sethold()))}
        if self.datavd() != 0 { try!(write!(f, " datavd=0x{:x}", self.datavd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master FIFO Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mfcr(pub u32);
impl Mfcr {
    #[doc="Transmit FIFO Watermark"]
    #[inline] pub fn txwater(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if TXWATER != 0"]
    #[inline] pub fn test_txwater(&self) -> bool {
        self.txwater() != 0
    }

    #[doc="Sets the TXWATER field."]
    #[inline] pub fn set_txwater<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Watermark"]
    #[inline] pub fn rxwater(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if RXWATER != 0"]
    #[inline] pub fn test_rxwater(&self) -> bool {
        self.rxwater() != 0
    }

    #[doc="Sets the RXWATER field."]
    #[inline] pub fn set_rxwater<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Mfcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mfcr(other)
    }
}

impl ::core::fmt::Display for Mfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mfcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txwater() != 0 { try!(write!(f, " txwater=0x{:x}", self.txwater()))}
        if self.rxwater() != 0 { try!(write!(f, " rxwater=0x{:x}", self.rxwater()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master FIFO Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mfsr(pub u32);
impl Mfsr {
    #[doc="Transmit FIFO Count"]
    #[inline] pub fn txcount(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if TXCOUNT != 0"]
    #[inline] pub fn test_txcount(&self) -> bool {
        self.txcount() != 0
    }

    #[doc="Sets the TXCOUNT field."]
    #[inline] pub fn set_txcount<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive FIFO Count"]
    #[inline] pub fn rxcount(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if RXCOUNT != 0"]
    #[inline] pub fn test_rxcount(&self) -> bool {
        self.rxcount() != 0
    }

    #[doc="Sets the RXCOUNT field."]
    #[inline] pub fn set_rxcount<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Mfsr {
    #[inline]
    fn from(other: u32) -> Self {
         Mfsr(other)
    }
}

impl ::core::fmt::Display for Mfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mfsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txcount() != 0 { try!(write!(f, " txcount=0x{:x}", self.txcount()))}
        if self.rxcount() != 0 { try!(write!(f, " rxcount=0x{:x}", self.rxcount()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Transmit Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mtdr(pub u32);
impl Mtdr {
    #[doc="Transmit Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Command Data"]
    #[inline] pub fn cmd(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if CMD != 0"]
    #[inline] pub fn test_cmd(&self) -> bool {
        self.cmd() != 0
    }

    #[doc="Sets the CMD field."]
    #[inline] pub fn set_cmd<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Mtdr {
    #[inline]
    fn from(other: u32) -> Self {
         Mtdr(other)
    }
}

impl ::core::fmt::Display for Mtdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mtdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        if self.cmd() != 0 { try!(write!(f, " cmd=0x{:x}", self.cmd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Master Receive Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mrdr(pub u32);
impl Mrdr {
    #[doc="Receive Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RX Empty"]
    #[inline] pub fn rxempty(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RXEMPTY != 0"]
    #[inline] pub fn test_rxempty(&self) -> bool {
        self.rxempty() != 0
    }

    #[doc="Sets the RXEMPTY field."]
    #[inline] pub fn set_rxempty<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Mrdr {
    #[inline]
    fn from(other: u32) -> Self {
         Mrdr(other)
    }
}

impl ::core::fmt::Display for Mrdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mrdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        if self.rxempty() != 0 { try!(write!(f, " rxempty"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc="Slave Enable"]
    #[inline] pub fn sen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SEN != 0"]
    #[inline] pub fn test_sen(&self) -> bool {
        self.sen() != 0
    }

    #[doc="Sets the SEN field."]
    #[inline] pub fn set_sen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Software Reset"]
    #[inline] pub fn rst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RST != 0"]
    #[inline] pub fn test_rst(&self) -> bool {
        self.rst() != 0
    }

    #[doc="Sets the RST field."]
    #[inline] pub fn set_rst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Filter Enable"]
    #[inline] pub fn filten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FILTEN != 0"]
    #[inline] pub fn test_filten(&self) -> bool {
        self.filten() != 0
    }

    #[doc="Sets the FILTEN field."]
    #[inline] pub fn set_filten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Filter Doze Enable"]
    #[inline] pub fn filtdz(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FILTDZ != 0"]
    #[inline] pub fn test_filtdz(&self) -> bool {
        self.filtdz() != 0
    }

    #[doc="Sets the FILTDZ field."]
    #[inline] pub fn set_filtdz<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

}

impl From<u32> for Scr {
    #[inline]
    fn from(other: u32) -> Self {
         Scr(other)
    }
}

impl ::core::fmt::Display for Scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sen() != 0 { try!(write!(f, " sen"))}
        if self.rst() != 0 { try!(write!(f, " rst"))}
        if self.filten() != 0 { try!(write!(f, " filten"))}
        if self.filtdz() != 0 { try!(write!(f, " filtdz"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ssr(pub u32);
impl Ssr {
    #[doc="Transmit Data Flag"]
    #[inline] pub fn tdf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TDF != 0"]
    #[inline] pub fn test_tdf(&self) -> bool {
        self.tdf() != 0
    }

    #[doc="Sets the TDF field."]
    #[inline] pub fn set_tdf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Flag"]
    #[inline] pub fn rdf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDF != 0"]
    #[inline] pub fn test_rdf(&self) -> bool {
        self.rdf() != 0
    }

    #[doc="Sets the RDF field."]
    #[inline] pub fn set_rdf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Address Valid Flag"]
    #[inline] pub fn avf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AVF != 0"]
    #[inline] pub fn test_avf(&self) -> bool {
        self.avf() != 0
    }

    #[doc="Sets the AVF field."]
    #[inline] pub fn set_avf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit ACK Flag"]
    #[inline] pub fn taf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TAF != 0"]
    #[inline] pub fn test_taf(&self) -> bool {
        self.taf() != 0
    }

    #[doc="Sets the TAF field."]
    #[inline] pub fn set_taf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Repeated Start Flag"]
    #[inline] pub fn rsf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RSF != 0"]
    #[inline] pub fn test_rsf(&self) -> bool {
        self.rsf() != 0
    }

    #[doc="Sets the RSF field."]
    #[inline] pub fn set_rsf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="STOP Detect Flag"]
    #[inline] pub fn sdf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SDF != 0"]
    #[inline] pub fn test_sdf(&self) -> bool {
        self.sdf() != 0
    }

    #[doc="Sets the SDF field."]
    #[inline] pub fn set_sdf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Bit Error Flag"]
    #[inline] pub fn bef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BEF != 0"]
    #[inline] pub fn test_bef(&self) -> bool {
        self.bef() != 0
    }

    #[doc="Sets the BEF field."]
    #[inline] pub fn set_bef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="FIFO Error Flag"]
    #[inline] pub fn fef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FEF != 0"]
    #[inline] pub fn test_fef(&self) -> bool {
        self.fef() != 0
    }

    #[doc="Sets the FEF field."]
    #[inline] pub fn set_fef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Address Match 0 Flag"]
    #[inline] pub fn am0f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if AM0F != 0"]
    #[inline] pub fn test_am0f(&self) -> bool {
        self.am0f() != 0
    }

    #[doc="Sets the AM0F field."]
    #[inline] pub fn set_am0f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Address Match 1 Flag"]
    #[inline] pub fn am1f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if AM1F != 0"]
    #[inline] pub fn test_am1f(&self) -> bool {
        self.am1f() != 0
    }

    #[doc="Sets the AM1F field."]
    #[inline] pub fn set_am1f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="General Call Flag"]
    #[inline] pub fn gcf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GCF != 0"]
    #[inline] pub fn test_gcf(&self) -> bool {
        self.gcf() != 0
    }

    #[doc="Sets the GCF field."]
    #[inline] pub fn set_gcf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SMBus Alert Response Flag"]
    #[inline] pub fn sarf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SARF != 0"]
    #[inline] pub fn test_sarf(&self) -> bool {
        self.sarf() != 0
    }

    #[doc="Sets the SARF field."]
    #[inline] pub fn set_sarf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Slave Busy Flag"]
    #[inline] pub fn sbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if SBF != 0"]
    #[inline] pub fn test_sbf(&self) -> bool {
        self.sbf() != 0
    }

    #[doc="Sets the SBF field."]
    #[inline] pub fn set_sbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Bus Busy Flag"]
    #[inline] pub fn bbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if BBF != 0"]
    #[inline] pub fn test_bbf(&self) -> bool {
        self.bbf() != 0
    }

    #[doc="Sets the BBF field."]
    #[inline] pub fn set_bbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

}

impl From<u32> for Ssr {
    #[inline]
    fn from(other: u32) -> Self {
         Ssr(other)
    }
}

impl ::core::fmt::Display for Ssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ssr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdf() != 0 { try!(write!(f, " tdf"))}
        if self.rdf() != 0 { try!(write!(f, " rdf"))}
        if self.avf() != 0 { try!(write!(f, " avf"))}
        if self.taf() != 0 { try!(write!(f, " taf"))}
        if self.rsf() != 0 { try!(write!(f, " rsf"))}
        if self.sdf() != 0 { try!(write!(f, " sdf"))}
        if self.bef() != 0 { try!(write!(f, " bef"))}
        if self.fef() != 0 { try!(write!(f, " fef"))}
        if self.am0f() != 0 { try!(write!(f, " am0f"))}
        if self.am1f() != 0 { try!(write!(f, " am1f"))}
        if self.gcf() != 0 { try!(write!(f, " gcf"))}
        if self.sarf() != 0 { try!(write!(f, " sarf"))}
        if self.sbf() != 0 { try!(write!(f, " sbf"))}
        if self.bbf() != 0 { try!(write!(f, " bbf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Interrupt Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sier(pub u32);
impl Sier {
    #[doc="Transmit Data Interrupt Enable"]
    #[inline] pub fn tdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TDIE != 0"]
    #[inline] pub fn test_tdie(&self) -> bool {
        self.tdie() != 0
    }

    #[doc="Sets the TDIE field."]
    #[inline] pub fn set_tdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data Interrupt Enable"]
    #[inline] pub fn rdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDIE != 0"]
    #[inline] pub fn test_rdie(&self) -> bool {
        self.rdie() != 0
    }

    #[doc="Sets the RDIE field."]
    #[inline] pub fn set_rdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Address Valid Interrupt Enable"]
    #[inline] pub fn avie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AVIE != 0"]
    #[inline] pub fn test_avie(&self) -> bool {
        self.avie() != 0
    }

    #[doc="Sets the AVIE field."]
    #[inline] pub fn set_avie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmit ACK Interrupt Enable"]
    #[inline] pub fn taie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TAIE != 0"]
    #[inline] pub fn test_taie(&self) -> bool {
        self.taie() != 0
    }

    #[doc="Sets the TAIE field."]
    #[inline] pub fn set_taie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Repeated Start Interrupt Enable"]
    #[inline] pub fn rsie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RSIE != 0"]
    #[inline] pub fn test_rsie(&self) -> bool {
        self.rsie() != 0
    }

    #[doc="Sets the RSIE field."]
    #[inline] pub fn set_rsie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="STOP Detect Interrupt Enable"]
    #[inline] pub fn sdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SDIE != 0"]
    #[inline] pub fn test_sdie(&self) -> bool {
        self.sdie() != 0
    }

    #[doc="Sets the SDIE field."]
    #[inline] pub fn set_sdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Bit Error Interrupt Enable"]
    #[inline] pub fn beie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BEIE != 0"]
    #[inline] pub fn test_beie(&self) -> bool {
        self.beie() != 0
    }

    #[doc="Sets the BEIE field."]
    #[inline] pub fn set_beie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="FIFO Error Interrupt Enable"]
    #[inline] pub fn feie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FEIE != 0"]
    #[inline] pub fn test_feie(&self) -> bool {
        self.feie() != 0
    }

    #[doc="Sets the FEIE field."]
    #[inline] pub fn set_feie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Address Match 0 Interrupt Enable"]
    #[inline] pub fn am0ie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if AM0IE != 0"]
    #[inline] pub fn test_am0ie(&self) -> bool {
        self.am0ie() != 0
    }

    #[doc="Sets the AM0IE field."]
    #[inline] pub fn set_am0ie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Address Match 1 Interrupt Enable"]
    #[inline] pub fn am1f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if AM1F != 0"]
    #[inline] pub fn test_am1f(&self) -> bool {
        self.am1f() != 0
    }

    #[doc="Sets the AM1F field."]
    #[inline] pub fn set_am1f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="General Call Interrupt Enable"]
    #[inline] pub fn gcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if GCIE != 0"]
    #[inline] pub fn test_gcie(&self) -> bool {
        self.gcie() != 0
    }

    #[doc="Sets the GCIE field."]
    #[inline] pub fn set_gcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="SMBus Alert Response Interrupt Enable"]
    #[inline] pub fn sarie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SARIE != 0"]
    #[inline] pub fn test_sarie(&self) -> bool {
        self.sarie() != 0
    }

    #[doc="Sets the SARIE field."]
    #[inline] pub fn set_sarie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Sier {
    #[inline]
    fn from(other: u32) -> Self {
         Sier(other)
    }
}

impl ::core::fmt::Display for Sier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdie() != 0 { try!(write!(f, " tdie"))}
        if self.rdie() != 0 { try!(write!(f, " rdie"))}
        if self.avie() != 0 { try!(write!(f, " avie"))}
        if self.taie() != 0 { try!(write!(f, " taie"))}
        if self.rsie() != 0 { try!(write!(f, " rsie"))}
        if self.sdie() != 0 { try!(write!(f, " sdie"))}
        if self.beie() != 0 { try!(write!(f, " beie"))}
        if self.feie() != 0 { try!(write!(f, " feie"))}
        if self.am0ie() != 0 { try!(write!(f, " am0ie"))}
        if self.am1f() != 0 { try!(write!(f, " am1f"))}
        if self.gcie() != 0 { try!(write!(f, " gcie"))}
        if self.sarie() != 0 { try!(write!(f, " sarie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave DMA Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sder(pub u32);
impl Sder {
    #[doc="Transmit Data DMA Enable"]
    #[inline] pub fn tdde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TDDE != 0"]
    #[inline] pub fn test_tdde(&self) -> bool {
        self.tdde() != 0
    }

    #[doc="Sets the TDDE field."]
    #[inline] pub fn set_tdde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receive Data DMA Enable"]
    #[inline] pub fn rdde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RDDE != 0"]
    #[inline] pub fn test_rdde(&self) -> bool {
        self.rdde() != 0
    }

    #[doc="Sets the RDDE field."]
    #[inline] pub fn set_rdde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Address Valid DMA Enable"]
    #[inline] pub fn avde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if AVDE != 0"]
    #[inline] pub fn test_avde(&self) -> bool {
        self.avde() != 0
    }

    #[doc="Sets the AVDE field."]
    #[inline] pub fn set_avde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

}

impl From<u32> for Sder {
    #[inline]
    fn from(other: u32) -> Self {
         Sder(other)
    }
}

impl ::core::fmt::Display for Sder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tdde() != 0 { try!(write!(f, " tdde"))}
        if self.rdde() != 0 { try!(write!(f, " rdde"))}
        if self.avde() != 0 { try!(write!(f, " avde"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Configuration Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scfgr1(pub u32);
impl Scfgr1 {
    #[doc="Address SCL Stall"]
    #[inline] pub fn adrstall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if ADRSTALL != 0"]
    #[inline] pub fn test_adrstall(&self) -> bool {
        self.adrstall() != 0
    }

    #[doc="Sets the ADRSTALL field."]
    #[inline] pub fn set_adrstall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RX SCL Stall"]
    #[inline] pub fn rxstall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RXSTALL != 0"]
    #[inline] pub fn test_rxstall(&self) -> bool {
        self.rxstall() != 0
    }

    #[doc="Sets the RXSTALL field."]
    #[inline] pub fn set_rxstall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TX Data SCL Stall"]
    #[inline] pub fn txdstall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXDSTALL != 0"]
    #[inline] pub fn test_txdstall(&self) -> bool {
        self.txdstall() != 0
    }

    #[doc="Sets the TXDSTALL field."]
    #[inline] pub fn set_txdstall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ACK SCL Stall"]
    #[inline] pub fn ackstall(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ACKSTALL != 0"]
    #[inline] pub fn test_ackstall(&self) -> bool {
        self.ackstall() != 0
    }

    #[doc="Sets the ACKSTALL field."]
    #[inline] pub fn set_ackstall<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="General Call Enable"]
    #[inline] pub fn gcen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if GCEN != 0"]
    #[inline] pub fn test_gcen(&self) -> bool {
        self.gcen() != 0
    }

    #[doc="Sets the GCEN field."]
    #[inline] pub fn set_gcen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="SMBus Alert Enable"]
    #[inline] pub fn saen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SAEN != 0"]
    #[inline] pub fn test_saen(&self) -> bool {
        self.saen() != 0
    }

    #[doc="Sets the SAEN field."]
    #[inline] pub fn set_saen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Transmit Flag Configuration"]
    #[inline] pub fn txcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TXCFG != 0"]
    #[inline] pub fn test_txcfg(&self) -> bool {
        self.txcfg() != 0
    }

    #[doc="Sets the TXCFG field."]
    #[inline] pub fn set_txcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Receive Data Configuration"]
    #[inline] pub fn rxcfg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RXCFG != 0"]
    #[inline] pub fn test_rxcfg(&self) -> bool {
        self.rxcfg() != 0
    }

    #[doc="Sets the RXCFG field."]
    #[inline] pub fn set_rxcfg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Ignore NACK"]
    #[inline] pub fn ignack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if IGNACK != 0"]
    #[inline] pub fn test_ignack(&self) -> bool {
        self.ignack() != 0
    }

    #[doc="Sets the IGNACK field."]
    #[inline] pub fn set_ignack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="High Speed Mode Enable"]
    #[inline] pub fn hsmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if HSMEN != 0"]
    #[inline] pub fn test_hsmen(&self) -> bool {
        self.hsmen() != 0
    }

    #[doc="Sets the HSMEN field."]
    #[inline] pub fn set_hsmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Address Configuration"]
    #[inline] pub fn addrcfg(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if ADDRCFG != 0"]
    #[inline] pub fn test_addrcfg(&self) -> bool {
        self.addrcfg() != 0
    }

    #[doc="Sets the ADDRCFG field."]
    #[inline] pub fn set_addrcfg<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Scfgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Scfgr1(other)
    }
}

impl ::core::fmt::Display for Scfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adrstall() != 0 { try!(write!(f, " adrstall"))}
        if self.rxstall() != 0 { try!(write!(f, " rxstall"))}
        if self.txdstall() != 0 { try!(write!(f, " txdstall"))}
        if self.ackstall() != 0 { try!(write!(f, " ackstall"))}
        if self.gcen() != 0 { try!(write!(f, " gcen"))}
        if self.saen() != 0 { try!(write!(f, " saen"))}
        if self.txcfg() != 0 { try!(write!(f, " txcfg"))}
        if self.rxcfg() != 0 { try!(write!(f, " rxcfg"))}
        if self.ignack() != 0 { try!(write!(f, " ignack"))}
        if self.hsmen() != 0 { try!(write!(f, " hsmen"))}
        if self.addrcfg() != 0 { try!(write!(f, " addrcfg=0x{:x}", self.addrcfg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Configuration Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scfgr2(pub u32);
impl Scfgr2 {
    #[doc="Clock Hold Time"]
    #[inline] pub fn clkhold(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CLKHOLD != 0"]
    #[inline] pub fn test_clkhold(&self) -> bool {
        self.clkhold() != 0
    }

    #[doc="Sets the CLKHOLD field."]
    #[inline] pub fn set_clkhold<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Data Valid Delay"]
    #[inline] pub fn datavd(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if DATAVD != 0"]
    #[inline] pub fn test_datavd(&self) -> bool {
        self.datavd() != 0
    }

    #[doc="Sets the DATAVD field."]
    #[inline] pub fn set_datavd<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Glitch Filter SCL"]
    #[inline] pub fn filtscl(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if FILTSCL != 0"]
    #[inline] pub fn test_filtscl(&self) -> bool {
        self.filtscl() != 0
    }

    #[doc="Sets the FILTSCL field."]
    #[inline] pub fn set_filtscl<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Glitch Filter SDA"]
    #[inline] pub fn filtsda(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FILTSDA != 0"]
    #[inline] pub fn test_filtsda(&self) -> bool {
        self.filtsda() != 0
    }

    #[doc="Sets the FILTSDA field."]
    #[inline] pub fn set_filtsda<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Scfgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Scfgr2(other)
    }
}

impl ::core::fmt::Display for Scfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clkhold() != 0 { try!(write!(f, " clkhold=0x{:x}", self.clkhold()))}
        if self.datavd() != 0 { try!(write!(f, " datavd=0x{:x}", self.datavd()))}
        if self.filtscl() != 0 { try!(write!(f, " filtscl=0x{:x}", self.filtscl()))}
        if self.filtsda() != 0 { try!(write!(f, " filtsda=0x{:x}", self.filtsda()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Address Match Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Samr(pub u32);
impl Samr {
    #[doc="Address 0 Value"]
    #[inline] pub fn addr0(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3ff) as u16) } // [10:1]
    }

    #[doc="Returns true if ADDR0 != 0"]
    #[inline] pub fn test_addr0(&self) -> bool {
        self.addr0() != 0
    }

    #[doc="Sets the ADDR0 field."]
    #[inline] pub fn set_addr0<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Address 1 Value"]
    #[inline] pub fn addr1(&self) -> ::bobbin_bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x3ff) as u16) } // [26:17]
    }

    #[doc="Returns true if ADDR1 != 0"]
    #[inline] pub fn test_addr1(&self) -> bool {
        self.addr1() != 0
    }

    #[doc="Sets the ADDR1 field."]
    #[inline] pub fn set_addr1<V: Into<::bobbin_bits::U10>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 17);
        self.0 |= value << 17;
        self
    }

}

impl From<u32> for Samr {
    #[inline]
    fn from(other: u32) -> Self {
         Samr(other)
    }
}

impl ::core::fmt::Display for Samr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Samr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.addr0() != 0 { try!(write!(f, " addr0=0x{:x}", self.addr0()))}
        if self.addr1() != 0 { try!(write!(f, " addr1=0x{:x}", self.addr1()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Address Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sasr(pub u32);
impl Sasr {
    #[doc="Received Address"]
    #[inline] pub fn raddr(&self) -> ::bobbin_bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7ff) as u16) } // [10:0]
    }

    #[doc="Returns true if RADDR != 0"]
    #[inline] pub fn test_raddr(&self) -> bool {
        self.raddr() != 0
    }

    #[doc="Sets the RADDR field."]
    #[inline] pub fn set_raddr<V: Into<::bobbin_bits::U11>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Address Not Valid"]
    #[inline] pub fn anv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if ANV != 0"]
    #[inline] pub fn test_anv(&self) -> bool {
        self.anv() != 0
    }

    #[doc="Sets the ANV field."]
    #[inline] pub fn set_anv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Sasr {
    #[inline]
    fn from(other: u32) -> Self {
         Sasr(other)
    }
}

impl ::core::fmt::Display for Sasr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sasr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.raddr() != 0 { try!(write!(f, " raddr=0x{:x}", self.raddr()))}
        if self.anv() != 0 { try!(write!(f, " anv"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Transmit ACK Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Star(pub u32);
impl Star {
    #[doc="Transmit NACK"]
    #[inline] pub fn txnack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXNACK != 0"]
    #[inline] pub fn test_txnack(&self) -> bool {
        self.txnack() != 0
    }

    #[doc="Sets the TXNACK field."]
    #[inline] pub fn set_txnack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Star {
    #[inline]
    fn from(other: u32) -> Self {
         Star(other)
    }
}

impl ::core::fmt::Display for Star {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Star {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txnack() != 0 { try!(write!(f, " txnack"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Transmit Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Stdr(pub u32);
impl Stdr {
    #[doc="Transmit Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Stdr {
    #[inline]
    fn from(other: u32) -> Self {
         Stdr(other)
    }
}

impl ::core::fmt::Display for Stdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Stdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Slave Receive Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srdr(pub u32);
impl Srdr {
    #[doc="Receive Data"]
    #[inline] pub fn data(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="RX Empty"]
    #[inline] pub fn rxempty(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if RXEMPTY != 0"]
    #[inline] pub fn test_rxempty(&self) -> bool {
        self.rxempty() != 0
    }

    #[doc="Sets the RXEMPTY field."]
    #[inline] pub fn set_rxempty<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Start Of Frame"]
    #[inline] pub fn sof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if SOF != 0"]
    #[inline] pub fn test_sof(&self) -> bool {
        self.sof() != 0
    }

    #[doc="Sets the SOF field."]
    #[inline] pub fn set_sof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Srdr {
    #[inline]
    fn from(other: u32) -> Self {
         Srdr(other)
    }
}

impl ::core::fmt::Display for Srdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        if self.rxempty() != 0 { try!(write!(f, " rxempty"))}
        if self.sof() != 0 { try!(write!(f, " sof"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

