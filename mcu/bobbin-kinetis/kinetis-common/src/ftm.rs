
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="FTM Peripheral"]
pub struct FtmPeriph(pub usize); 

pub struct FtmCh { pub periph: FtmPeriph, pub index: usize }

impl FtmPeriph {
    #[doc="Get the SC Register."]
    #[inline] pub fn sc_reg(&self) -> ::bobbin_mcu::register::Register<Sc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sc, 0x0)
    }

    #[doc="Get the *mut pointer for the SC register."]
    #[inline] pub fn sc_mut(&self) -> *mut Sc { 
        self.sc_reg().ptr()
    }

    #[doc="Get the *const pointer for the SC register."]
    #[inline] pub fn sc_ptr(&self) -> *const Sc { 
        self.sc_reg().ptr()
    }

    #[doc="Read the SC register."]
    #[inline] pub fn sc(&self) -> Sc { 
        self.sc_reg().read()
    }

    #[doc="Write the SC register."]
    #[inline] pub fn write_sc(&self, value: Sc) -> &Self { 
        self.sc_reg().write(value);
        self
    }

    #[doc="Set the SC register."]
    #[inline] pub fn set_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        self.sc_reg().set(f);
        self
    }

    #[doc="Modify the SC register."]
    #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        self.sc_reg().with(f);
        self
    }

    #[doc="Get the CNT Register."]
    #[inline] pub fn cnt_reg(&self) -> ::bobbin_mcu::register::Register<Cnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cnt, 0x4)
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        self.cnt_reg().read()
    }

    #[doc="Write the CNT register."]
    #[inline] pub fn write_cnt(&self, value: Cnt) -> &Self { 
        self.cnt_reg().write(value);
        self
    }

    #[doc="Set the CNT register."]
    #[inline] pub fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        self.cnt_reg().set(f);
        self
    }

    #[doc="Modify the CNT register."]
    #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        self.cnt_reg().with(f);
        self
    }

    #[doc="Get the MOD Register."]
    #[inline] pub fn mod_reg(&self) -> ::bobbin_mcu::register::Register<Mod> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mod, 0x8)
    }

    #[doc="Get the *mut pointer for the MOD register."]
    #[inline] pub fn mod_mut(&self) -> *mut Mod { 
        self.mod_reg().ptr()
    }

    #[doc="Get the *const pointer for the MOD register."]
    #[inline] pub fn mod_ptr(&self) -> *const Mod { 
        self.mod_reg().ptr()
    }

    #[doc="Read the MOD register."]
    #[inline] pub fn _mod(&self) -> Mod { 
        self.mod_reg().read()
    }

    #[doc="Write the MOD register."]
    #[inline] pub fn write_mod(&self, value: Mod) -> &Self { 
        self.mod_reg().write(value);
        self
    }

    #[doc="Set the MOD register."]
    #[inline] pub fn set_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
        self.mod_reg().set(f);
        self
    }

    #[doc="Modify the MOD register."]
    #[inline] pub fn with_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
        self.mod_reg().with(f);
        self
    }

    #[doc="Get the CSC Register."]
    #[inline] pub fn csc_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Csc, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Csc, 0xc, 0x8)
    }

    #[doc="Get the *mut pointer for the CSC register."]
    #[inline] pub fn csc_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Csc { 
        self.csc_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CSC register."]
    #[inline] pub fn csc_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Csc { 
        self.csc_reg().ptr(index.into())
    }

    #[doc="Read the CSC register."]
    #[inline] pub fn csc<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Csc { 
        self.csc_reg().read(index.into())
    }

    #[doc="Write the CSC register."]
    #[inline] pub fn write_csc<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Csc) -> &Self {
        self.csc_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CSC register."]
    #[inline] pub fn set_csc<I: Into<::bobbin_bits::R8>, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
        self.csc_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CSC register."]
    #[inline] pub fn with_csc<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
        self.csc_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CV Register."]
    #[inline] pub fn cv_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Cv, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Cv, 0x10, 0x8)
    }

    #[doc="Get the *mut pointer for the CV register."]
    #[inline] pub fn cv_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Cv { 
        self.cv_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CV register."]
    #[inline] pub fn cv_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Cv { 
        self.cv_reg().ptr(index.into())
    }

    #[doc="Read the CV register."]
    #[inline] pub fn cv<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Cv { 
        self.cv_reg().read(index.into())
    }

    #[doc="Write the CV register."]
    #[inline] pub fn write_cv<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Cv) -> &Self {
        self.cv_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CV register."]
    #[inline] pub fn set_cv<I: Into<::bobbin_bits::R8>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        self.cv_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CV register."]
    #[inline] pub fn with_cv<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        self.cv_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CNTIN Register."]
    #[inline] pub fn cntin_reg(&self) -> ::bobbin_mcu::register::Register<Cntin> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cntin, 0x4c)
    }

    #[doc="Get the *mut pointer for the CNTIN register."]
    #[inline] pub fn cntin_mut(&self) -> *mut Cntin { 
        self.cntin_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNTIN register."]
    #[inline] pub fn cntin_ptr(&self) -> *const Cntin { 
        self.cntin_reg().ptr()
    }

    #[doc="Read the CNTIN register."]
    #[inline] pub fn cntin(&self) -> Cntin { 
        self.cntin_reg().read()
    }

    #[doc="Write the CNTIN register."]
    #[inline] pub fn write_cntin(&self, value: Cntin) -> &Self { 
        self.cntin_reg().write(value);
        self
    }

    #[doc="Set the CNTIN register."]
    #[inline] pub fn set_cntin<F: FnOnce(Cntin) -> Cntin>(&self, f: F) -> &Self {
        self.cntin_reg().set(f);
        self
    }

    #[doc="Modify the CNTIN register."]
    #[inline] pub fn with_cntin<F: FnOnce(Cntin) -> Cntin>(&self, f: F) -> &Self {
        self.cntin_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x50)
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        self.status_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
        self.status_reg().ptr()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        self.status_reg().read()
    }

    #[doc="Write the STATUS register."]
    #[inline] pub fn write_status(&self, value: Status) -> &Self { 
        self.status_reg().write(value);
        self
    }

    #[doc="Set the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().set(f);
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().with(f);
        self
    }

    #[doc="Get the MODE Register."]
    #[inline] pub fn mode_reg(&self) -> ::bobbin_mcu::register::Register<Mode> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mode, 0x54)
    }

    #[doc="Get the *mut pointer for the MODE register."]
    #[inline] pub fn mode_mut(&self) -> *mut Mode { 
        self.mode_reg().ptr()
    }

    #[doc="Get the *const pointer for the MODE register."]
    #[inline] pub fn mode_ptr(&self) -> *const Mode { 
        self.mode_reg().ptr()
    }

    #[doc="Read the MODE register."]
    #[inline] pub fn mode(&self) -> Mode { 
        self.mode_reg().read()
    }

    #[doc="Write the MODE register."]
    #[inline] pub fn write_mode(&self, value: Mode) -> &Self { 
        self.mode_reg().write(value);
        self
    }

    #[doc="Set the MODE register."]
    #[inline] pub fn set_mode<F: FnOnce(Mode) -> Mode>(&self, f: F) -> &Self {
        self.mode_reg().set(f);
        self
    }

    #[doc="Modify the MODE register."]
    #[inline] pub fn with_mode<F: FnOnce(Mode) -> Mode>(&self, f: F) -> &Self {
        self.mode_reg().with(f);
        self
    }

    #[doc="Get the SYNC Register."]
    #[inline] pub fn sync_reg(&self) -> ::bobbin_mcu::register::Register<Sync> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sync, 0x58)
    }

    #[doc="Get the *mut pointer for the SYNC register."]
    #[inline] pub fn sync_mut(&self) -> *mut Sync { 
        self.sync_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNC register."]
    #[inline] pub fn sync_ptr(&self) -> *const Sync { 
        self.sync_reg().ptr()
    }

    #[doc="Read the SYNC register."]
    #[inline] pub fn sync(&self) -> Sync { 
        self.sync_reg().read()
    }

    #[doc="Write the SYNC register."]
    #[inline] pub fn write_sync(&self, value: Sync) -> &Self { 
        self.sync_reg().write(value);
        self
    }

    #[doc="Set the SYNC register."]
    #[inline] pub fn set_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
        self.sync_reg().set(f);
        self
    }

    #[doc="Modify the SYNC register."]
    #[inline] pub fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
        self.sync_reg().with(f);
        self
    }

    #[doc="Get the OUTINIT Register."]
    #[inline] pub fn outinit_reg(&self) -> ::bobbin_mcu::register::Register<Outinit> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Outinit, 0x5c)
    }

    #[doc="Get the *mut pointer for the OUTINIT register."]
    #[inline] pub fn outinit_mut(&self) -> *mut Outinit { 
        self.outinit_reg().ptr()
    }

    #[doc="Get the *const pointer for the OUTINIT register."]
    #[inline] pub fn outinit_ptr(&self) -> *const Outinit { 
        self.outinit_reg().ptr()
    }

    #[doc="Read the OUTINIT register."]
    #[inline] pub fn outinit(&self) -> Outinit { 
        self.outinit_reg().read()
    }

    #[doc="Write the OUTINIT register."]
    #[inline] pub fn write_outinit(&self, value: Outinit) -> &Self { 
        self.outinit_reg().write(value);
        self
    }

    #[doc="Set the OUTINIT register."]
    #[inline] pub fn set_outinit<F: FnOnce(Outinit) -> Outinit>(&self, f: F) -> &Self {
        self.outinit_reg().set(f);
        self
    }

    #[doc="Modify the OUTINIT register."]
    #[inline] pub fn with_outinit<F: FnOnce(Outinit) -> Outinit>(&self, f: F) -> &Self {
        self.outinit_reg().with(f);
        self
    }

    #[doc="Get the OUTMASK Register."]
    #[inline] pub fn outmask_reg(&self) -> ::bobbin_mcu::register::Register<Outmask> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Outmask, 0x60)
    }

    #[doc="Get the *mut pointer for the OUTMASK register."]
    #[inline] pub fn outmask_mut(&self) -> *mut Outmask { 
        self.outmask_reg().ptr()
    }

    #[doc="Get the *const pointer for the OUTMASK register."]
    #[inline] pub fn outmask_ptr(&self) -> *const Outmask { 
        self.outmask_reg().ptr()
    }

    #[doc="Read the OUTMASK register."]
    #[inline] pub fn outmask(&self) -> Outmask { 
        self.outmask_reg().read()
    }

    #[doc="Write the OUTMASK register."]
    #[inline] pub fn write_outmask(&self, value: Outmask) -> &Self { 
        self.outmask_reg().write(value);
        self
    }

    #[doc="Set the OUTMASK register."]
    #[inline] pub fn set_outmask<F: FnOnce(Outmask) -> Outmask>(&self, f: F) -> &Self {
        self.outmask_reg().set(f);
        self
    }

    #[doc="Modify the OUTMASK register."]
    #[inline] pub fn with_outmask<F: FnOnce(Outmask) -> Outmask>(&self, f: F) -> &Self {
        self.outmask_reg().with(f);
        self
    }

    #[doc="Get the COMBINE Register."]
    #[inline] pub fn combine_reg(&self) -> ::bobbin_mcu::register::Register<Combine> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Combine, 0x64)
    }

    #[doc="Get the *mut pointer for the COMBINE register."]
    #[inline] pub fn combine_mut(&self) -> *mut Combine { 
        self.combine_reg().ptr()
    }

    #[doc="Get the *const pointer for the COMBINE register."]
    #[inline] pub fn combine_ptr(&self) -> *const Combine { 
        self.combine_reg().ptr()
    }

    #[doc="Read the COMBINE register."]
    #[inline] pub fn combine(&self) -> Combine { 
        self.combine_reg().read()
    }

    #[doc="Write the COMBINE register."]
    #[inline] pub fn write_combine(&self, value: Combine) -> &Self { 
        self.combine_reg().write(value);
        self
    }

    #[doc="Set the COMBINE register."]
    #[inline] pub fn set_combine<F: FnOnce(Combine) -> Combine>(&self, f: F) -> &Self {
        self.combine_reg().set(f);
        self
    }

    #[doc="Modify the COMBINE register."]
    #[inline] pub fn with_combine<F: FnOnce(Combine) -> Combine>(&self, f: F) -> &Self {
        self.combine_reg().with(f);
        self
    }

    #[doc="Get the DEADTIME Register."]
    #[inline] pub fn deadtime_reg(&self) -> ::bobbin_mcu::register::Register<Deadtime> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Deadtime, 0x68)
    }

    #[doc="Get the *mut pointer for the DEADTIME register."]
    #[inline] pub fn deadtime_mut(&self) -> *mut Deadtime { 
        self.deadtime_reg().ptr()
    }

    #[doc="Get the *const pointer for the DEADTIME register."]
    #[inline] pub fn deadtime_ptr(&self) -> *const Deadtime { 
        self.deadtime_reg().ptr()
    }

    #[doc="Read the DEADTIME register."]
    #[inline] pub fn deadtime(&self) -> Deadtime { 
        self.deadtime_reg().read()
    }

    #[doc="Write the DEADTIME register."]
    #[inline] pub fn write_deadtime(&self, value: Deadtime) -> &Self { 
        self.deadtime_reg().write(value);
        self
    }

    #[doc="Set the DEADTIME register."]
    #[inline] pub fn set_deadtime<F: FnOnce(Deadtime) -> Deadtime>(&self, f: F) -> &Self {
        self.deadtime_reg().set(f);
        self
    }

    #[doc="Modify the DEADTIME register."]
    #[inline] pub fn with_deadtime<F: FnOnce(Deadtime) -> Deadtime>(&self, f: F) -> &Self {
        self.deadtime_reg().with(f);
        self
    }

    #[doc="Get the EXTTRIG Register."]
    #[inline] pub fn exttrig_reg(&self) -> ::bobbin_mcu::register::Register<Exttrig> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exttrig, 0x6c)
    }

    #[doc="Get the *mut pointer for the EXTTRIG register."]
    #[inline] pub fn exttrig_mut(&self) -> *mut Exttrig { 
        self.exttrig_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTTRIG register."]
    #[inline] pub fn exttrig_ptr(&self) -> *const Exttrig { 
        self.exttrig_reg().ptr()
    }

    #[doc="Read the EXTTRIG register."]
    #[inline] pub fn exttrig(&self) -> Exttrig { 
        self.exttrig_reg().read()
    }

    #[doc="Write the EXTTRIG register."]
    #[inline] pub fn write_exttrig(&self, value: Exttrig) -> &Self { 
        self.exttrig_reg().write(value);
        self
    }

    #[doc="Set the EXTTRIG register."]
    #[inline] pub fn set_exttrig<F: FnOnce(Exttrig) -> Exttrig>(&self, f: F) -> &Self {
        self.exttrig_reg().set(f);
        self
    }

    #[doc="Modify the EXTTRIG register."]
    #[inline] pub fn with_exttrig<F: FnOnce(Exttrig) -> Exttrig>(&self, f: F) -> &Self {
        self.exttrig_reg().with(f);
        self
    }

    #[doc="Get the POL Register."]
    #[inline] pub fn pol_reg(&self) -> ::bobbin_mcu::register::Register<Pol> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pol, 0x70)
    }

    #[doc="Get the *mut pointer for the POL register."]
    #[inline] pub fn pol_mut(&self) -> *mut Pol { 
        self.pol_reg().ptr()
    }

    #[doc="Get the *const pointer for the POL register."]
    #[inline] pub fn pol_ptr(&self) -> *const Pol { 
        self.pol_reg().ptr()
    }

    #[doc="Read the POL register."]
    #[inline] pub fn pol(&self) -> Pol { 
        self.pol_reg().read()
    }

    #[doc="Write the POL register."]
    #[inline] pub fn write_pol(&self, value: Pol) -> &Self { 
        self.pol_reg().write(value);
        self
    }

    #[doc="Set the POL register."]
    #[inline] pub fn set_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Self {
        self.pol_reg().set(f);
        self
    }

    #[doc="Modify the POL register."]
    #[inline] pub fn with_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Self {
        self.pol_reg().with(f);
        self
    }

    #[doc="Get the FMS Register."]
    #[inline] pub fn fms_reg(&self) -> ::bobbin_mcu::register::Register<Fms> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fms, 0x74)
    }

    #[doc="Get the *mut pointer for the FMS register."]
    #[inline] pub fn fms_mut(&self) -> *mut Fms { 
        self.fms_reg().ptr()
    }

    #[doc="Get the *const pointer for the FMS register."]
    #[inline] pub fn fms_ptr(&self) -> *const Fms { 
        self.fms_reg().ptr()
    }

    #[doc="Read the FMS register."]
    #[inline] pub fn fms(&self) -> Fms { 
        self.fms_reg().read()
    }

    #[doc="Write the FMS register."]
    #[inline] pub fn write_fms(&self, value: Fms) -> &Self { 
        self.fms_reg().write(value);
        self
    }

    #[doc="Set the FMS register."]
    #[inline] pub fn set_fms<F: FnOnce(Fms) -> Fms>(&self, f: F) -> &Self {
        self.fms_reg().set(f);
        self
    }

    #[doc="Modify the FMS register."]
    #[inline] pub fn with_fms<F: FnOnce(Fms) -> Fms>(&self, f: F) -> &Self {
        self.fms_reg().with(f);
        self
    }

    #[doc="Get the FILTER Register."]
    #[inline] pub fn filter_reg(&self) -> ::bobbin_mcu::register::Register<Filter> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Filter, 0x78)
    }

    #[doc="Get the *mut pointer for the FILTER register."]
    #[inline] pub fn filter_mut(&self) -> *mut Filter { 
        self.filter_reg().ptr()
    }

    #[doc="Get the *const pointer for the FILTER register."]
    #[inline] pub fn filter_ptr(&self) -> *const Filter { 
        self.filter_reg().ptr()
    }

    #[doc="Read the FILTER register."]
    #[inline] pub fn filter(&self) -> Filter { 
        self.filter_reg().read()
    }

    #[doc="Write the FILTER register."]
    #[inline] pub fn write_filter(&self, value: Filter) -> &Self { 
        self.filter_reg().write(value);
        self
    }

    #[doc="Set the FILTER register."]
    #[inline] pub fn set_filter<F: FnOnce(Filter) -> Filter>(&self, f: F) -> &Self {
        self.filter_reg().set(f);
        self
    }

    #[doc="Modify the FILTER register."]
    #[inline] pub fn with_filter<F: FnOnce(Filter) -> Filter>(&self, f: F) -> &Self {
        self.filter_reg().with(f);
        self
    }

    #[doc="Get the FLTCTRL Register."]
    #[inline] pub fn fltctrl_reg(&self) -> ::bobbin_mcu::register::Register<Fltctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fltctrl, 0x7c)
    }

    #[doc="Get the *mut pointer for the FLTCTRL register."]
    #[inline] pub fn fltctrl_mut(&self) -> *mut Fltctrl { 
        self.fltctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the FLTCTRL register."]
    #[inline] pub fn fltctrl_ptr(&self) -> *const Fltctrl { 
        self.fltctrl_reg().ptr()
    }

    #[doc="Read the FLTCTRL register."]
    #[inline] pub fn fltctrl(&self) -> Fltctrl { 
        self.fltctrl_reg().read()
    }

    #[doc="Write the FLTCTRL register."]
    #[inline] pub fn write_fltctrl(&self, value: Fltctrl) -> &Self { 
        self.fltctrl_reg().write(value);
        self
    }

    #[doc="Set the FLTCTRL register."]
    #[inline] pub fn set_fltctrl<F: FnOnce(Fltctrl) -> Fltctrl>(&self, f: F) -> &Self {
        self.fltctrl_reg().set(f);
        self
    }

    #[doc="Modify the FLTCTRL register."]
    #[inline] pub fn with_fltctrl<F: FnOnce(Fltctrl) -> Fltctrl>(&self, f: F) -> &Self {
        self.fltctrl_reg().with(f);
        self
    }

    #[doc="Get the QDCTRL Register."]
    #[inline] pub fn qdctrl_reg(&self) -> ::bobbin_mcu::register::Register<Qdctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Qdctrl, 0x80)
    }

    #[doc="Get the *mut pointer for the QDCTRL register."]
    #[inline] pub fn qdctrl_mut(&self) -> *mut Qdctrl { 
        self.qdctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the QDCTRL register."]
    #[inline] pub fn qdctrl_ptr(&self) -> *const Qdctrl { 
        self.qdctrl_reg().ptr()
    }

    #[doc="Read the QDCTRL register."]
    #[inline] pub fn qdctrl(&self) -> Qdctrl { 
        self.qdctrl_reg().read()
    }

    #[doc="Write the QDCTRL register."]
    #[inline] pub fn write_qdctrl(&self, value: Qdctrl) -> &Self { 
        self.qdctrl_reg().write(value);
        self
    }

    #[doc="Set the QDCTRL register."]
    #[inline] pub fn set_qdctrl<F: FnOnce(Qdctrl) -> Qdctrl>(&self, f: F) -> &Self {
        self.qdctrl_reg().set(f);
        self
    }

    #[doc="Modify the QDCTRL register."]
    #[inline] pub fn with_qdctrl<F: FnOnce(Qdctrl) -> Qdctrl>(&self, f: F) -> &Self {
        self.qdctrl_reg().with(f);
        self
    }

    #[doc="Get the CONF Register."]
    #[inline] pub fn conf_reg(&self) -> ::bobbin_mcu::register::Register<Conf> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Conf, 0x84)
    }

    #[doc="Get the *mut pointer for the CONF register."]
    #[inline] pub fn conf_mut(&self) -> *mut Conf { 
        self.conf_reg().ptr()
    }

    #[doc="Get the *const pointer for the CONF register."]
    #[inline] pub fn conf_ptr(&self) -> *const Conf { 
        self.conf_reg().ptr()
    }

    #[doc="Read the CONF register."]
    #[inline] pub fn conf(&self) -> Conf { 
        self.conf_reg().read()
    }

    #[doc="Write the CONF register."]
    #[inline] pub fn write_conf(&self, value: Conf) -> &Self { 
        self.conf_reg().write(value);
        self
    }

    #[doc="Set the CONF register."]
    #[inline] pub fn set_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
        self.conf_reg().set(f);
        self
    }

    #[doc="Modify the CONF register."]
    #[inline] pub fn with_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
        self.conf_reg().with(f);
        self
    }

    #[doc="Get the FLTPOL Register."]
    #[inline] pub fn fltpol_reg(&self) -> ::bobbin_mcu::register::Register<Fltpol> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fltpol, 0x88)
    }

    #[doc="Get the *mut pointer for the FLTPOL register."]
    #[inline] pub fn fltpol_mut(&self) -> *mut Fltpol { 
        self.fltpol_reg().ptr()
    }

    #[doc="Get the *const pointer for the FLTPOL register."]
    #[inline] pub fn fltpol_ptr(&self) -> *const Fltpol { 
        self.fltpol_reg().ptr()
    }

    #[doc="Read the FLTPOL register."]
    #[inline] pub fn fltpol(&self) -> Fltpol { 
        self.fltpol_reg().read()
    }

    #[doc="Write the FLTPOL register."]
    #[inline] pub fn write_fltpol(&self, value: Fltpol) -> &Self { 
        self.fltpol_reg().write(value);
        self
    }

    #[doc="Set the FLTPOL register."]
    #[inline] pub fn set_fltpol<F: FnOnce(Fltpol) -> Fltpol>(&self, f: F) -> &Self {
        self.fltpol_reg().set(f);
        self
    }

    #[doc="Modify the FLTPOL register."]
    #[inline] pub fn with_fltpol<F: FnOnce(Fltpol) -> Fltpol>(&self, f: F) -> &Self {
        self.fltpol_reg().with(f);
        self
    }

    #[doc="Get the SYNCONF Register."]
    #[inline] pub fn synconf_reg(&self) -> ::bobbin_mcu::register::Register<Synconf> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Synconf, 0x8c)
    }

    #[doc="Get the *mut pointer for the SYNCONF register."]
    #[inline] pub fn synconf_mut(&self) -> *mut Synconf { 
        self.synconf_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNCONF register."]
    #[inline] pub fn synconf_ptr(&self) -> *const Synconf { 
        self.synconf_reg().ptr()
    }

    #[doc="Read the SYNCONF register."]
    #[inline] pub fn synconf(&self) -> Synconf { 
        self.synconf_reg().read()
    }

    #[doc="Write the SYNCONF register."]
    #[inline] pub fn write_synconf(&self, value: Synconf) -> &Self { 
        self.synconf_reg().write(value);
        self
    }

    #[doc="Set the SYNCONF register."]
    #[inline] pub fn set_synconf<F: FnOnce(Synconf) -> Synconf>(&self, f: F) -> &Self {
        self.synconf_reg().set(f);
        self
    }

    #[doc="Modify the SYNCONF register."]
    #[inline] pub fn with_synconf<F: FnOnce(Synconf) -> Synconf>(&self, f: F) -> &Self {
        self.synconf_reg().with(f);
        self
    }

    #[doc="Get the INVCTRL Register."]
    #[inline] pub fn invctrl_reg(&self) -> ::bobbin_mcu::register::Register<Invctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Invctrl, 0x90)
    }

    #[doc="Get the *mut pointer for the INVCTRL register."]
    #[inline] pub fn invctrl_mut(&self) -> *mut Invctrl { 
        self.invctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the INVCTRL register."]
    #[inline] pub fn invctrl_ptr(&self) -> *const Invctrl { 
        self.invctrl_reg().ptr()
    }

    #[doc="Read the INVCTRL register."]
    #[inline] pub fn invctrl(&self) -> Invctrl { 
        self.invctrl_reg().read()
    }

    #[doc="Write the INVCTRL register."]
    #[inline] pub fn write_invctrl(&self, value: Invctrl) -> &Self { 
        self.invctrl_reg().write(value);
        self
    }

    #[doc="Set the INVCTRL register."]
    #[inline] pub fn set_invctrl<F: FnOnce(Invctrl) -> Invctrl>(&self, f: F) -> &Self {
        self.invctrl_reg().set(f);
        self
    }

    #[doc="Modify the INVCTRL register."]
    #[inline] pub fn with_invctrl<F: FnOnce(Invctrl) -> Invctrl>(&self, f: F) -> &Self {
        self.invctrl_reg().with(f);
        self
    }

    #[doc="Get the SWOCTRL Register."]
    #[inline] pub fn swoctrl_reg(&self) -> ::bobbin_mcu::register::Register<Swoctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Swoctrl, 0x94)
    }

    #[doc="Get the *mut pointer for the SWOCTRL register."]
    #[inline] pub fn swoctrl_mut(&self) -> *mut Swoctrl { 
        self.swoctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the SWOCTRL register."]
    #[inline] pub fn swoctrl_ptr(&self) -> *const Swoctrl { 
        self.swoctrl_reg().ptr()
    }

    #[doc="Read the SWOCTRL register."]
    #[inline] pub fn swoctrl(&self) -> Swoctrl { 
        self.swoctrl_reg().read()
    }

    #[doc="Write the SWOCTRL register."]
    #[inline] pub fn write_swoctrl(&self, value: Swoctrl) -> &Self { 
        self.swoctrl_reg().write(value);
        self
    }

    #[doc="Set the SWOCTRL register."]
    #[inline] pub fn set_swoctrl<F: FnOnce(Swoctrl) -> Swoctrl>(&self, f: F) -> &Self {
        self.swoctrl_reg().set(f);
        self
    }

    #[doc="Modify the SWOCTRL register."]
    #[inline] pub fn with_swoctrl<F: FnOnce(Swoctrl) -> Swoctrl>(&self, f: F) -> &Self {
        self.swoctrl_reg().with(f);
        self
    }

    #[doc="Get the PWMLOAD Register."]
    #[inline] pub fn pwmload_reg(&self) -> ::bobbin_mcu::register::Register<Pwmload> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pwmload, 0x98)
    }

    #[doc="Get the *mut pointer for the PWMLOAD register."]
    #[inline] pub fn pwmload_mut(&self) -> *mut Pwmload { 
        self.pwmload_reg().ptr()
    }

    #[doc="Get the *const pointer for the PWMLOAD register."]
    #[inline] pub fn pwmload_ptr(&self) -> *const Pwmload { 
        self.pwmload_reg().ptr()
    }

    #[doc="Read the PWMLOAD register."]
    #[inline] pub fn pwmload(&self) -> Pwmload { 
        self.pwmload_reg().read()
    }

    #[doc="Write the PWMLOAD register."]
    #[inline] pub fn write_pwmload(&self, value: Pwmload) -> &Self { 
        self.pwmload_reg().write(value);
        self
    }

    #[doc="Set the PWMLOAD register."]
    #[inline] pub fn set_pwmload<F: FnOnce(Pwmload) -> Pwmload>(&self, f: F) -> &Self {
        self.pwmload_reg().set(f);
        self
    }

    #[doc="Modify the PWMLOAD register."]
    #[inline] pub fn with_pwmload<F: FnOnce(Pwmload) -> Pwmload>(&self, f: F) -> &Self {
        self.pwmload_reg().with(f);
        self
    }

}

