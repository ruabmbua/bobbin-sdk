#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::bobbin_common::pin::*;
pub use ::bobbin_common::gate::*;

pub use super::port::*;

pin!(PA00, Pa00, pa00, PORTA, Porta, PA00_PIN, PortPin, PORTA_PERIPH, PA00_OWNED, PA00_REF_COUNT, 0);
   pin_source!(Pa00, super::sercom::Sercom1, super::sig::SigPad0, 3);
   pin_source!(Pa00, super::tcc::Tcc2, super::sig::SigWo0, 4);
pin!(PA01, Pa01, pa01, PORTA, Porta, PA01_PIN, PortPin, PORTA_PERIPH, PA01_OWNED, PA01_REF_COUNT, 1);
   pin_source!(Pa01, super::sercom::Sercom1, super::sig::SigPad1, 3);
   pin_source!(Pa01, super::tcc::Tcc2, super::sig::SigWo1, 4);
pin!(PA02, Pa02, pa02, PORTA, Porta, PA02_PIN, PortPin, PORTA_PERIPH, PA02_OWNED, PA02_REF_COUNT, 2);
   pin_source!(Pa02, super::adc::AdcCh0, super::sig::SigAin, 1);
pin!(PA03, Pa03, pa03, PORTA, Porta, PA03_PIN, PortPin, PORTA_PERIPH, PA03_OWNED, PA03_REF_COUNT, 3);
   pin_source!(Pa03, super::adc::AdcCh1, super::sig::SigAin, 1);
pin!(PA04, Pa04, pa04, PORTA, Porta, PA04_PIN, PortPin, PORTA_PERIPH, PA04_OWNED, PA04_REF_COUNT, 4);
   pin_source!(Pa04, super::adc::AdcCh4, super::sig::SigAin, 1);
   pin_source!(Pa04, super::adc::AdcCh0, super::sig::SigAin, 1);
   pin_source!(Pa04, super::sercom::Sercom0, super::sig::SigPad0, 3);
   pin_source!(Pa04, super::tcc::Tcc0, super::sig::SigWo0, 4);
pin!(PA05, Pa05, pa05, PORTA, Porta, PA05_PIN, PortPin, PORTA_PERIPH, PA05_OWNED, PA05_REF_COUNT, 5);
   pin_source!(Pa05, super::adc::AdcCh5, super::sig::SigAin, 1);
   pin_source!(Pa05, super::adc::AdcCh1, super::sig::SigAin, 1);
   pin_source!(Pa05, super::sercom::Sercom0, super::sig::SigPad1, 3);
   pin_source!(Pa05, super::tcc::Tcc0, super::sig::SigWo1, 4);
pin!(PA06, Pa06, pa06, PORTA, Porta, PA06_PIN, PortPin, PORTA_PERIPH, PA06_OWNED, PA06_REF_COUNT, 6);
   pin_source!(Pa06, super::adc::AdcCh6, super::sig::SigAin, 1);
   pin_source!(Pa06, super::adc::AdcCh2, super::sig::SigAin, 1);
   pin_source!(Pa06, super::sercom::Sercom0, super::sig::SigPad2, 3);
   pin_source!(Pa06, super::tcc::Tcc1, super::sig::SigWo0, 4);
pin!(PA07, Pa07, pa07, PORTA, Porta, PA07_PIN, PortPin, PORTA_PERIPH, PA07_OWNED, PA07_REF_COUNT, 7);
   pin_source!(Pa07, super::adc::AdcCh7, super::sig::SigAin, 1);
   pin_source!(Pa07, super::adc::AdcCh3, super::sig::SigAin, 1);
   pin_source!(Pa07, super::sercom::Sercom0, super::sig::SigPad3, 3);
   pin_source!(Pa07, super::tcc::Tcc1, super::sig::SigWo1, 4);
