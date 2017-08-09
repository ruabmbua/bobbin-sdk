use bobbin_common::bits::*;
use ::chip::rcc::{self, RCC, En};
use ::chip::flash;
use ::chip::pwr;
use ::chip::usart_f24::*;
use ::chip::tim_bas::*;
use ::chip::tim_gen::*;
use ::chip::tim_adv::*;

use core::fmt;


pub fn enable_pll_external_mode() {
    let rcc = rcc::RCC;
    let flash = flash::FLASH;
    let pwr = pwr::PWR;

    // Enable internal high-speed oscillator.
    rcc.with_cr(|r| r.set_hsion(1));

    // Wait for HSI Ready
    while rcc.cr().hsirdy() == 0 {}

    // Select HSI as SYSCLK source. 
    rcc.with_cfgr(|r| r.set_sw(0b00));
    while RCC.cfgr().sws() != 0b00 {}

    // Enable external high-speed oscillator 8MHz.
    // rcc.with_cr(|r| r.set_hseon(1));

    // Enable external source
    rcc.with_cr(|r| r.set_hseon(1).set_hsebyp(1));
    
    // Wait for HSE Ready
    while rcc.cr().hserdy() == 0 {}

    pwr.with_csr(|r| r.set_vosrdy(1));

    // Set prescalers for AHB, ADC, ABP1, ABP2

    // HPRE = HPRE_DIV_NONE
    // PPRE1 = PPRE_DIV_4
    // PPRE2 = PPRE_DIV_2
    rcc.with_cfgr(|r| r.set_hpre(0).set_ppre1(0b101).set_ppre2(0b100));

    // Configure PLL
    // PLLSRC = HSE
    // M = 8
    // N = 336
    // P = 2
    // Q = 7
    // R = 0
    
    rcc.with_pllcfgr(|r|
        r.set_pllsrc(1)
            .set_pllm(8)
            .set_plln(336)
            .set_pllp(0)
            .set_pllq(7)
    );

    // rcc.with_pllcfgr(|r|
    //     r.set_pllsrc(1)
    //         .set_pllq3(0).set_pllq2(1).set_pllq1(1).set_pllq0(1)
    //         .set_pllp1(0).set_pllp0(0)
    //         .set_plln8(1).set_plln7(0).set_plln6(1).set_plln5(0).set_plln4(1).set_plln3(0).set_plln2(0).set_plln1(0).set_plln0(0)
    //         .set_pllm5(0).set_pllm4(0).set_pllm3(1).set_pllm2(0).set_pllm1(0).set_pllm0(0)                                
    // );

    // Enable PLL oscillator and wait for it to stabilize.
    rcc.with_cr(|r| r.set_pllon(1));
    
    // Wait for PLL Ready
    while rcc.cr().pllrdy() == 0 {}

    // Configure flash settings.

    flash.with_acr(|r| r.set_icen(1).set_dcen(1).set_latency(5));
    
    // Select PLL as SYSCLK source.

    rcc.with_cfgr(|r| r.set_sw(0b10));
    while rcc.cfgr().sws() != 0b10 {}
    
    // Disable internal high-speed oscillator.        
    rcc.with_cr(|r| r.set_hsion(0));
}

pub const LSI: Hz = Some(32_000);
pub const HSI: Hz = Some(16_000_000);

pub type Hz = Option<u32>;

pub trait ClockTree {
    fn lsi(&self) -> Hz { LSI }
    fn hsi(&self) -> Hz { HSI }
    fn lse(&self) -> Hz;
    fn hse(&self) -> Hz;
    fn pllclk(&self) -> Hz;
    fn sysclk(&self) -> Hz;
    fn hclk(&self) -> Hz;
    fn systick(&self) -> Hz;
    fn pclk1(&self) -> Hz;
    fn tim_pclk1(&self) -> Hz;
    fn pclk2(&self) -> Hz;
    fn tim_pclk2(&self) -> Hz;
}

pub trait Clock<T: ClockTree> {
    fn clock(&self, t: &T) -> Hz;
}

pub struct DynamicClock {
    pub hse_osc: Hz,
    pub lse_osc: Hz,
}

impl ClockTree for DynamicClock {
    fn hse(&self) -> Hz {
        self.hse_osc
    }

    fn lse(&self) -> Hz {
        self.lse_osc
    }

    fn pllclk(&self) -> Hz {
        let cfgr = RCC.pllcfgr();
        let vco = match cfgr.pllsrc() {
            U1::B0 => self.hsi(),
            U1::B1 => self.hse(),
        }.map(|v| v * cfgr.plln().into_u32() / cfgr.pllm().into_u32());
        vco.map(|v| v / (2 * (cfgr.pllp().into_u32() + 1)))
    }