#[doc="Status And Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc(pub u32);
impl Sc {
    #[doc="Prescale Factor Selection"]
    #[inline] pub fn ps(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps() != 0
    }

    #[doc="Sets the PS field."]
    #[inline] pub fn set_ps<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Source Selection"]
    #[inline] pub fn clks(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if CLKS != 0"]
    #[inline] pub fn test_clks(&self) -> bool {
        self.clks() != 0
    }

    #[doc="Sets the CLKS field."]
    #[inline] pub fn set_clks<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Center-Aligned PWM Select"]
    #[inline] pub fn cpwms(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CPWMS != 0"]
    #[inline] pub fn test_cpwms(&self) -> bool {
        self.cpwms() != 0
    }

    #[doc="Sets the CPWMS field."]
    #[inline] pub fn set_cpwms<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Timer Overflow Interrupt Enable"]
    #[inline] pub fn toie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TOIE != 0"]
    #[inline] pub fn test_toie(&self) -> bool {
        self.toie() != 0
    }

    #[doc="Sets the TOIE field."]
    #[inline] pub fn set_toie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Timer Overflow Flag"]
    #[inline] pub fn tof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TOF != 0"]
    #[inline] pub fn test_tof(&self) -> bool {
        self.tof() != 0
    }

    #[doc="Sets the TOF field."]
    #[inline] pub fn set_tof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="PWM Enable"]
    #[inline] pub fn pwmen<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 16 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PWMEN != 0"]
    #[inline] pub fn test_pwmen<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.pwmen(index) != 0
    }

    #[doc="Sets the PWMEN field."]
    #[inline] pub fn set_pwmen<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 16 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Filter Prescaler"]
    #[inline] pub fn fltps(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if FLTPS != 0"]
    #[inline] pub fn test_fltps(&self) -> bool {
        self.fltps() != 0
    }

    #[doc="Sets the FLTPS field."]
    #[inline] pub fn set_fltps<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Sc {
    #[inline]
    fn from(other: u32) -> Self {
         Sc(other)
    }
}