pin!(PA08, Pa08, pa08, PORTA, Porta, PA08_PIN, PortPin, PORTA_PERIPH, PA08_OWNED, PA08_REF_COUNT, 8);
   pin_source!(Pa08, super::adc::AdcCh16, super::sig::SigAin, 1);
   pin_source!(Pa08, super::sercom::Sercom0, super::sig::SigPad0, 2);
   pin_source!(Pa08, super::sercom::Sercom2, super::sig::SigPad0, 3);
   pin_source!(Pa08, super::tcc::Tcc0, super::sig::SigWo0, 4);
   pin_source!(Pa08, super::tcc::Tcc1, super::sig::SigWo2, 5);
pin!(PA09, Pa09, pa09, PORTA, Porta, PA09_PIN, PortPin, PORTA_PERIPH, PA09_OWNED, PA09_REF_COUNT, 9);
   pin_source!(Pa09, super::adc::AdcCh17, super::sig::SigAin, 1);
   pin_source!(Pa09, super::sercom::Sercom0, super::sig::SigPad1, 2);
   pin_source!(Pa09, super::sercom::Sercom2, super::sig::SigPad1, 3);
   pin_source!(Pa09, super::tcc::Tcc0, super::sig::SigWo1, 4);
   pin_source!(Pa09, super::tcc::Tcc1, super::sig::SigWo3, 5);
pin!(PA10, Pa10, pa10, PORTA, Porta, PA10_PIN, PortPin, PORTA_PERIPH, PA10_OWNED, PA10_REF_COUNT, 10);
   pin_source!(Pa10, super::adc::AdcCh18, super::sig::SigAin, 1);
   pin_source!(Pa10, super::sercom::Sercom0, super::sig::SigPad2, 2);
   pin_source!(Pa10, super::sercom::Sercom2, super::sig::SigPad2, 3);
   pin_source!(Pa10, super::tcc::Tcc1, super::sig::SigWo0, 4);
   pin_source!(Pa10, super::tcc::Tcc0, super::sig::SigWo2, 5);
pin!(PA11, Pa11, pa11, PORTA, Porta, PA11_PIN, PortPin, PORTA_PERIPH, PA11_OWNED, PA11_REF_COUNT, 11);
   pin_source!(Pa11, super::adc::AdcCh19, super::sig::SigAin, 1);
   pin_source!(Pa11, super::sercom::Sercom0, super::sig::SigPad3, 2);
   pin_source!(Pa11, super::sercom::Sercom2, super::sig::SigPad3, 3);
   pin_source!(Pa11, super::tcc::Tcc1, super::sig::SigWo1, 4);
   pin_source!(Pa11, super::tcc::Tcc0, super::sig::SigWo3, 5);
pin!(PA12, Pa12, pa12, PORTA, Porta, PA12_PIN, PortPin, PORTA_PERIPH, PA12_OWNED, PA12_REF_COUNT, 12);
   pin_source!(Pa12, super::sercom::Sercom2, super::sig::SigPad0, 2);
   pin_source!(Pa12, super::sercom::Sercom4, super::sig::SigPad0, 3);
   pin_source!(Pa12, super::tcc::Tcc2, super::sig::SigWo0, 4);
   pin_source!(Pa12, super::tcc::Tcc0, super::sig::SigWo6, 5);
pin!(PA13, Pa13, pa13, PORTA, Porta, PA13_PIN, PortPin, PORTA_PERIPH, PA13_OWNED, PA13_REF_COUNT, 13);
   pin_source!(Pa13, super::sercom::Sercom2, super::sig::SigPad1, 2);
   pin_source!(Pa13, super::sercom::Sercom4, super::sig::SigPad1, 3);
   pin_source!(Pa13, super::tcc::Tcc2, super::sig::SigWo1, 4);
   pin_source!(Pa13, super::tcc::Tcc0, super::sig::SigWo7, 5);
pin!(PA14, Pa14, pa14, PORTA, Porta, PA14_PIN, PortPin, PORTA_PERIPH, PA14_OWNED, PA14_REF_COUNT, 14);
   pin_source!(Pa14, super::sercom::Sercom2, super::sig::SigPad2, 2);
   pin_source!(Pa14, super::sercom::Sercom4, super::sig::SigPad2, 3);
   pin_source!(Pa14, super::tc::Tc3Ch0, super::sig::SigWo, 4);
   pin_source!(Pa14, super::tcc::Tcc0, super::sig::SigWo4, 5);
