pub use ::bobbin_common::*;
pub use ::bobbin_common::tree::*;
pub use ::hz::Hz;

pub struct ClockTree<T>(T);


// Define Global Clocks

pub struct Osc8m {}
impl Clock for Osc8m { fn hz() -> Hz { Hz::from_num(8000000) } }

pub struct Dfll48m {}
impl Clock for Dfll48m { fn hz() -> Hz { Hz::from_num(48000000) } }

pub struct Fdpll96m {}
impl Clock for Fdpll96m { fn hz() -> Hz { Hz::from_num(96000000) } }


pub trait Clocks {
    type Xosc: Clock;
    type Xosc32k: Clock;
    fn xosc() -> Hz { Self::Xosc::hz() }
    fn xosc32k() -> Hz { Self::Xosc32k::hz() }
    fn osc8m() -> Hz { Hz::from_num(8000000) }
    fn dfll48m() -> Hz { Hz::from_num(48000000) }
    fn fdpll96m() -> Hz { Hz::from_num(96000000) }
    fn gclkgen0() -> Hz { Hz::from_num(0) }
    fn gclkgen1() -> Hz { Hz::from_num(0) }
    fn gclkgen2() -> Hz { Hz::from_num(0) }
    fn gclkgen3() -> Hz { Hz::from_num(0) }
    fn gclkgen4() -> Hz { Hz::from_num(0) }
    fn gclkgen5() -> Hz { Hz::from_num(0) }
    fn gclkgen6() -> Hz { Hz::from_num(0) }
    fn gclkgen7() -> Hz { Hz::from_num(0) }
    fn gclkgen8() -> Hz { Hz::from_num(0) }
    fn gclk_dffl48m_ref() -> Hz { Hz::from_num(0) }
    fn gclk_dpll() -> Hz { Hz::from_num(0) }
    fn gclk_dpll_32k() -> Hz { Hz::from_num(0) }
    fn gclk_wdt() -> Hz { Hz::from_num(0) }
    fn gclk_rtc() -> Hz { Hz::from_num(0) }
    fn gclk_eic() -> Hz { Hz::from_num(0) }
    fn gclk_usb() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_0() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_1() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_2() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_3() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_4() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_5() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_6() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_7() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_8() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_9() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_10() -> Hz { Hz::from_num(0) }
    fn gclk_evsys_channel_11() -> Hz { Hz::from_num(0) }
    fn gclk_sercomx_slow() -> Hz { Hz::from_num(0) }
    fn gclk_sercom0_core() -> Hz { Hz::from_num(0) }
    fn gclk_sercom1_core() -> Hz { Hz::from_num(0) }
    fn gclk_sercom2_core() -> Hz { Hz::from_num(0) }
    fn gclk_sercom3_core() -> Hz { Hz::from_num(0) }
    fn gclk_sercom4_core() -> Hz { Hz::from_num(0) }
    fn gclk_sercom5_core() -> Hz { Hz::from_num(0) }
    fn gclk_tcc0_tcc1() -> Hz { Hz::from_num(0) }
    fn gclk_tcc2_tc3() -> Hz { Hz::from_num(0) }
    fn gclk_tc4_tc5() -> Hz { Hz::from_num(0) }
    fn gclk_tc6_tc7() -> Hz { Hz::from_num(0) }
    fn gclk_adc() -> Hz { Hz::from_num(0) }
    fn gclk_adc_dig() -> Hz { Hz::from_num(0) }
    fn gclk_20() -> Hz { Hz::from_num(0) }
    fn gclk_ac_ana() -> Hz { Hz::from_num(0) }
    fn gclk_22() -> Hz { Hz::from_num(0) }
    fn gclk_dac() -> Hz { Hz::from_num(0) }
    fn gclk_ptc() -> Hz { Hz::from_num(0) }
    fn gclk_i2s_0() -> Hz { Hz::from_num(0) }
    fn gclk_i2s_1() -> Hz { Hz::from_num(0) }
}

impl<T> ClockFor<::gclk::Gclk> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::gclk::Gclk) -> Hz { T::apba() }
}

impl<T> ClockFor<::nvmctrl::Nvmctrl> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::nvmctrl::Nvmctrl) -> Hz { T::ahb() }
}

impl<T> ClockFor<::pm::Pm> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::pm::Pm) -> Hz { T::apba() }
}

impl<T> ClockFor<::sysctrl::Sysctrl> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::sysctrl::Sysctrl) -> Hz { T::apba() }
}

impl<T> ClockFor<::wdt::Wdt> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::wdt::Wdt) -> Hz { T::apba() }
}

impl<T> ClockFor<::rtc::Rtc> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::rtc::Rtc) -> Hz { T::apba() }
}

impl<T> ClockFor<::dmac::Dmac> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dmac::Dmac) -> Hz { T::ahb() }
}

impl<T> ClockFor<::adc::Adc> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::adc::Adc) -> Hz { T::apbc() }
}

impl<T> ClockFor<::dac::Dac> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::dac::Dac) -> Hz { T::apbc() }
}

impl<T> ClockFor<::tcc::Tcc0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tcc::Tcc0) -> Hz { T::apbc() }
}

impl<T> ClockFor<::tcc::Tcc1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tcc::Tcc1) -> Hz { T::apbc() }
}

impl<T> ClockFor<::tcc::Tcc2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tcc::Tcc2) -> Hz { T::apbc() }
}

impl<T> ClockFor<::tc::Tc3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tc::Tc3) -> Hz { T::apbc() }
}

impl<T> ClockFor<::tc::Tc4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tc::Tc4) -> Hz { T::apbc() }
}

impl<T> ClockFor<::tc::Tc5> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::tc::Tc5) -> Hz { T::apbc() }
}

impl<T> ClockFor<::port::Porta> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::port::Porta) -> Hz { T::apbb() }
}

impl<T> ClockFor<::port::Portb> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::port::Portb) -> Hz { T::apbb() }
}

impl<T> ClockFor<::sercom::Sercom0> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::sercom::Sercom0) -> Hz { T::apbc() }
}

impl<T> ClockFor<::sercom::Sercom1> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::sercom::Sercom1) -> Hz { T::apbc() }
}

impl<T> ClockFor<::sercom::Sercom2> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::sercom::Sercom2) -> Hz { T::apbc() }
}

impl<T> ClockFor<::sercom::Sercom3> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::sercom::Sercom3) -> Hz { T::apbc() }
}

impl<T> ClockFor<::sercom::Sercom4> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::sercom::Sercom4) -> Hz { T::apbc() }
}

impl<T> ClockFor<::sercom::Sercom5> for ClockTree<T> where T: Clocks {
    fn clock_for(_: ::sercom::Sercom5) -> Hz { T::apbc() }
}