impl ::core::fmt::Display for Sc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ps() != 0 { try!(write!(f, " ps=0x{:x}", self.ps()))}
        if self.clks() != 0 { try!(write!(f, " clks=0x{:x}", self.clks()))}
        if self.cpwms() != 0 { try!(write!(f, " cpwms"))}
        if self.toie() != 0 { try!(write!(f, " toie"))}
        if self.tof() != 0 { try!(write!(f, " tof"))}
        if self.pwmen(0) != 0 { try!(write!(f, " pwmen[0]"))}
        if self.pwmen(1) != 0 { try!(write!(f, " pwmen[1]"))}
        if self.pwmen(2) != 0 { try!(write!(f, " pwmen[2]"))}
        if self.pwmen(3) != 0 { try!(write!(f, " pwmen[3]"))}
        if self.pwmen(4) != 0 { try!(write!(f, " pwmen[4]"))}
        if self.pwmen(5) != 0 { try!(write!(f, " pwmen[5]"))}
        if self.pwmen(6) != 0 { try!(write!(f, " pwmen[6]"))}
        if self.pwmen(7) != 0 { try!(write!(f, " pwmen[7]"))}
        if self.fltps() != 0 { try!(write!(f, " fltps=0x{:x}", self.fltps()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="Counter Value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cnt(other)
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
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Modulo"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mod(pub u32);
impl Mod {
    #[doc="Modulo Value"]
    #[inline] pub fn _mod(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MOD != 0"]
    #[inline] pub fn test_mod(&self) -> bool {
        self._mod() != 0
    }

    #[doc="Sets the MOD field."]
    #[inline] pub fn set_mod<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mod {
    #[inline]
    fn from(other: u32) -> Self {
         Mod(other)
    }
}

impl ::core::fmt::Display for Mod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._mod() != 0 { try!(write!(f, " mod=0x{:x}", self._mod()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel (n) Status And Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csc(pub u32);
impl Csc {
    #[doc="DMA Enable"]
    #[inline] pub fn dma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA != 0"]
    #[inline] pub fn test_dma(&self) -> bool {
        self.dma() != 0
    }

    #[doc="Sets the DMA field."]
    #[inline] pub fn set_dma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Edge or Level Select"]
    #[inline] pub fn elsa(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ELSA != 0"]
    #[inline] pub fn test_elsa(&self) -> bool {
        self.elsa() != 0
    }

    #[doc="Sets the ELSA field."]
    #[inline] pub fn set_elsa<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Edge or Level Select"]
    #[inline] pub fn elsb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ELSB != 0"]
    #[inline] pub fn test_elsb(&self) -> bool {
        self.elsb() != 0
    }

    #[doc="Sets the ELSB field."]
    #[inline] pub fn set_elsb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel Mode Select"]
    #[inline] pub fn msa(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MSA != 0"]
    #[inline] pub fn test_msa(&self) -> bool {
        self.msa() != 0
    }

    #[doc="Sets the MSA field."]
    #[inline] pub fn set_msa<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel Mode Select"]
    #[inline] pub fn msb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MSB != 0"]
    #[inline] pub fn test_msb(&self) -> bool {
        self.msb() != 0
    }

    #[doc="Sets the MSB field."]
    #[inline] pub fn set_msb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel Interrupt Enable"]
    #[inline] pub fn chie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CHIE != 0"]
    #[inline] pub fn test_chie(&self) -> bool {
        self.chie() != 0
    }

    #[doc="Sets the CHIE field."]
    #[inline] pub fn set_chie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel Flag"]
    #[inline] pub fn chf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHF != 0"]
    #[inline] pub fn test_chf(&self) -> bool {
        self.chf() != 0
    }

    #[doc="Sets the CHF field."]
    #[inline] pub fn set_chf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Csc {
    #[inline]
    fn from(other: u32) -> Self {
         Csc(other)
    }
}