pin!(PA15, Pa15, pa15, PORTA, Porta, PA15_PIN, PortPin, PORTA_PERIPH, PA15_OWNED, PA15_REF_COUNT, 15);
   pin_source!(Pa15, super::sercom::Sercom2, super::sig::SigPad3, 2);
   pin_source!(Pa15, super::sercom::Sercom4, super::sig::SigPad3, 3);
   pin_source!(Pa15, super::tc::Tc3Ch1, super::sig::SigWo, 4);
   pin_source!(Pa15, super::tcc::Tcc0, super::sig::SigWo5, 5);
pin!(PA16, Pa16, pa16, PORTA, Porta, PA16_PIN, PortPin, PORTA_PERIPH, PA16_OWNED, PA16_REF_COUNT, 16);
   pin_source!(Pa16, super::sercom::Sercom1, super::sig::SigPad0, 2);
   pin_source!(Pa16, super::sercom::Sercom3, super::sig::SigPad0, 3);
   pin_source!(Pa16, super::tcc::Tcc2, super::sig::SigWo0, 4);
   pin_source!(Pa16, super::tcc::Tcc0, super::sig::SigWo6, 5);
pin!(PA17, Pa17, pa17, PORTA, Porta, PA17_PIN, PortPin, PORTA_PERIPH, PA17_OWNED, PA17_REF_COUNT, 17);
   pin_source!(Pa17, super::sercom::Sercom1, super::sig::SigPad1, 2);
   pin_source!(Pa17, super::sercom::Sercom3, super::sig::SigPad1, 3);
   pin_source!(Pa17, super::tcc::Tcc2, super::sig::SigWo1, 4);
   pin_source!(Pa17, super::tcc::Tcc0, super::sig::SigWo7, 5);
pin!(PA18, Pa18, pa18, PORTA, Porta, PA18_PIN, PortPin, PORTA_PERIPH, PA18_OWNED, PA18_REF_COUNT, 18);
   pin_source!(Pa18, super::sercom::Sercom1, super::sig::SigPad2, 2);
   pin_source!(Pa18, super::sercom::Sercom3, super::sig::SigPad2, 3);
   pin_source!(Pa18, super::tc::Tc3Ch0, super::sig::SigWo, 4);
   pin_source!(Pa18, super::tcc::Tcc0, super::sig::SigWo2, 5);
pin!(PA19, Pa19, pa19, PORTA, Porta, PA19_PIN, PortPin, PORTA_PERIPH, PA19_OWNED, PA19_REF_COUNT, 19);
   pin_source!(Pa19, super::sercom::Sercom1, super::sig::SigPad3, 2);
   pin_source!(Pa19, super::sercom::Sercom3, super::sig::SigPad3, 3);
   pin_source!(Pa19, super::tc::Tc3Ch1, super::sig::SigWo, 4);
   pin_source!(Pa19, super::tcc::Tcc0, super::sig::SigWo3, 5);
pin!(PA20, Pa20, pa20, PORTA, Porta, PA20_PIN, PortPin, PORTA_PERIPH, PA20_OWNED, PA20_REF_COUNT, 20);
   pin_source!(Pa20, super::sercom::Sercom5, super::sig::SigPad2, 2);
   pin_source!(Pa20, super::sercom::Sercom3, super::sig::SigPad2, 3);
   pin_source!(Pa20, super::tcc::Tcc0, super::sig::SigWo6, 5);
pin!(PA21, Pa21, pa21, PORTA, Porta, PA21_PIN, PortPin, PORTA_PERIPH, PA21_OWNED, PA21_REF_COUNT, 21);
   pin_source!(Pa21, super::sercom::Sercom5, super::sig::SigPad3, 2);
   pin_source!(Pa21, super::sercom::Sercom3, super::sig::SigPad3, 3);
   pin_source!(Pa21, super::tcc::Tcc0, super::sig::SigWo7, 5);