    fn sysclk(&self) -> Hz {
        match RCC.cfgr().sws() {
            U2::B00 => self.hsi(),
            U2::B01 => self.hse(),
            U2::B10 => self.pllclk(),
            U2::B11 => panic!("Invalid value for RCC_CFGR[SWS]"),
        }
    }

    fn hclk(&self) -> Hz {
        let shift = match RCC.cfgr().hpre() {
            U4::B0000 => 0,
            U4::B0001 => 0,
            U4::B0010 => 0,
            U4::B0011 => 0,
            U4::B0100 => 0,
            U4::B0101 => 0,
            U4::B0110 => 0,
            U4::B0111 => 0,
            U4::B1000 => 1,
            U4::B1001 => 2,
            U4::B1010 => 3,
            U4::B1011 => 4,
            // NOTE: Divide by 32 is skipped
            U4::B1100 => 6,
            U4::B1101 => 7,
            U4::B1110 => 8,
            U4::B1111 => 9,
        };
        self.sysclk().map(|v| v >> shift)
    }    

    fn systick(&self) -> Hz {
        self.hclk().map(|v| v >> 3)
    }

    fn pclk1(&self) -> Hz {
        let shift = match RCC.cfgr().ppre1() {
            U3::B000 => 0,
            U3::B001 => 0,
            U3::B010 => 0,
            U3::B011 => 0,            
            U3::B100 => 1,
            U3::B101 => 2,
            U3::B110 => 3,
            U3::B111 => 4,
        };
        self.hclk().map(|v| v >> shift)
    }

    fn tim_pclk1(&self) -> Hz {
        match RCC.cfgr().ppre1() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => self.pclk1(),
            _ => self.pclk1().map(|v| v << 1),        
        }
    }

    fn pclk2(&self) -> Hz {
        let shift = match RCC.cfgr().ppre2() {
            U3::B000 => 0,
            U3::B001 => 0,
            U3::B010 => 0,
            U3::B011 => 0,            
            U3::B100 => 1,
            U3::B101 => 2,
            U3::B110 => 3,
            U3::B111 => 4,
        };
        self.hclk().map(|v| v >> shift)
    }

    fn tim_pclk2(&self) -> Hz {
        match RCC.cfgr().ppre2() {
            U3::B000 | U3::B001 | U3::B010 | U3::B011 => self.pclk2(),
            _ => self.pclk2().map(|v| v << 1),
        }
    }    
    
}
impl fmt::Debug for DynamicClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[DynamicCLock")?;
        write!(f, " LSI={:?}", self.lsi())?;
        write!(f, " HSI={:?}", self.hsi())?;
        write!(f, " LSE={:?}", self.lse())?;
        write!(f, " HSE={:?}", self.hse())?;
        write!(f, " PLLCLK={:?}", self.pllclk())?;
        write!(f, " SYSCLK={:?}", self.sysclk())?;
        write!(f, " SYSTICK={:?}", self.systick())?;
        write!(f, " HCLK={:?}", self.hclk())?;
        write!(f, " PCLK1={:?}", self.pclk1())?;
        write!(f, " TIM_PCLK1={:?}", self.tim_pclk1())?;
        write!(f, " PCLK2={:?}", self.pclk2())?;
        write!(f, " TIM_PCLK2={:?}", self.tim_pclk2())?;
        write!(f, " PCLK2={:?}", self.pclk2())?;
        write!(f, " TIM_PCLK2={:?}", self.tim_pclk2())?;
        write!(f, "]")?;
        Ok(())
    }
}

macro_rules! impl_clock {
    ($ty:ty, $id:ident) => (
        impl<T: ClockTree> Clock<T> for $ty {
            fn clock(&self, t: &T) -> Hz {
                if self.en() != 0 {
                    t.$id()
                } else {
                    None
                }
            }
        }
    )
}

impl_clock!(Usart1, pclk2);
impl_clock!(Usart2, pclk1);
impl_clock!(Usart3, pclk1);
impl_clock!(Uart4, pclk1);
impl_clock!(Uart5, pclk1);
impl_clock!(Usart6, pclk2);
impl_clock!(Uart7, pclk1);
impl_clock!(Uart8, pclk1);

impl_clock!(Tim1, tim_pclk2);
impl_clock!(Tim2, tim_pclk1);
impl_clock!(Tim3, tim_pclk1);
impl_clock!(Tim4, tim_pclk1);
impl_clock!(Tim5, tim_pclk1);
impl_clock!(Tim6, tim_pclk1);
impl_clock!(Tim7, tim_pclk1);
impl_clock!(Tim8, tim_pclk2);
impl_clock!(Tim9, tim_pclk2);
impl_clock!(Tim10, tim_pclk2);
impl_clock!(Tim11, tim_pclk2);
impl_clock!(Tim12, tim_pclk1);
impl_clock!(Tim13, tim_pclk1);
impl_clock!(Tim14, tim_pclk1);