impl ::core::fmt::Display for Csc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dma() != 0 { try!(write!(f, " dma"))}
        if self.elsa() != 0 { try!(write!(f, " elsa"))}
        if self.elsb() != 0 { try!(write!(f, " elsb"))}
        if self.msa() != 0 { try!(write!(f, " msa"))}
        if self.msb() != 0 { try!(write!(f, " msb"))}
        if self.chie() != 0 { try!(write!(f, " chie"))}
        if self.chf() != 0 { try!(write!(f, " chf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel (n) Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
    #[doc="Channel Value"]
    #[inline] pub fn val(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if VAL != 0"]
    #[inline] pub fn test_val(&self) -> bool {
        self.val() != 0
    }

    #[doc="Sets the VAL field."]
    #[inline] pub fn set_val<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cv {
    #[inline]
    fn from(other: u32) -> Self {
         Cv(other)
    }
}

impl ::core::fmt::Display for Cv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.val() != 0 { try!(write!(f, " val=0x{:x}", self.val()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter Initial Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cntin(pub u32);
impl Cntin {
    #[doc="Initial Value Of The FTM Counter"]
    #[inline] pub fn init(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cntin {
    #[inline]
    fn from(other: u32) -> Self {
         Cntin(other)
    }
}

impl ::core::fmt::Display for Cntin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cntin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.init() != 0 { try!(write!(f, " init=0x{:x}", self.init()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture And Compare Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="Channel n Flag"]
    #[inline] pub fn chf<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHF != 0"]
    #[inline] pub fn test_chf<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.chf(index) != 0
    }

    #[doc="Sets the CHF field."]
    #[inline] pub fn set_chf<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Status {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.chf(0) != 0 { try!(write!(f, " chf[0]"))}
        if self.chf(1) != 0 { try!(write!(f, " chf[1]"))}
        if self.chf(2) != 0 { try!(write!(f, " chf[2]"))}
        if self.chf(3) != 0 { try!(write!(f, " chf[3]"))}
        if self.chf(4) != 0 { try!(write!(f, " chf[4]"))}
        if self.chf(5) != 0 { try!(write!(f, " chf[5]"))}
        if self.chf(6) != 0 { try!(write!(f, " chf[6]"))}
        if self.chf(7) != 0 { try!(write!(f, " chf[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Features Mode Selection"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc="FTM Enable"]
    #[inline] pub fn ftmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FTMEN != 0"]
    #[inline] pub fn test_ftmen(&self) -> bool {
        self.ftmen() != 0
    }

    #[doc="Sets the FTMEN field."]
    #[inline] pub fn set_ftmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Initialize The Channels Output"]
    #[inline] pub fn init(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Write Protection Disable"]
    #[inline] pub fn wpdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WPDIS != 0"]
    #[inline] pub fn test_wpdis(&self) -> bool {
        self.wpdis() != 0
    }

    #[doc="Sets the WPDIS field."]
    #[inline] pub fn set_wpdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="PWM Synchronization Mode"]
    #[inline] pub fn pwmsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PWMSYNC != 0"]
    #[inline] pub fn test_pwmsync(&self) -> bool {
        self.pwmsync() != 0
    }

    #[doc="Sets the PWMSYNC field."]
    #[inline] pub fn set_pwmsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Capture Test Mode Enable"]
    #[inline] pub fn captest(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CAPTEST != 0"]
    #[inline] pub fn test_captest(&self) -> bool {
        self.captest() != 0
    }

    #[doc="Sets the CAPTEST field."]
    #[inline] pub fn set_captest<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Fault Control Mode"]
    #[inline] pub fn faultm(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if FAULTM != 0"]
    #[inline] pub fn test_faultm(&self) -> bool {
        self.faultm() != 0
    }

    #[doc="Sets the FAULTM field."]
    #[inline] pub fn set_faultm<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Fault Interrupt Enable"]
    #[inline] pub fn faultie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FAULTIE != 0"]
    #[inline] pub fn test_faultie(&self) -> bool {
        self.faultie() != 0
    }

    #[doc="Sets the FAULTIE field."]
    #[inline] pub fn set_faultie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Mode {
    #[inline]
    fn from(other: u32) -> Self {
         Mode(other)
    }
}