pin!(PA22, Pa22, pa22, PORTA, Porta, PA22_PIN, PortPin, PORTA_PERIPH, PA22_OWNED, PA22_REF_COUNT, 22);
   pin_source!(Pa22, super::sercom::Sercom3, super::sig::SigPad0, 2);
   pin_source!(Pa22, super::sercom::Sercom5, super::sig::SigPad0, 3);
   pin_source!(Pa22, super::tc::Tc4Ch0, super::sig::SigWo, 4);
   pin_source!(Pa22, super::tcc::Tcc0, super::sig::SigWo4, 5);
pin!(PA23, Pa23, pa23, PORTA, Porta, PA23_PIN, PortPin, PORTA_PERIPH, PA23_OWNED, PA23_REF_COUNT, 23);
   pin_source!(Pa23, super::sercom::Sercom3, super::sig::SigPad1, 2);
   pin_source!(Pa23, super::sercom::Sercom5, super::sig::SigPad1, 3);
   pin_source!(Pa23, super::tc::Tc4Ch1, super::sig::SigWo, 4);
   pin_source!(Pa23, super::tcc::Tcc0, super::sig::SigWo5, 5);
pin!(PA24, Pa24, pa24, PORTA, Porta, PA24_PIN, PortPin, PORTA_PERIPH, PA24_OWNED, PA24_REF_COUNT, 24);
   pin_source!(Pa24, super::sercom::Sercom3, super::sig::SigPad2, 2);
   pin_source!(Pa24, super::sercom::Sercom5, super::sig::SigPad2, 3);
   pin_source!(Pa24, super::tc::Tc5Ch0, super::sig::SigWo, 4);
   pin_source!(Pa24, super::tcc::Tcc1, super::sig::SigWo2, 5);
pin!(PA25, Pa25, pa25, PORTA, Porta, PA25_PIN, PortPin, PORTA_PERIPH, PA25_OWNED, PA25_REF_COUNT, 25);
   pin_source!(Pa25, super::sercom::Sercom3, super::sig::SigPad3, 2);
   pin_source!(Pa25, super::sercom::Sercom5, super::sig::SigPad3, 3);
   pin_source!(Pa25, super::tc::Tc5Ch1, super::sig::SigWo, 4);
   pin_source!(Pa25, super::tcc::Tcc1, super::sig::SigWo3, 5);
pin!(PA27, Pa27, pa27, PORTA, Porta, PA27_PIN, PortPin, PORTA_PERIPH, PA27_OWNED, PA27_REF_COUNT, 27);
pin!(PA28, Pa28, pa28, PORTA, Porta, PA28_PIN, PortPin, PORTA_PERIPH, PA28_OWNED, PA28_REF_COUNT, 28);
pin!(PA30, Pa30, pa30, PORTA, Porta, PA30_PIN, PortPin, PORTA_PERIPH, PA30_OWNED, PA30_REF_COUNT, 30);
   pin_source!(Pa30, super::sercom::Sercom1, super::sig::SigPad2, 3);
   pin_source!(Pa30, super::tcc::Tcc1, super::sig::SigWo0, 4);
pin!(PA31, Pa31, pa31, PORTA, Porta, PA31_PIN, PortPin, PORTA_PERIPH, PA31_OWNED, PA31_REF_COUNT, 31);
   pin_source!(Pa31, super::sercom::Sercom1, super::sig::SigPad3, 3);
   pin_source!(Pa31, super::tcc::Tcc1, super::sig::SigWo1, 4);
pin!(PB00, Pb00, pb00, PORTB, Portb, PB00_PIN, PortPin, PORTB_PERIPH, PB00_OWNED, PB00_REF_COUNT, 0);
   pin_source!(Pb00, super::adc::AdcCh8, super::sig::SigAin, 1);
   pin_source!(Pb00, super::sercom::Sercom5, super::sig::SigPad2, 3);
pin!(PB01, Pb01, pb01, PORTB, Portb, PB01_PIN, PortPin, PORTB_PERIPH, PB01_OWNED, PB01_REF_COUNT, 1);
   pin_source!(Pb01, super::adc::AdcCh9, super::sig::SigAin, 1);
   pin_source!(Pb01, super::sercom::Sercom5, super::sig::SigPad3, 3);
pin!(PB02, Pb02, pb02, PORTB, Portb, PB02_PIN, PortPin, PORTB_PERIPH, PB02_OWNED, PB02_REF_COUNT, 2);
   pin_source!(Pb02, super::adc::AdcCh10, super::sig::SigAin, 1);
   pin_source!(Pb02, super::sercom::Sercom5, super::sig::SigPad0, 3);
pin!(PB03, Pb03, pb03, PORTB, Portb, PB03_PIN, PortPin, PORTB_PERIPH, PB03_OWNED, PB03_REF_COUNT, 3);
   pin_source!(Pb03, super::adc::AdcCh11, super::sig::SigAin, 1);
   pin_source!(Pb03, super::sercom::Sercom5, super::sig::SigPad1, 3);
pin!(PB04, Pb04, pb04, PORTB, Portb, PB04_PIN, PortPin, PORTB_PERIPH, PB04_OWNED, PB04_REF_COUNT, 4);
   pin_source!(Pb04, super::adc::AdcCh12, super::sig::SigAin, 1);
pin!(PB05, Pb05, pb05, PORTB, Portb, PB05_PIN, PortPin, PORTB_PERIPH, PB05_OWNED, PB05_REF_COUNT, 5);
   pin_source!(Pb05, super::adc::AdcCh13, super::sig::SigAin, 1);
pin!(PB06, Pb06, pb06, PORTB, Portb, PB06_PIN, PortPin, PORTB_PERIPH, PB06_OWNED, PB06_REF_COUNT, 6);
   pin_source!(Pb06, super::adc::AdcCh14, super::sig::SigAin, 1);
pin!(PB07, Pb07, pb07, PORTB, Portb, PB07_PIN, PortPin, PORTB_PERIPH, PB07_OWNED, PB07_REF_COUNT, 7);
   pin_source!(Pb07, super::adc::AdcCh15, super::sig::SigAin, 1);
pin!(PB08, Pb08, pb08, PORTB, Portb, PB08_PIN, PortPin, PORTB_PERIPH, PB08_OWNED, PB08_REF_COUNT, 8);
   pin_source!(Pb08, super::adc::AdcCh2, super::sig::SigAin, 1);
   pin_source!(Pb08, super::sercom::Sercom4, super::sig::SigPad0, 3);
   pin_source!(Pb08, super::tc::Tc4Ch0, super::sig::SigWo, 4);
pin!(PB09, Pb09, pb09, PORTB, Portb, PB09_PIN, PortPin, PORTB_PERIPH, PB09_OWNED, PB09_REF_COUNT, 9);
   pin_source!(Pb09, super::adc::AdcCh3, super::sig::SigAin, 1);
   pin_source!(Pb09, super::sercom::Sercom4, super::sig::SigPad1, 3);
   pin_source!(Pb09, super::tc::Tc4Ch1, super::sig::SigWo, 4);
pin!(PB10, Pb10, pb10, PORTB, Portb, PB10_PIN, PortPin, PORTB_PERIPH, PB10_OWNED, PB10_REF_COUNT, 10);
   pin_source!(Pb10, super::sercom::Sercom4, super::sig::SigPad2, 3);
   pin_source!(Pb10, super::tc::Tc5Ch0, super::sig::SigWo, 4);
   pin_source!(Pb10, super::tcc::Tcc0, super::sig::SigWo4, 5);
pin!(PB11, Pb11, pb11, PORTB, Portb, PB11_PIN, PortPin, PORTB_PERIPH, PB11_OWNED, PB11_REF_COUNT, 11);
   pin_source!(Pb11, super::sercom::Sercom4, super::sig::SigPad3, 3);
   pin_source!(Pb11, super::tc::Tc5Ch1, super::sig::SigWo, 4);
   pin_source!(Pb11, super::tcc::Tcc0, super::sig::SigWo5, 5);