impl ::core::fmt::Display for Mode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ftmen() != 0 { try!(write!(f, " ftmen"))}
        if self.init() != 0 { try!(write!(f, " init"))}
        if self.wpdis() != 0 { try!(write!(f, " wpdis"))}
        if self.pwmsync() != 0 { try!(write!(f, " pwmsync"))}
        if self.captest() != 0 { try!(write!(f, " captest"))}
        if self.faultm() != 0 { try!(write!(f, " faultm=0x{:x}", self.faultm()))}
        if self.faultie() != 0 { try!(write!(f, " faultie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
    #[doc="Minimum Loading Point Enable"]
    #[inline] pub fn cntmin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CNTMIN != 0"]
    #[inline] pub fn test_cntmin(&self) -> bool {
        self.cntmin() != 0
    }

    #[doc="Sets the CNTMIN field."]
    #[inline] pub fn set_cntmin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Maximum Loading Point Enable"]
    #[inline] pub fn cntmax(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CNTMAX != 0"]
    #[inline] pub fn test_cntmax(&self) -> bool {
        self.cntmax() != 0
    }

    #[doc="Sets the CNTMAX field."]
    #[inline] pub fn set_cntmax<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FTM Counter Reinitialization By Synchronization (FTM counter synchronization)"]
    #[inline] pub fn reinit(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if REINIT != 0"]
    #[inline] pub fn test_reinit(&self) -> bool {
        self.reinit() != 0
    }

    #[doc="Sets the REINIT field."]
    #[inline] pub fn set_reinit<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Output Mask Synchronization"]
    #[inline] pub fn synchom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCHOM != 0"]
    #[inline] pub fn test_synchom(&self) -> bool {
        self.synchom() != 0
    }

    #[doc="Sets the SYNCHOM field."]
    #[inline] pub fn set_synchom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PWM Synchronization Hardware Trigger 0"]
    #[inline] pub fn trig0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TRIG0 != 0"]
    #[inline] pub fn test_trig0(&self) -> bool {
        self.trig0() != 0
    }

    #[doc="Sets the TRIG0 field."]
    #[inline] pub fn set_trig0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PWM Synchronization Hardware Trigger 1"]
    #[inline] pub fn trig1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TRIG1 != 0"]
    #[inline] pub fn test_trig1(&self) -> bool {
        self.trig1() != 0
    }

    #[doc="Sets the TRIG1 field."]
    #[inline] pub fn set_trig1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PWM Synchronization Hardware Trigger 2"]
    #[inline] pub fn trig2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TRIG2 != 0"]
    #[inline] pub fn test_trig2(&self) -> bool {
        self.trig2() != 0
    }

    #[doc="Sets the TRIG2 field."]
    #[inline] pub fn set_trig2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PWM Synchronization Software Trigger"]
    #[inline] pub fn swsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SWSYNC != 0"]
    #[inline] pub fn test_swsync(&self) -> bool {
        self.swsync() != 0
    }

    #[doc="Sets the SWSYNC field."]
    #[inline] pub fn set_swsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Sync {
    #[inline]
    fn from(other: u32) -> Self {
         Sync(other)
    }
}

impl ::core::fmt::Display for Sync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntmin() != 0 { try!(write!(f, " cntmin"))}
        if self.cntmax() != 0 { try!(write!(f, " cntmax"))}
        if self.reinit() != 0 { try!(write!(f, " reinit"))}
        if self.synchom() != 0 { try!(write!(f, " synchom"))}
        if self.trig0() != 0 { try!(write!(f, " trig0"))}
        if self.trig1() != 0 { try!(write!(f, " trig1"))}
        if self.trig2() != 0 { try!(write!(f, " trig2"))}
        if self.swsync() != 0 { try!(write!(f, " swsync"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Initial State For Channels Output"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outinit(pub u32);
impl Outinit {
    #[doc="Channel n Output Initialization Value"]
    #[inline] pub fn choi<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHOI != 0"]
    #[inline] pub fn test_choi<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.choi(index) != 0
    }

    #[doc="Sets the CHOI field."]
    #[inline] pub fn set_choi<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Outinit {
    #[inline]
    fn from(other: u32) -> Self {
         Outinit(other)
    }
}

impl ::core::fmt::Display for Outinit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outinit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.choi(0) != 0 { try!(write!(f, " choi[0]"))}
        if self.choi(1) != 0 { try!(write!(f, " choi[1]"))}
        if self.choi(2) != 0 { try!(write!(f, " choi[2]"))}
        if self.choi(3) != 0 { try!(write!(f, " choi[3]"))}
        if self.choi(4) != 0 { try!(write!(f, " choi[4]"))}
        if self.choi(5) != 0 { try!(write!(f, " choi[5]"))}
        if self.choi(6) != 0 { try!(write!(f, " choi[6]"))}
        if self.choi(7) != 0 { try!(write!(f, " choi[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Output Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outmask(pub u32);
impl Outmask {
    #[doc="Channel 0 Output Mask"]
    #[inline] pub fn chom<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHOM != 0"]
    #[inline] pub fn test_chom<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.chom(index) != 0
    }

    #[doc="Sets the CHOM field."]
    #[inline] pub fn set_chom<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Outmask {
    #[inline]
    fn from(other: u32) -> Self {
         Outmask(other)
    }
}

impl ::core::fmt::Display for Outmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.chom(0) != 0 { try!(write!(f, " chom[0]"))}
        if self.chom(1) != 0 { try!(write!(f, " chom[1]"))}
        if self.chom(2) != 0 { try!(write!(f, " chom[2]"))}
        if self.chom(3) != 0 { try!(write!(f, " chom[3]"))}
        if self.chom(4) != 0 { try!(write!(f, " chom[4]"))}
        if self.chom(5) != 0 { try!(write!(f, " chom[5]"))}
        if self.chom(6) != 0 { try!(write!(f, " chom[6]"))}
        if self.chom(7) != 0 { try!(write!(f, " chom[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Function For Linked Channels"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Combine(pub u32);
impl Combine {
    #[doc="Combine Channels For n"]
    #[inline] pub fn combine<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMBINE != 0"]
    #[inline] pub fn test_combine<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.combine(index) != 0
    }

    #[doc="Sets the COMBINE field."]
    #[inline] pub fn set_combine<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Complement Of Channel (n) For n"]
    #[inline] pub fn comp<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMP != 0"]
    #[inline] pub fn test_comp<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.comp(index) != 0
    }

    #[doc="Sets the COMP field."]
    #[inline] pub fn set_comp<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Dual Edge Capture Mode Enable For n"]
    #[inline] pub fn decapen<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DECAPEN != 0"]
    #[inline] pub fn test_decapen<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.decapen(index) != 0
    }

    #[doc="Sets the DECAPEN field."]
    #[inline] pub fn set_decapen<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Dual Edge Capture Mode Captures For n"]
    #[inline] pub fn decap<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 3 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DECAP != 0"]
    #[inline] pub fn test_decap<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.decap(index) != 0
    }

    #[doc="Sets the DECAP field."]
    #[inline] pub fn set_decap<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 3 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Deadtime Enable For n"]
    #[inline] pub fn dten<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 4 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DTEN != 0"]
    #[inline] pub fn test_dten<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.dten(index) != 0
    }

    #[doc="Sets the DTEN field."]
    #[inline] pub fn set_dten<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 4 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Synchronization Enable For n"]
    #[inline] pub fn syncen<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 5 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SYNCEN != 0"]
    #[inline] pub fn test_syncen<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.syncen(index) != 0
    }

    #[doc="Sets the SYNCEN field."]
    #[inline] pub fn set_syncen<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 5 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Fault Control Enable For n"]
    #[inline] pub fn faulten<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 6 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FAULTEN != 0"]
    #[inline] pub fn test_faulten<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.faulten(index) != 0
    }

    #[doc="Sets the FAULTEN field."]
    #[inline] pub fn set_faulten<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 6 + (index << 3);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Combine {
    #[inline]
    fn from(other: u32) -> Self {
         Combine(other)
    }
}