pin!(PB12, Pb12, pb12, PORTB, Portb, PB12_PIN, PortPin, PORTB_PERIPH, PB12_OWNED, PB12_REF_COUNT, 12);
   pin_source!(Pb12, super::sercom::Sercom4, super::sig::SigPad0, 2);
   pin_source!(Pb12, super::tc::Tc4Ch0, super::sig::SigWo, 4);
   pin_source!(Pb12, super::tcc::Tcc0, super::sig::SigWo6, 5);
pin!(PB13, Pb13, pb13, PORTB, Portb, PB13_PIN, PortPin, PORTB_PERIPH, PB13_OWNED, PB13_REF_COUNT, 13);
   pin_source!(Pb13, super::sercom::Sercom4, super::sig::SigPad1, 2);
   pin_source!(Pb13, super::tc::Tc4Ch1, super::sig::SigWo, 4);
   pin_source!(Pb13, super::tcc::Tcc0, super::sig::SigWo7, 5);
pin!(PB14, Pb14, pb14, PORTB, Portb, PB14_PIN, PortPin, PORTB_PERIPH, PB14_OWNED, PB14_REF_COUNT, 14);
   pin_source!(Pb14, super::sercom::Sercom4, super::sig::SigPad2, 2);
   pin_source!(Pb14, super::tc::Tc5Ch0, super::sig::SigWo, 4);
pin!(PB15, Pb15, pb15, PORTB, Portb, PB15_PIN, PortPin, PORTB_PERIPH, PB15_OWNED, PB15_REF_COUNT, 15);
   pin_source!(Pb15, super::sercom::Sercom4, super::sig::SigPad3, 2);
   pin_source!(Pb15, super::tc::Tc5Ch1, super::sig::SigWo, 4);
pin!(PB16, Pb16, pb16, PORTB, Portb, PB16_PIN, PortPin, PORTB_PERIPH, PB16_OWNED, PB16_REF_COUNT, 16);
   pin_source!(Pb16, super::sercom::Sercom5, super::sig::SigPad0, 2);
   pin_source!(Pb16, super::tcc::Tcc0, super::sig::SigWo4, 5);
pin!(PB17, Pb17, pb17, PORTB, Portb, PB17_PIN, PortPin, PORTB_PERIPH, PB17_OWNED, PB17_REF_COUNT, 17);
   pin_source!(Pb17, super::sercom::Sercom5, super::sig::SigPad1, 2);
   pin_source!(Pb17, super::tcc::Tcc0, super::sig::SigWo5, 5);
pin!(PB22, Pb22, pb22, PORTB, Portb, PB22_PIN, PortPin, PORTB_PERIPH, PB22_OWNED, PB22_REF_COUNT, 22);
   pin_source!(Pb22, super::sercom::Sercom5, super::sig::SigPad2, 3);
pin!(PB23, Pb23, pb23, PORTB, Portb, PB23_PIN, PortPin, PORTB_PERIPH, PB23_OWNED, PB23_REF_COUNT, 23);
   pin_source!(Pb23, super::sercom::Sercom5, super::sig::SigPad3, 3);
pin!(PB30, Pb30, pb30, PORTB, Portb, PB30_PIN, PortPin, PORTB_PERIPH, PB30_OWNED, PB30_REF_COUNT, 30);
   pin_source!(Pb30, super::sercom::Sercom5, super::sig::SigPad0, 3);
   pin_source!(Pb30, super::tcc::Tcc0, super::sig::SigWo0, 4);
   pin_source!(Pb30, super::tcc::Tcc1, super::sig::SigWo2, 5);
pin!(PB31, Pb31, pb31, PORTB, Portb, PB31_PIN, PortPin, PORTB_PERIPH, PB31_OWNED, PB31_REF_COUNT, 31);
   pin_source!(Pb31, super::sercom::Sercom5, super::sig::SigPad1, 3);
   pin_source!(Pb31, super::tcc::Tcc0, super::sig::SigWo1, 4);
   pin_source!(Pb31, super::tcc::Tcc1, super::sig::SigWo3, 5);