impl ::core::fmt::Display for Combine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Combine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.combine(0) != 0 { try!(write!(f, " combine[0]"))}
        if self.combine(1) != 0 { try!(write!(f, " combine[1]"))}
        if self.combine(2) != 0 { try!(write!(f, " combine[2]"))}
        if self.combine(3) != 0 { try!(write!(f, " combine[3]"))}
        if self.comp(0) != 0 { try!(write!(f, " comp[0]"))}
        if self.comp(1) != 0 { try!(write!(f, " comp[1]"))}
        if self.comp(2) != 0 { try!(write!(f, " comp[2]"))}
        if self.comp(3) != 0 { try!(write!(f, " comp[3]"))}
        if self.decapen(0) != 0 { try!(write!(f, " decapen[0]"))}
        if self.decapen(1) != 0 { try!(write!(f, " decapen[1]"))}
        if self.decapen(2) != 0 { try!(write!(f, " decapen[2]"))}
        if self.decapen(3) != 0 { try!(write!(f, " decapen[3]"))}
        if self.decap(0) != 0 { try!(write!(f, " decap[0]"))}
        if self.decap(1) != 0 { try!(write!(f, " decap[1]"))}
        if self.decap(2) != 0 { try!(write!(f, " decap[2]"))}
        if self.decap(3) != 0 { try!(write!(f, " decap[3]"))}
        if self.dten(0) != 0 { try!(write!(f, " dten[0]"))}
        if self.dten(1) != 0 { try!(write!(f, " dten[1]"))}
        if self.dten(2) != 0 { try!(write!(f, " dten[2]"))}
        if self.dten(3) != 0 { try!(write!(f, " dten[3]"))}
        if self.syncen(0) != 0 { try!(write!(f, " syncen[0]"))}
        if self.syncen(1) != 0 { try!(write!(f, " syncen[1]"))}
        if self.syncen(2) != 0 { try!(write!(f, " syncen[2]"))}
        if self.syncen(3) != 0 { try!(write!(f, " syncen[3]"))}
        if self.faulten(0) != 0 { try!(write!(f, " faulten[0]"))}
        if self.faulten(1) != 0 { try!(write!(f, " faulten[1]"))}
        if self.faulten(2) != 0 { try!(write!(f, " faulten[2]"))}
        if self.faulten(3) != 0 { try!(write!(f, " faulten[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Deadtime Insertion Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Deadtime(pub u32);
impl Deadtime {
    #[doc="Deadtime Value"]
    #[inline] pub fn dtval(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DTVAL != 0"]
    #[inline] pub fn test_dtval(&self) -> bool {
        self.dtval() != 0
    }

    #[doc="Sets the DTVAL field."]
    #[inline] pub fn set_dtval<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Deadtime Prescaler Value"]
    #[inline] pub fn dtps(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if DTPS != 0"]
    #[inline] pub fn test_dtps(&self) -> bool {
        self.dtps() != 0
    }

    #[doc="Sets the DTPS field."]
    #[inline] pub fn set_dtps<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Deadtime {
    #[inline]
    fn from(other: u32) -> Self {
         Deadtime(other)
    }
}

impl ::core::fmt::Display for Deadtime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Deadtime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtval() != 0 { try!(write!(f, " dtval=0x{:x}", self.dtval()))}
        if self.dtps() != 0 { try!(write!(f, " dtps=0x{:x}", self.dtps()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM External Trigger"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exttrig(pub u32);
impl Exttrig {
    #[doc="Channel 2 Trigger Enable"]
    #[inline] pub fn ch2trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH2TRIG != 0"]
    #[inline] pub fn test_ch2trig(&self) -> bool {
        self.ch2trig() != 0
    }

    #[doc="Sets the CH2TRIG field."]
    #[inline] pub fn set_ch2trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 3 Trigger Enable"]
    #[inline] pub fn ch3trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH3TRIG != 0"]
    #[inline] pub fn test_ch3trig(&self) -> bool {
        self.ch3trig() != 0
    }

    #[doc="Sets the CH3TRIG field."]
    #[inline] pub fn set_ch3trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 4 Trigger Enable"]
    #[inline] pub fn ch4trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH4TRIG != 0"]
    #[inline] pub fn test_ch4trig(&self) -> bool {
        self.ch4trig() != 0
    }

    #[doc="Sets the CH4TRIG field."]
    #[inline] pub fn set_ch4trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 5 Trigger Enable"]
    #[inline] pub fn ch5trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH5TRIG != 0"]
    #[inline] pub fn test_ch5trig(&self) -> bool {
        self.ch5trig() != 0
    }

    #[doc="Sets the CH5TRIG field."]
    #[inline] pub fn set_ch5trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 0 Trigger Enable"]
    #[inline] pub fn ch0trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH0TRIG != 0"]
    #[inline] pub fn test_ch0trig(&self) -> bool {
        self.ch0trig() != 0
    }

    #[doc="Sets the CH0TRIG field."]
    #[inline] pub fn set_ch0trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 1 Trigger Enable"]
    #[inline] pub fn ch1trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH1TRIG != 0"]
    #[inline] pub fn test_ch1trig(&self) -> bool {
        self.ch1trig() != 0
    }

    #[doc="Sets the CH1TRIG field."]
    #[inline] pub fn set_ch1trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Initialization Trigger Enable"]
    #[inline] pub fn inittrigen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INITTRIGEN != 0"]
    #[inline] pub fn test_inittrigen(&self) -> bool {
        self.inittrigen() != 0
    }

    #[doc="Sets the INITTRIGEN field."]
    #[inline] pub fn set_inittrigen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel Trigger Flag"]
    #[inline] pub fn trigf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TRIGF != 0"]
    #[inline] pub fn test_trigf(&self) -> bool {
        self.trigf() != 0
    }

    #[doc="Sets the TRIGF field."]
    #[inline] pub fn set_trigf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Exttrig {
    #[inline]
    fn from(other: u32) -> Self {
         Exttrig(other)
    }
}

impl ::core::fmt::Display for Exttrig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exttrig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch2trig() != 0 { try!(write!(f, " ch2trig"))}
        if self.ch3trig() != 0 { try!(write!(f, " ch3trig"))}
        if self.ch4trig() != 0 { try!(write!(f, " ch4trig"))}
        if self.ch5trig() != 0 { try!(write!(f, " ch5trig"))}
        if self.ch0trig() != 0 { try!(write!(f, " ch0trig"))}
        if self.ch1trig() != 0 { try!(write!(f, " ch1trig"))}
        if self.inittrigen() != 0 { try!(write!(f, " inittrigen"))}
        if self.trigf() != 0 { try!(write!(f, " trigf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channels Polarity"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pol(pub u32);
impl Pol {
    #[doc="Channel n Polarity"]
    #[inline] pub fn pol<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if POL != 0"]
    #[inline] pub fn test_pol<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.pol(index) != 0
    }

    #[doc="Sets the POL field."]
    #[inline] pub fn set_pol<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Pol {
    #[inline]
    fn from(other: u32) -> Self {
         Pol(other)
    }
}

impl ::core::fmt::Display for Pol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pol(0) != 0 { try!(write!(f, " pol[0]"))}
        if self.pol(1) != 0 { try!(write!(f, " pol[1]"))}
        if self.pol(2) != 0 { try!(write!(f, " pol[2]"))}
        if self.pol(3) != 0 { try!(write!(f, " pol[3]"))}
        if self.pol(4) != 0 { try!(write!(f, " pol[4]"))}
        if self.pol(5) != 0 { try!(write!(f, " pol[5]"))}
        if self.pol(6) != 0 { try!(write!(f, " pol[6]"))}
        if self.pol(7) != 0 { try!(write!(f, " pol[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fault Mode Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fms(pub u32);
impl Fms {
    #[doc="Fault Detection Flag 0"]
    #[inline] pub fn faultf0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FAULTF0 != 0"]
    #[inline] pub fn test_faultf0(&self) -> bool {
        self.faultf0() != 0
    }

    #[doc="Sets the FAULTF0 field."]
    #[inline] pub fn set_faultf0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fault Detection Flag 1"]
    #[inline] pub fn faultf1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FAULTF1 != 0"]
    #[inline] pub fn test_faultf1(&self) -> bool {
        self.faultf1() != 0
    }

    #[doc="Sets the FAULTF1 field."]
    #[inline] pub fn set_faultf1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Fault Detection Flag 2"]
    #[inline] pub fn faultf2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FAULTF2 != 0"]
    #[inline] pub fn test_faultf2(&self) -> bool {
        self.faultf2() != 0
    }

    #[doc="Sets the FAULTF2 field."]
    #[inline] pub fn set_faultf2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Fault Detection Flag 3"]
    #[inline] pub fn faultf3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FAULTF3 != 0"]
    #[inline] pub fn test_faultf3(&self) -> bool {
        self.faultf3() != 0
    }

    #[doc="Sets the FAULTF3 field."]
    #[inline] pub fn set_faultf3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Fault Inputs"]
    #[inline] pub fn faultin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FAULTIN != 0"]
    #[inline] pub fn test_faultin(&self) -> bool {
        self.faultin() != 0
    }

    #[doc="Sets the FAULTIN field."]
    #[inline] pub fn set_faultin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Write Protection Enable"]
    #[inline] pub fn wpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if WPEN != 0"]
    #[inline] pub fn test_wpen(&self) -> bool {
        self.wpen() != 0
    }

    #[doc="Sets the WPEN field."]
    #[inline] pub fn set_wpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Fault Detection Flag"]
    #[inline] pub fn faultf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FAULTF != 0"]
    #[inline] pub fn test_faultf(&self) -> bool {
        self.faultf() != 0
    }

    #[doc="Sets the FAULTF field."]
    #[inline] pub fn set_faultf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Fms {
    #[inline]
    fn from(other: u32) -> Self {
         Fms(other)
    }
}

impl ::core::fmt::Display for Fms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.faultf0() != 0 { try!(write!(f, " faultf0"))}
        if self.faultf1() != 0 { try!(write!(f, " faultf1"))}
        if self.faultf2() != 0 { try!(write!(f, " faultf2"))}
        if self.faultf3() != 0 { try!(write!(f, " faultf3"))}
        if self.faultin() != 0 { try!(write!(f, " faultin"))}
        if self.wpen() != 0 { try!(write!(f, " wpen"))}
        if self.faultf() != 0 { try!(write!(f, " faultf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Input Capture Filter Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Filter(pub u32);
impl Filter {
    #[doc="Channel n Input Filter"]
    #[inline] pub fn chfval<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 2);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CHFVAL != 0"]
    #[inline] pub fn test_chfval<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.chfval(index) != 0
    }

    #[doc="Sets the CHFVAL field."]
    #[inline] pub fn set_chfval<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 2);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Filter {
    #[inline]
    fn from(other: u32) -> Self {
         Filter(other)
    }
}

impl ::core::fmt::Display for Filter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Filter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.chfval(0) != 0 { try!(write!(f, " chfval[0]=0x{:x}", self.chfval(0)))}
        if self.chfval(1) != 0 { try!(write!(f, " chfval[1]=0x{:x}", self.chfval(1)))}
        if self.chfval(2) != 0 { try!(write!(f, " chfval[2]=0x{:x}", self.chfval(2)))}
        if self.chfval(3) != 0 { try!(write!(f, " chfval[3]=0x{:x}", self.chfval(3)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fault Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltctrl(pub u32);
impl Fltctrl {
    #[doc="Fault Input n Enable"]
    #[inline] pub fn faulten<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FAULTEN != 0"]
    #[inline] pub fn test_faulten<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.faulten(index) != 0
    }

    #[doc="Sets the FAULTEN field."]
    #[inline] pub fn set_faulten<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Fault Input 0 Filter Enable"]
    #[inline] pub fn ffltren<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 4 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FFLTREN != 0"]
    #[inline] pub fn test_ffltren<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.ffltren(index) != 0
    }

    #[doc="Sets the FFLTREN field."]
    #[inline] pub fn set_ffltren<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 4 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Fault Input Filter"]
    #[inline] pub fn ffval(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if FFVAL != 0"]
    #[inline] pub fn test_ffval(&self) -> bool {
        self.ffval() != 0
    }

    #[doc="Sets the FFVAL field."]
    #[inline] pub fn set_ffval<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Fltctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Fltctrl(other)
    }
}

impl ::core::fmt::Display for Fltctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.faulten(0) != 0 { try!(write!(f, " faulten[0]"))}
        if self.faulten(1) != 0 { try!(write!(f, " faulten[1]"))}
        if self.faulten(2) != 0 { try!(write!(f, " faulten[2]"))}
        if self.faulten(3) != 0 { try!(write!(f, " faulten[3]"))}
        if self.ffltren(0) != 0 { try!(write!(f, " ffltren[0]"))}
        if self.ffltren(1) != 0 { try!(write!(f, " ffltren[1]"))}
        if self.ffltren(2) != 0 { try!(write!(f, " ffltren[2]"))}
        if self.ffltren(3) != 0 { try!(write!(f, " ffltren[3]"))}
        if self.ffval() != 0 { try!(write!(f, " ffval=0x{:x}", self.ffval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Quadrature Decoder Control And Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Qdctrl(pub u32);
impl Qdctrl {
    #[doc="Quadrature Decoder Mode Enable"]
    #[inline] pub fn quaden(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if QUADEN != 0"]
    #[inline] pub fn test_quaden(&self) -> bool {
        self.quaden() != 0
    }

    #[doc="Sets the QUADEN field."]
    #[inline] pub fn set_quaden<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Timer Overflow Direction In Quadrature Decoder Mode"]
    #[inline] pub fn tofdir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TOFDIR != 0"]
    #[inline] pub fn test_tofdir(&self) -> bool {
        self.tofdir() != 0
    }

    #[doc="Sets the TOFDIR field."]
    #[inline] pub fn set_tofdir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FTM Counter Direction In Quadrature Decoder Mode"]
    #[inline] pub fn quadir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if QUADIR != 0"]
    #[inline] pub fn test_quadir(&self) -> bool {
        self.quadir() != 0
    }

    #[doc="Sets the QUADIR field."]
    #[inline] pub fn set_quadir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Quadrature Decoder Mode"]
    #[inline] pub fn quadmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if QUADMODE != 0"]
    #[inline] pub fn test_quadmode(&self) -> bool {
        self.quadmode() != 0
    }

    #[doc="Sets the QUADMODE field."]
    #[inline] pub fn set_quadmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Phase B Input Polarity"]
    #[inline] pub fn phbpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PHBPOL != 0"]
    #[inline] pub fn test_phbpol(&self) -> bool {
        self.phbpol() != 0
    }

    #[doc="Sets the PHBPOL field."]
    #[inline] pub fn set_phbpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Phase A Input Polarity"]
    #[inline] pub fn phapol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PHAPOL != 0"]
    #[inline] pub fn test_phapol(&self) -> bool {
        self.phapol() != 0
    }

    #[doc="Sets the PHAPOL field."]
    #[inline] pub fn set_phapol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Phase B Input Filter Enable"]
    #[inline] pub fn phbfltren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PHBFLTREN != 0"]
    #[inline] pub fn test_phbfltren(&self) -> bool {
        self.phbfltren() != 0
    }

    #[doc="Sets the PHBFLTREN field."]
    #[inline] pub fn set_phbfltren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Phase A Input Filter Enable"]
    #[inline] pub fn phafltren(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PHAFLTREN != 0"]
    #[inline] pub fn test_phafltren(&self) -> bool {
        self.phafltren() != 0
    }

    #[doc="Sets the PHAFLTREN field."]
    #[inline] pub fn set_phafltren<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Qdctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Qdctrl(other)
    }
}

impl ::core::fmt::Display for Qdctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Qdctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.quaden() != 0 { try!(write!(f, " quaden"))}
        if self.tofdir() != 0 { try!(write!(f, " tofdir"))}
        if self.quadir() != 0 { try!(write!(f, " quadir"))}
        if self.quadmode() != 0 { try!(write!(f, " quadmode"))}
        if self.phbpol() != 0 { try!(write!(f, " phbpol"))}
        if self.phapol() != 0 { try!(write!(f, " phapol"))}
        if self.phbfltren() != 0 { try!(write!(f, " phbfltren"))}
        if self.phafltren() != 0 { try!(write!(f, " phafltren"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Conf(pub u32);
impl Conf {
    #[doc="TOF Frequency"]
    #[inline] pub fn numtof(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if NUMTOF != 0"]
    #[inline] pub fn test_numtof(&self) -> bool {
        self.numtof() != 0
    }

    #[doc="Sets the NUMTOF field."]
    #[inline] pub fn set_numtof<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="BDM Mode"]
    #[inline] pub fn bdmmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if BDMMODE != 0"]
    #[inline] pub fn test_bdmmode(&self) -> bool {
        self.bdmmode() != 0
    }

    #[doc="Sets the BDMMODE field."]
    #[inline] pub fn set_bdmmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Global Time Base Enable"]
    #[inline] pub fn gtbeen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GTBEEN != 0"]
    #[inline] pub fn test_gtbeen(&self) -> bool {
        self.gtbeen() != 0
    }

    #[doc="Sets the GTBEEN field."]
    #[inline] pub fn set_gtbeen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Global Time Base Output"]
    #[inline] pub fn gtbeout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GTBEOUT != 0"]
    #[inline] pub fn test_gtbeout(&self) -> bool {
        self.gtbeout() != 0
    }

    #[doc="Sets the GTBEOUT field."]
    #[inline] pub fn set_gtbeout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

}

impl From<u32> for Conf {
    #[inline]
    fn from(other: u32) -> Self {
         Conf(other)
    }
}

impl ::core::fmt::Display for Conf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Conf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.numtof() != 0 { try!(write!(f, " numtof=0x{:x}", self.numtof()))}
        if self.bdmmode() != 0 { try!(write!(f, " bdmmode=0x{:x}", self.bdmmode()))}
        if self.gtbeen() != 0 { try!(write!(f, " gtbeen"))}
        if self.gtbeout() != 0 { try!(write!(f, " gtbeout"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM Fault Input Polarity"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltpol(pub u32);
impl Fltpol {
    #[doc="Fault Input 0 Polarity"]
    #[inline] pub fn fltpol<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FLTPOL != 0"]
    #[inline] pub fn test_fltpol<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.fltpol(index) != 0
    }

    #[doc="Sets the FLTPOL field."]
    #[inline] pub fn set_fltpol<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Fltpol {
    #[inline]
    fn from(other: u32) -> Self {
         Fltpol(other)
    }
}

impl ::core::fmt::Display for Fltpol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltpol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fltpol(0) != 0 { try!(write!(f, " fltpol[0]"))}
        if self.fltpol(1) != 0 { try!(write!(f, " fltpol[1]"))}
        if self.fltpol(2) != 0 { try!(write!(f, " fltpol[2]"))}
        if self.fltpol(3) != 0 { try!(write!(f, " fltpol[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Synconf(pub u32);
impl Synconf {
    #[doc="Hardware Trigger Mode"]
    #[inline] pub fn hwtrigmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HWTRIGMODE != 0"]
    #[inline] pub fn test_hwtrigmode(&self) -> bool {
        self.hwtrigmode() != 0
    }

    #[doc="Sets the HWTRIGMODE field."]
    #[inline] pub fn set_hwtrigmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CNTIN Register Synchronization"]
    #[inline] pub fn cntinc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CNTINC != 0"]
    #[inline] pub fn test_cntinc(&self) -> bool {
        self.cntinc() != 0
    }

    #[doc="Sets the CNTINC field."]
    #[inline] pub fn set_cntinc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="INVCTRL Register Synchronization"]
    #[inline] pub fn invc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if INVC != 0"]
    #[inline] pub fn test_invc(&self) -> bool {
        self.invc() != 0
    }

    #[doc="Sets the INVC field."]
    #[inline] pub fn set_invc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SWOCTRL Register Synchronization"]
    #[inline] pub fn swoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SWOC != 0"]
    #[inline] pub fn test_swoc(&self) -> bool {
        self.swoc() != 0
    }

    #[doc="Sets the SWOC field."]
    #[inline] pub fn set_swoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Synchronization Mode"]
    #[inline] pub fn syncmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNCMODE != 0"]
    #[inline] pub fn test_syncmode(&self) -> bool {
        self.syncmode() != 0
    }

    #[doc="Sets the SYNCMODE field."]
    #[inline] pub fn set_syncmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="FTM counter synchronization is activated by the software trigger."]
    #[inline] pub fn swrstcnt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SWRSTCNT != 0"]
    #[inline] pub fn test_swrstcnt(&self) -> bool {
        self.swrstcnt() != 0
    }

    #[doc="Sets the SWRSTCNT field."]
    #[inline] pub fn set_swrstcnt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MOD, CNTIN, and CV registers synchronization is activated by the software trigger."]
    #[inline] pub fn swwrbuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SWWRBUF != 0"]
    #[inline] pub fn test_swwrbuf(&self) -> bool {
        self.swwrbuf() != 0
    }

    #[doc="Sets the SWWRBUF field."]
    #[inline] pub fn set_swwrbuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Output mask synchronization is activated by the software trigger."]
    #[inline] pub fn swom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SWOM != 0"]
    #[inline] pub fn test_swom(&self) -> bool {
        self.swom() != 0
    }

    #[doc="Sets the SWOM field."]
    #[inline] pub fn set_swom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Inverting control synchronization is activated by the software trigger."]
    #[inline] pub fn swinvc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SWINVC != 0"]
    #[inline] pub fn test_swinvc(&self) -> bool {
        self.swinvc() != 0
    }

    #[doc="Sets the SWINVC field."]
    #[inline] pub fn set_swinvc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Software output control synchronization is activated by the software trigger."]
    #[inline] pub fn swsoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SWSOC != 0"]
    #[inline] pub fn test_swsoc(&self) -> bool {
        self.swsoc() != 0
    }

    #[doc="Sets the SWSOC field."]
    #[inline] pub fn set_swsoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="FTM counter synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwrstcnt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if HWRSTCNT != 0"]
    #[inline] pub fn test_hwrstcnt(&self) -> bool {
        self.hwrstcnt() != 0
    }

    #[doc="Sets the HWRSTCNT field."]
    #[inline] pub fn set_hwrstcnt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwwrbuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if HWWRBUF != 0"]
    #[inline] pub fn test_hwwrbuf(&self) -> bool {
        self.hwwrbuf() != 0
    }

    #[doc="Sets the HWWRBUF field."]
    #[inline] pub fn set_hwwrbuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Output mask synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if HWOM != 0"]
    #[inline] pub fn test_hwom(&self) -> bool {
        self.hwom() != 0
    }

    #[doc="Sets the HWOM field."]
    #[inline] pub fn set_hwom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Inverting control synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwinvc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if HWINVC != 0"]
    #[inline] pub fn test_hwinvc(&self) -> bool {
        self.hwinvc() != 0
    }

    #[doc="Sets the HWINVC field."]
    #[inline] pub fn set_hwinvc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Software output control synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwsoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if HWSOC != 0"]
    #[inline] pub fn test_hwsoc(&self) -> bool {
        self.hwsoc() != 0
    }

    #[doc="Sets the HWSOC field."]
    #[inline] pub fn set_hwsoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for Synconf {
    #[inline]
    fn from(other: u32) -> Self {
         Synconf(other)
    }
}

impl ::core::fmt::Display for Synconf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Synconf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hwtrigmode() != 0 { try!(write!(f, " hwtrigmode"))}
        if self.cntinc() != 0 { try!(write!(f, " cntinc"))}
        if self.invc() != 0 { try!(write!(f, " invc"))}
        if self.swoc() != 0 { try!(write!(f, " swoc"))}
        if self.syncmode() != 0 { try!(write!(f, " syncmode"))}
        if self.swrstcnt() != 0 { try!(write!(f, " swrstcnt"))}
        if self.swwrbuf() != 0 { try!(write!(f, " swwrbuf"))}
        if self.swom() != 0 { try!(write!(f, " swom"))}
        if self.swinvc() != 0 { try!(write!(f, " swinvc"))}
        if self.swsoc() != 0 { try!(write!(f, " swsoc"))}
        if self.hwrstcnt() != 0 { try!(write!(f, " hwrstcnt"))}
        if self.hwwrbuf() != 0 { try!(write!(f, " hwwrbuf"))}
        if self.hwom() != 0 { try!(write!(f, " hwom"))}
        if self.hwinvc() != 0 { try!(write!(f, " hwinvc"))}
        if self.hwsoc() != 0 { try!(write!(f, " hwsoc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM Inverting Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Invctrl(pub u32);
impl Invctrl {
    #[doc="Pair Channels n Inverting Enable"]
    #[inline] pub fn inven<I: Into<::bobbin_bits::R4>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INVEN != 0"]
    #[inline] pub fn test_inven<I: Into<::bobbin_bits::R4>>(&self, index: I) -> bool{
        self.inven(index) != 0
    }

    #[doc="Sets the INVEN field."]
    #[inline] pub fn set_inven<I: Into<::bobbin_bits::R4>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Invctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Invctrl(other)
    }
}

impl ::core::fmt::Display for Invctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Invctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inven(0) != 0 { try!(write!(f, " inven[0]"))}
        if self.inven(1) != 0 { try!(write!(f, " inven[1]"))}
        if self.inven(2) != 0 { try!(write!(f, " inven[2]"))}
        if self.inven(3) != 0 { try!(write!(f, " inven[3]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM Software Output Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swoctrl(pub u32);
impl Swoctrl {
    #[doc="Channel 0 Software Output Control Enable"]
    #[inline] pub fn choc<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHOC != 0"]
    #[inline] pub fn test_choc<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.choc(index) != 0
    }

    #[doc="Sets the CHOC field."]
    #[inline] pub fn set_choc<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Channel n Software Output Control Value"]
    #[inline] pub fn chocv<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CHOCV != 0"]
    #[inline] pub fn test_chocv<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.chocv(index) != 0
    }

    #[doc="Sets the CHOCV field."]
    #[inline] pub fn set_chocv<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Swoctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Swoctrl(other)
    }
}

impl ::core::fmt::Display for Swoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.choc(0) != 0 { try!(write!(f, " choc[0]"))}
        if self.choc(1) != 0 { try!(write!(f, " choc[1]"))}
        if self.choc(2) != 0 { try!(write!(f, " choc[2]"))}
        if self.choc(3) != 0 { try!(write!(f, " choc[3]"))}
        if self.choc(4) != 0 { try!(write!(f, " choc[4]"))}
        if self.choc(5) != 0 { try!(write!(f, " choc[5]"))}
        if self.choc(6) != 0 { try!(write!(f, " choc[6]"))}
        if self.choc(7) != 0 { try!(write!(f, " choc[7]"))}
        if self.chocv(0) != 0 { try!(write!(f, " chocv[0]"))}
        if self.chocv(1) != 0 { try!(write!(f, " chocv[1]"))}
        if self.chocv(2) != 0 { try!(write!(f, " chocv[2]"))}
        if self.chocv(3) != 0 { try!(write!(f, " chocv[3]"))}
        if self.chocv(4) != 0 { try!(write!(f, " chocv[4]"))}
        if self.chocv(5) != 0 { try!(write!(f, " chocv[5]"))}
        if self.chocv(6) != 0 { try!(write!(f, " chocv[6]"))}
        if self.chocv(7) != 0 { try!(write!(f, " chocv[7]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM PWM Load"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pwmload(pub u32);
impl Pwmload {
    #[doc="Channel n Select"]
    #[inline] pub fn chsel<I: Into<::bobbin_bits::R8>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHSEL != 0"]
    #[inline] pub fn test_chsel<I: Into<::bobbin_bits::R8>>(&self, index: I) -> bool{
        self.chsel(index) != 0
    }

    #[doc="Sets the CHSEL field."]
    #[inline] pub fn set_chsel<I: Into<::bobbin_bits::R8>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="Load Enable"]
    #[inline] pub fn ldok(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LDOK != 0"]
    #[inline] pub fn test_ldok(&self) -> bool {
        self.ldok() != 0
    }

    #[doc="Sets the LDOK field."]
    #[inline] pub fn set_ldok<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Pwmload {
    #[inline]
    fn from(other: u32) -> Self {
         Pwmload(other)
    }
}

impl ::core::fmt::Display for Pwmload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pwmload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.chsel(0) != 0 { try!(write!(f, " chsel[0]"))}
        if self.chsel(1) != 0 { try!(write!(f, " chsel[1]"))}
        if self.chsel(2) != 0 { try!(write!(f, " chsel[2]"))}
        if self.chsel(3) != 0 { try!(write!(f, " chsel[3]"))}
        if self.chsel(4) != 0 { try!(write!(f, " chsel[4]"))}
        if self.chsel(5) != 0 { try!(write!(f, " chsel[5]"))}
        if self.chsel(6) != 0 { try!(write!(f, " chsel[6]"))}
        if self.chsel(7) != 0 { try!(write!(f, " chsel[7]"))}
        if self.ldok() != 0 { try!(write!(f, " ldok"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

