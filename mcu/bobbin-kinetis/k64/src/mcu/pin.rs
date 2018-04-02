#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::bobbin_common::pin::*;
pub use ::bobbin_common::gate::*;

pub use super::port::*;

pin!(PTA0, Pta0, pta0, PORTA, Porta, PTA0_PIN, PortPin, PORTA_PERIPH, PTA0_OWNED, PTA0_REF_COUNT, 0);
   pin_source!(Pta0, super::gpio::Pa0, super::sig::SigGpio, 1);
   pin_source!(Pta0, super::ftm::Ftm0Ch5, super::sig::SigFtm, 3);
pin!(PTA1, Pta1, pta1, PORTA, Porta, PTA1_PIN, PortPin, PORTA_PERIPH, PTA1_OWNED, PTA1_REF_COUNT, 1);
   pin_source!(Pta1, super::gpio::Pa1, super::sig::SigGpio, 1);
   pin_source!(Pta1, super::uart::Uart0, super::sig::SigUartRx, 2);
   pin_source!(Pta1, super::ftm::Ftm0Ch6, super::sig::SigFtm, 3);
pin!(PTA2, Pta2, pta2, PORTA, Porta, PTA2_PIN, PortPin, PORTA_PERIPH, PTA2_OWNED, PTA2_REF_COUNT, 2);
   pin_source!(Pta2, super::gpio::Pa2, super::sig::SigGpio, 1);
   pin_source!(Pta2, super::uart::Uart0, super::sig::SigUartTx, 2);
   pin_source!(Pta2, super::ftm::Ftm0Ch7, super::sig::SigFtm, 3);
pin!(PTA3, Pta3, pta3, PORTA, Porta, PTA3_PIN, PortPin, PORTA_PERIPH, PTA3_OWNED, PTA3_REF_COUNT, 3);
   pin_source!(Pta3, super::gpio::Pa3, super::sig::SigGpio, 1);
   pin_source!(Pta3, super::ftm::Ftm0Ch0, super::sig::SigFtm, 3);
pin!(PTA4, Pta4, pta4, PORTA, Porta, PTA4_PIN, PortPin, PORTA_PERIPH, PTA4_OWNED, PTA4_REF_COUNT, 4);
   pin_source!(Pta4, super::gpio::Pa4, super::sig::SigGpio, 1);
   pin_source!(Pta4, super::ftm::Ftm0Ch1, super::sig::SigFtm, 3);
pin!(PTA5, Pta5, pta5, PORTA, Porta, PTA5_PIN, PortPin, PORTA_PERIPH, PTA5_OWNED, PTA5_REF_COUNT, 5);
   pin_source!(Pta5, super::gpio::Pa5, super::sig::SigGpio, 1);
   pin_source!(Pta5, super::ftm::Ftm0Ch2, super::sig::SigFtm, 3);
pin!(PTA6, Pta6, pta6, PORTA, Porta, PTA6_PIN, PortPin, PORTA_PERIPH, PTA6_OWNED, PTA6_REF_COUNT, 6);
   pin_source!(Pta6, super::gpio::Pa6, super::sig::SigGpio, 1);
   pin_source!(Pta6, super::ftm::Ftm0Ch3, super::sig::SigFtm, 3);
pin!(PTA7, Pta7, pta7, PORTA, Porta, PTA7_PIN, PortPin, PORTA_PERIPH, PTA7_OWNED, PTA7_REF_COUNT, 7);
   pin_source!(Pta7, super::adc::Adc0Ch10, super::sig::SigAdc, 0);
   pin_source!(Pta7, super::gpio::Pa7, super::sig::SigGpio, 1);
   pin_source!(Pta7, super::ftm::Ftm0Ch4, super::sig::SigFtm, 3);
pin!(PTA8, Pta8, pta8, PORTA, Porta, PTA8_PIN, PortPin, PORTA_PERIPH, PTA8_OWNED, PTA8_REF_COUNT, 8);
   pin_source!(Pta8, super::adc::Adc0Ch11, super::sig::SigAdc, 0);
   pin_source!(Pta8, super::gpio::Pa8, super::sig::SigGpio, 1);
   pin_source!(Pta8, super::ftm::Ftm1Ch0, super::sig::SigFtm, 3);
pin!(PTA9, Pta9, pta9, PORTA, Porta, PTA9_PIN, PortPin, PORTA_PERIPH, PTA9_OWNED, PTA9_REF_COUNT, 9);
   pin_source!(Pta9, super::gpio::Pa9, super::sig::SigGpio, 1);
   pin_source!(Pta9, super::ftm::Ftm1Ch1, super::sig::SigFtm, 3);
pin!(PTA10, Pta10, pta10, PORTA, Porta, PTA10_PIN, PortPin, PORTA_PERIPH, PTA10_OWNED, PTA10_REF_COUNT, 10);
   pin_source!(Pta10, super::gpio::Pa10, super::sig::SigGpio, 1);
   pin_source!(Pta10, super::ftm::Ftm2Ch0, super::sig::SigFtm, 3);
pin!(PTA11, Pta11, pta11, PORTA, Porta, PTA11_PIN, PortPin, PORTA_PERIPH, PTA11_OWNED, PTA11_REF_COUNT, 11);
   pin_source!(Pta11, super::gpio::Pa11, super::sig::SigGpio, 1);
   pin_source!(Pta11, super::ftm::Ftm2Ch1, super::sig::SigFtm, 3);
pin!(PTA12, Pta12, pta12, PORTA, Porta, PTA12_PIN, PortPin, PORTA_PERIPH, PTA12_OWNED, PTA12_REF_COUNT, 12);
   pin_source!(Pta12, super::gpio::Pa12, super::sig::SigGpio, 1);
   pin_source!(Pta12, super::flexcan::Can0, super::sig::SigCanTx, 2);
   pin_source!(Pta12, super::ftm::Ftm1Ch0, super::sig::SigFtm, 3);
pin!(PTA13, Pta13, pta13, PORTA, Porta, PTA13_PIN, PortPin, PORTA_PERIPH, PTA13_OWNED, PTA13_REF_COUNT, 13);
   pin_source!(Pta13, super::gpio::Pa13, super::sig::SigGpio, 1);
   pin_source!(Pta13, super::flexcan::Can0, super::sig::SigCanRx, 2);
   pin_source!(Pta13, super::ftm::Ftm1Ch1, super::sig::SigFtm, 3);
pin!(PTA14, Pta14, pta14, PORTA, Porta, PTA14_PIN, PortPin, PORTA_PERIPH, PTA14_OWNED, PTA14_REF_COUNT, 14);
   pin_source!(Pta14, super::gpio::Pa14, super::sig::SigGpio, 1);
   pin_source!(Pta14, super::spi::Spi0, super::sig::SigSpiPcs0, 2);
   pin_source!(Pta14, super::uart::Uart0, super::sig::SigUartTx, 3);
pin!(PTA15, Pta15, pta15, PORTA, Porta, PTA15_PIN, PortPin, PORTA_PERIPH, PTA15_OWNED, PTA15_REF_COUNT, 15);
   pin_source!(Pta15, super::gpio::Pa15, super::sig::SigGpio, 1);
   pin_source!(Pta15, super::spi::Spi0, super::sig::SigSpiSck, 2);
   pin_source!(Pta15, super::uart::Uart0, super::sig::SigUartRx, 3);
pin!(PTA16, Pta16, pta16, PORTA, Porta, PTA16_PIN, PortPin, PORTA_PERIPH, PTA16_OWNED, PTA16_REF_COUNT, 16);
   pin_source!(Pta16, super::gpio::Pa16, super::sig::SigGpio, 1);
   pin_source!(Pta16, super::spi::Spi0, super::sig::SigSpiSout, 2);
pin!(PTA17, Pta17, pta17, PORTA, Porta, PTA17_PIN, PortPin, PORTA_PERIPH, PTA17_OWNED, PTA17_REF_COUNT, 17);
   pin_source!(Pta17, super::adc::Adc1Ch17, super::sig::SigAdc, 0);
   pin_source!(Pta17, super::gpio::Pa17, super::sig::SigGpio, 1);
   pin_source!(Pta17, super::spi::Spi0, super::sig::SigSpiSin, 2);
pin!(PTA18, Pta18, pta18, PORTA, Porta, PTA18_PIN, PortPin, PORTA_PERIPH, PTA18_OWNED, PTA18_REF_COUNT, 18);
   pin_source!(Pta18, super::gpio::Pa18, super::sig::SigGpio, 1);
pin!(PTA19, Pta19, pta19, PORTA, Porta, PTA19_PIN, PortPin, PORTA_PERIPH, PTA19_OWNED, PTA19_REF_COUNT, 19);
   pin_source!(Pta19, super::gpio::Pa19, super::sig::SigGpio, 1);
pin!(PTA24, Pta24, pta24, PORTA, Porta, PTA24_PIN, PortPin, PORTA_PERIPH, PTA24_OWNED, PTA24_REF_COUNT, 24);
   pin_source!(Pta24, super::gpio::Pa24, super::sig::SigGpio, 1);
pin!(PTA25, Pta25, pta25, PORTA, Porta, PTA25_PIN, PortPin, PORTA_PERIPH, PTA25_OWNED, PTA25_REF_COUNT, 25);
   pin_source!(Pta25, super::gpio::Pa25, super::sig::SigGpio, 1);
pin!(PTA26, Pta26, pta26, PORTA, Porta, PTA26_PIN, PortPin, PORTA_PERIPH, PTA26_OWNED, PTA26_REF_COUNT, 26);
   pin_source!(Pta26, super::gpio::Pa26, super::sig::SigGpio, 1);
pin!(PTA27, Pta27, pta27, PORTA, Porta, PTA27_PIN, PortPin, PORTA_PERIPH, PTA27_OWNED, PTA27_REF_COUNT, 27);
   pin_source!(Pta27, super::gpio::Pa27, super::sig::SigGpio, 1);
pin!(PTA28, Pta28, pta28, PORTA, Porta, PTA28_PIN, PortPin, PORTA_PERIPH, PTA28_OWNED, PTA28_REF_COUNT, 28);
   pin_source!(Pta28, super::gpio::Pa28, super::sig::SigGpio, 1);
pin!(PTA29, Pta29, pta29, PORTA, Porta, PTA29_PIN, PortPin, PORTA_PERIPH, PTA29_OWNED, PTA29_REF_COUNT, 29);
   pin_source!(Pta29, super::gpio::Pa29, super::sig::SigGpio, 1);
pin!(PTB0, Ptb0, ptb0, PORTB, Portb, PTB0_PIN, PortPin, PORTB_PERIPH, PTB0_OWNED, PTB0_REF_COUNT, 0);
   pin_source!(Ptb0, super::adc::Adc0Ch8, super::sig::SigAdc, 0);
   pin_source!(Ptb0, super::adc::Adc1Ch8, super::sig::SigAdc, 0);
   pin_source!(Ptb0, super::gpio::Pb0, super::sig::SigGpio, 1);
   pin_source!(Ptb0, super::i2c::I2c0, super::sig::SigI2cScl, 2);
   pin_source!(Ptb0, super::ftm::Ftm1Ch0, super::sig::SigFtm, 3);
pin!(PTB1, Ptb1, ptb1, PORTB, Portb, PTB1_PIN, PortPin, PORTB_PERIPH, PTB1_OWNED, PTB1_REF_COUNT, 1);
   pin_source!(Ptb1, super::adc::Adc0Ch9, super::sig::SigAdc, 0);
   pin_source!(Ptb1, super::adc::Adc1Ch9, super::sig::SigAdc, 0);
   pin_source!(Ptb1, super::gpio::Pb1, super::sig::SigGpio, 1);
   pin_source!(Ptb1, super::i2c::I2c0, super::sig::SigI2cSda, 2);
   pin_source!(Ptb1, super::ftm::Ftm1Ch1, super::sig::SigFtm, 3);
pin!(PTB2, Ptb2, ptb2, PORTB, Portb, PTB2_PIN, PortPin, PORTB_PERIPH, PTB2_OWNED, PTB2_REF_COUNT, 2);
   pin_source!(Ptb2, super::adc::Adc0Ch12, super::sig::SigAdc, 0);
   pin_source!(Ptb2, super::gpio::Pb2, super::sig::SigGpio, 1);
   pin_source!(Ptb2, super::i2c::I2c0, super::sig::SigI2cScl, 2);
pin!(PTB3, Ptb3, ptb3, PORTB, Portb, PTB3_PIN, PortPin, PORTB_PERIPH, PTB3_OWNED, PTB3_REF_COUNT, 3);
   pin_source!(Ptb3, super::adc::Adc0Ch13, super::sig::SigAdc, 0);
   pin_source!(Ptb3, super::gpio::Pb3, super::sig::SigGpio, 1);
   pin_source!(Ptb3, super::i2c::I2c0, super::sig::SigI2cSda, 2);
pin!(PTB4, Ptb4, ptb4, PORTB, Portb, PTB4_PIN, PortPin, PORTB_PERIPH, PTB4_OWNED, PTB4_REF_COUNT, 4);
   pin_source!(Ptb4, super::adc::Adc1Ch10, super::sig::SigAdc, 0);
   pin_source!(Ptb4, super::gpio::Pb4, super::sig::SigGpio, 1);
pin!(PTB5, Ptb5, ptb5, PORTB, Portb, PTB5_PIN, PortPin, PORTB_PERIPH, PTB5_OWNED, PTB5_REF_COUNT, 5);
   pin_source!(Ptb5, super::adc::Adc1Ch11, super::sig::SigAdc, 0);
   pin_source!(Ptb5, super::gpio::Pb5, super::sig::SigGpio, 1);
pin!(PTB6, Ptb6, ptb6, PORTB, Portb, PTB6_PIN, PortPin, PORTB_PERIPH, PTB6_OWNED, PTB6_REF_COUNT, 6);
   pin_source!(Ptb6, super::adc::Adc1Ch12, super::sig::SigAdc, 0);
   pin_source!(Ptb6, super::gpio::Pb6, super::sig::SigGpio, 1);
pin!(PTB7, Ptb7, ptb7, PORTB, Portb, PTB7_PIN, PortPin, PORTB_PERIPH, PTB7_OWNED, PTB7_REF_COUNT, 7);
   pin_source!(Ptb7, super::adc::Adc1Ch13, super::sig::SigAdc, 0);
   pin_source!(Ptb7, super::gpio::Pb7, super::sig::SigGpio, 1);
pin!(PTB8, Ptb8, ptb8, PORTB, Portb, PTB8_PIN, PortPin, PORTB_PERIPH, PTB8_OWNED, PTB8_REF_COUNT, 8);
   pin_source!(Ptb8, super::gpio::Pb8, super::sig::SigGpio, 1);
pin!(PTB9, Ptb9, ptb9, PORTB, Portb, PTB9_PIN, PortPin, PORTB_PERIPH, PTB9_OWNED, PTB9_REF_COUNT, 9);
   pin_source!(Ptb9, super::gpio::Pb9, super::sig::SigGpio, 1);
   pin_source!(Ptb9, super::spi::Spi1, super::sig::SigSpiPcs1, 2);
pin!(PTB10, Ptb10, ptb10, PORTB, Portb, PTB10_PIN, PortPin, PORTB_PERIPH, PTB10_OWNED, PTB10_REF_COUNT, 10);
   pin_source!(Ptb10, super::adc::Adc1Ch14, super::sig::SigAdc, 0);
   pin_source!(Ptb10, super::gpio::Pb10, super::sig::SigGpio, 1);
   pin_source!(Ptb10, super::spi::Spi1, super::sig::SigSpiPcs0, 2);
   pin_source!(Ptb10, super::uart::Uart3, super::sig::SigUartRx, 3);
pin!(PTB11, Ptb11, ptb11, PORTB, Portb, PTB11_PIN, PortPin, PORTB_PERIPH, PTB11_OWNED, PTB11_REF_COUNT, 11);
   pin_source!(Ptb11, super::adc::Adc1Ch15, super::sig::SigAdc, 0);
   pin_source!(Ptb11, super::gpio::Pb11, super::sig::SigGpio, 1);
   pin_source!(Ptb11, super::spi::Spi1, super::sig::SigSpiSck, 2);
   pin_source!(Ptb11, super::uart::Uart3, super::sig::SigUartTx, 3);
pin!(PTB12, Ptb12, ptb12, PORTB, Portb, PTB12_PIN, PortPin, PORTB_PERIPH, PTB12_OWNED, PTB12_REF_COUNT, 12);
   pin_source!(Ptb12, super::gpio::Pb12, super::sig::SigGpio, 1);
   pin_source!(Ptb12, super::ftm::Ftm1Ch0, super::sig::SigFtm, 3);
   pin_source!(Ptb12, super::ftm::Ftm0Ch4, super::sig::SigFtm, 4);
pin!(PTB13, Ptb13, ptb13, PORTB, Portb, PTB13_PIN, PortPin, PORTB_PERIPH, PTB13_OWNED, PTB13_REF_COUNT, 13);
   pin_source!(Ptb13, super::gpio::Pb13, super::sig::SigGpio, 1);
   pin_source!(Ptb13, super::ftm::Ftm1Ch1, super::sig::SigFtm, 3);
   pin_source!(Ptb13, super::ftm::Ftm0Ch5, super::sig::SigFtm, 4);
pin!(PTB16, Ptb16, ptb16, PORTB, Portb, PTB16_PIN, PortPin, PORTB_PERIPH, PTB16_OWNED, PTB16_REF_COUNT, 16);
   pin_source!(Ptb16, super::gpio::Pb16, super::sig::SigGpio, 1);
   pin_source!(Ptb16, super::spi::Spi1, super::sig::SigSpiSout, 2);
   pin_source!(Ptb16, super::uart::Uart0, super::sig::SigUartRx, 3);
pin!(PTB17, Ptb17, ptb17, PORTB, Portb, PTB17_PIN, PortPin, PORTB_PERIPH, PTB17_OWNED, PTB17_REF_COUNT, 17);
   pin_source!(Ptb17, super::gpio::Pb17, super::sig::SigGpio, 1);
   pin_source!(Ptb17, super::spi::Spi1, super::sig::SigSpiSin, 2);
   pin_source!(Ptb17, super::uart::Uart0, super::sig::SigUartTx, 3);
pin!(PTB18, Ptb18, ptb18, PORTB, Portb, PTB18_PIN, PortPin, PORTB_PERIPH, PTB18_OWNED, PTB18_REF_COUNT, 18);
   pin_source!(Ptb18, super::gpio::Pb18, super::sig::SigGpio, 1);
   pin_source!(Ptb18, super::flexcan::Can0, super::sig::SigCanTx, 2);
   pin_source!(Ptb18, super::ftm::Ftm2Ch0, super::sig::SigFtm, 3);
pin!(PTB19, Ptb19, ptb19, PORTB, Portb, PTB19_PIN, PortPin, PORTB_PERIPH, PTB19_OWNED, PTB19_REF_COUNT, 19);
   pin_source!(Ptb19, super::gpio::Pb19, super::sig::SigGpio, 1);
   pin_source!(Ptb19, super::flexcan::Can0, super::sig::SigCanRx, 2);
   pin_source!(Ptb19, super::ftm::Ftm2Ch1, super::sig::SigFtm, 3);
pin!(PTB20, Ptb20, ptb20, PORTB, Portb, PTB20_PIN, PortPin, PORTB_PERIPH, PTB20_OWNED, PTB20_REF_COUNT, 20);
   pin_source!(Ptb20, super::gpio::Pb20, super::sig::SigGpio, 1);
   pin_source!(Ptb20, super::spi::Spi2, super::sig::SigSpiPcs0, 2);
pin!(PTB21, Ptb21, ptb21, PORTB, Portb, PTB21_PIN, PortPin, PORTB_PERIPH, PTB21_OWNED, PTB21_REF_COUNT, 21);
   pin_source!(Ptb21, super::gpio::Pb21, super::sig::SigGpio, 1);
   pin_source!(Ptb21, super::spi::Spi2, super::sig::SigSpiSck, 2);
pin!(PTB22, Ptb22, ptb22, PORTB, Portb, PTB22_PIN, PortPin, PORTB_PERIPH, PTB22_OWNED, PTB22_REF_COUNT, 22);
   pin_source!(Ptb22, super::gpio::Pb22, super::sig::SigGpio, 1);
   pin_source!(Ptb22, super::spi::Spi2, super::sig::SigSpiSout, 2);
pin!(PTB23, Ptb23, ptb23, PORTB, Portb, PTB23_PIN, PortPin, PORTB_PERIPH, PTB23_OWNED, PTB23_REF_COUNT, 23);
   pin_source!(Ptb23, super::gpio::Pb23, super::sig::SigGpio, 1);
   pin_source!(Ptb23, super::spi::Spi2, super::sig::SigSpiSin, 2);
   pin_source!(Ptb23, super::spi::Spi0, super::sig::SigSpiPcs5, 3);
pin!(PTC0, Ptc0, ptc0, PORTC, Portc, PTC0_PIN, PortPin, PORTC_PERIPH, PTC0_OWNED, PTC0_REF_COUNT, 0);
   pin_source!(Ptc0, super::adc::Adc0Ch14, super::sig::SigAdc, 0);
   pin_source!(Ptc0, super::gpio::Pc0, super::sig::SigGpio, 1);
   pin_source!(Ptc0, super::spi::Spi0, super::sig::SigSpiPcs4, 2);
pin!(PTC1, Ptc1, ptc1, PORTC, Portc, PTC1_PIN, PortPin, PORTC_PERIPH, PTC1_OWNED, PTC1_REF_COUNT, 1);
   pin_source!(Ptc1, super::adc::Adc0Ch15, super::sig::SigAdc, 0);
   pin_source!(Ptc1, super::gpio::Pc1, super::sig::SigGpio, 1);
   pin_source!(Ptc1, super::spi::Spi0, super::sig::SigSpiPcs3, 2);
   pin_source!(Ptc1, super::ftm::Ftm0Ch0, super::sig::SigFtm, 4);
pin!(PTC2, Ptc2, ptc2, PORTC, Portc, PTC2_PIN, PortPin, PORTC_PERIPH, PTC2_OWNED, PTC2_REF_COUNT, 2);
   pin_source!(Ptc2, super::adc::Adc0Ch4, super::sig::SigAdcSeb, 0);
   pin_source!(Ptc2, super::gpio::Pc2, super::sig::SigGpio, 1);
   pin_source!(Ptc2, super::spi::Spi0, super::sig::SigSpiPcs2, 2);
   pin_source!(Ptc2, super::ftm::Ftm0Ch1, super::sig::SigFtm, 4);
pin!(PTC3, Ptc3, ptc3, PORTC, Portc, PTC3_PIN, PortPin, PORTC_PERIPH, PTC3_OWNED, PTC3_REF_COUNT, 3);
   pin_source!(Ptc3, super::gpio::Pc3, super::sig::SigGpio, 1);
   pin_source!(Ptc3, super::spi::Spi0, super::sig::SigSpiPcs1, 2);
   pin_source!(Ptc3, super::uart::Uart1, super::sig::SigUartRx, 3);
   pin_source!(Ptc3, super::ftm::Ftm0Ch2, super::sig::SigFtm, 4);
pin!(PTC4, Ptc4, ptc4, PORTC, Portc, PTC4_PIN, PortPin, PORTC_PERIPH, PTC4_OWNED, PTC4_REF_COUNT, 4);
   pin_source!(Ptc4, super::gpio::Pc4, super::sig::SigGpio, 1);
   pin_source!(Ptc4, super::spi::Spi0, super::sig::SigSpiPcs0, 2);
   pin_source!(Ptc4, super::uart::Uart1, super::sig::SigUartTx, 3);
   pin_source!(Ptc4, super::ftm::Ftm0Ch3, super::sig::SigFtm, 4);
pin!(PTC5, Ptc5, ptc5, PORTC, Portc, PTC5_PIN, PortPin, PORTC_PERIPH, PTC5_OWNED, PTC5_REF_COUNT, 5);
   pin_source!(Ptc5, super::gpio::Pc5, super::sig::SigGpio, 1);
   pin_source!(Ptc5, super::spi::Spi0, super::sig::SigSpiSck, 2);
   pin_source!(Ptc5, super::ftm::Ftm0Ch2, super::sig::SigFtm, 7);
pin!(PTC6, Ptc6, ptc6, PORTC, Portc, PTC6_PIN, PortPin, PORTC_PERIPH, PTC6_OWNED, PTC6_REF_COUNT, 6);
   pin_source!(Ptc6, super::gpio::Pc6, super::sig::SigGpio, 1);
   pin_source!(Ptc6, super::spi::Spi0, super::sig::SigSpiSout, 2);
pin!(PTC7, Ptc7, ptc7, PORTC, Portc, PTC7_PIN, PortPin, PORTC_PERIPH, PTC7_OWNED, PTC7_REF_COUNT, 7);
   pin_source!(Ptc7, super::gpio::Pc7, super::sig::SigGpio, 1);
   pin_source!(Ptc7, super::spi::Spi0, super::sig::SigSpiSin, 2);
pin!(PTC8, Ptc8, ptc8, PORTC, Portc, PTC8_PIN, PortPin, PORTC_PERIPH, PTC8_OWNED, PTC8_REF_COUNT, 8);
   pin_source!(Ptc8, super::adc::Adc1Ch4, super::sig::SigAdcSeb, 0);
   pin_source!(Ptc8, super::gpio::Pc8, super::sig::SigGpio, 1);
pin!(PTC9, Ptc9, ptc9, PORTC, Portc, PTC9_PIN, PortPin, PORTC_PERIPH, PTC9_OWNED, PTC9_REF_COUNT, 9);
   pin_source!(Ptc9, super::adc::Adc1Ch5, super::sig::SigAdcSeb, 0);
   pin_source!(Ptc9, super::gpio::Pc9, super::sig::SigGpio, 1);
pin!(PTC10, Ptc10, ptc10, PORTC, Portc, PTC10_PIN, PortPin, PORTC_PERIPH, PTC10_OWNED, PTC10_REF_COUNT, 10);
   pin_source!(Ptc10, super::adc::Adc1Ch6, super::sig::SigAdcSeb, 0);
   pin_source!(Ptc10, super::gpio::Pc10, super::sig::SigGpio, 1);
   pin_source!(Ptc10, super::i2c::I2c1, super::sig::SigI2cScl, 2);
pin!(PTC11, Ptc11, ptc11, PORTC, Portc, PTC11_PIN, PortPin, PORTC_PERIPH, PTC11_OWNED, PTC11_REF_COUNT, 11);
   pin_source!(Ptc11, super::adc::Adc1Ch7, super::sig::SigAdcSeb, 0);
   pin_source!(Ptc11, super::gpio::Pc11, super::sig::SigGpio, 1);
   pin_source!(Ptc11, super::i2c::I2c1, super::sig::SigI2cSda, 2);
pin!(PTC12, Ptc12, ptc12, PORTC, Portc, PTC12_PIN, PortPin, PORTC_PERIPH, PTC12_OWNED, PTC12_REF_COUNT, 12);
   pin_source!(Ptc12, super::gpio::Pc12, super::sig::SigGpio, 1);
pin!(PTC13, Ptc13, ptc13, PORTC, Portc, PTC13_PIN, PortPin, PORTC_PERIPH, PTC13_OWNED, PTC13_REF_COUNT, 13);
   pin_source!(Ptc13, super::gpio::Pc13, super::sig::SigGpio, 1);
pin!(PTC14, Ptc14, ptc14, PORTC, Portc, PTC14_PIN, PortPin, PORTC_PERIPH, PTC14_OWNED, PTC14_REF_COUNT, 14);
   pin_source!(Ptc14, super::gpio::Pc14, super::sig::SigGpio, 1);
   pin_source!(Ptc14, super::uart::Uart4, super::sig::SigUartRx, 3);
pin!(PTC15, Ptc15, ptc15, PORTC, Portc, PTC15_PIN, PortPin, PORTC_PERIPH, PTC15_OWNED, PTC15_REF_COUNT, 15);
   pin_source!(Ptc15, super::gpio::Pc15, super::sig::SigGpio, 1);
   pin_source!(Ptc15, super::uart::Uart4, super::sig::SigUartTx, 3);
pin!(PTC16, Ptc16, ptc16, PORTC, Portc, PTC16_PIN, PortPin, PORTC_PERIPH, PTC16_OWNED, PTC16_REF_COUNT, 16);
   pin_source!(Ptc16, super::gpio::Pc16, super::sig::SigGpio, 1);
   pin_source!(Ptc16, super::uart::Uart3, super::sig::SigUartRx, 3);
pin!(PTC17, Ptc17, ptc17, PORTC, Portc, PTC17_PIN, PortPin, PORTC_PERIPH, PTC17_OWNED, PTC17_REF_COUNT, 17);
   pin_source!(Ptc17, super::gpio::Pc17, super::sig::SigGpio, 1);
   pin_source!(Ptc17, super::uart::Uart3, super::sig::SigUartTx, 3);
pin!(PTC18, Ptc18, ptc18, PORTC, Portc, PTC18_PIN, PortPin, PORTC_PERIPH, PTC18_OWNED, PTC18_REF_COUNT, 18);
   pin_source!(Ptc18, super::gpio::Pc18, super::sig::SigGpio, 1);
pin!(PTC19, Ptc19, ptc19, PORTC, Portc, PTC19_PIN, PortPin, PORTC_PERIPH, PTC19_OWNED, PTC19_REF_COUNT, 19);
   pin_source!(Ptc19, super::gpio::Pc19, super::sig::SigGpio, 1);
pin!(PTD0, Ptd0, ptd0, PORTD, Portd, PTD0_PIN, PortPin, PORTD_PERIPH, PTD0_OWNED, PTD0_REF_COUNT, 0);
   pin_source!(Ptd0, super::gpio::Pd0, super::sig::SigGpio, 1);
   pin_source!(Ptd0, super::spi::Spi0, super::sig::SigSpiPcs0, 2);
pin!(PTD1, Ptd1, ptd1, PORTD, Portd, PTD1_PIN, PortPin, PORTD_PERIPH, PTD1_OWNED, PTD1_REF_COUNT, 1);
   pin_source!(Ptd1, super::adc::Adc0Ch5, super::sig::SigAdcSeb, 0);
   pin_source!(Ptd1, super::gpio::Pd1, super::sig::SigGpio, 1);
   pin_source!(Ptd1, super::spi::Spi0, super::sig::SigSpiSck, 2);
pin!(PTD2, Ptd2, ptd2, PORTD, Portd, PTD2_PIN, PortPin, PORTD_PERIPH, PTD2_OWNED, PTD2_REF_COUNT, 2);
   pin_source!(Ptd2, super::gpio::Pd2, super::sig::SigGpio, 1);
   pin_source!(Ptd2, super::spi::Spi0, super::sig::SigSpiSout, 2);
   pin_source!(Ptd2, super::uart::Uart2, super::sig::SigUartRx, 3);
   pin_source!(Ptd2, super::i2c::I2c0, super::sig::SigI2cScl, 7);
pin!(PTD3, Ptd3, ptd3, PORTD, Portd, PTD3_PIN, PortPin, PORTD_PERIPH, PTD3_OWNED, PTD3_REF_COUNT, 3);
   pin_source!(Ptd3, super::gpio::Pd3, super::sig::SigGpio, 1);
   pin_source!(Ptd3, super::spi::Spi0, super::sig::SigSpiSin, 2);
   pin_source!(Ptd3, super::uart::Uart2, super::sig::SigUartTx, 3);
   pin_source!(Ptd3, super::i2c::I2c0, super::sig::SigI2cSda, 7);
pin!(PTD4, Ptd4, ptd4, PORTD, Portd, PTD4_PIN, PortPin, PORTD_PERIPH, PTD4_OWNED, PTD4_REF_COUNT, 4);
   pin_source!(Ptd4, super::gpio::Pd4, super::sig::SigGpio, 1);
   pin_source!(Ptd4, super::spi::Spi0, super::sig::SigSpiPcs1, 2);
   pin_source!(Ptd4, super::ftm::Ftm0Ch4, super::sig::SigFtm, 4);
   pin_source!(Ptd4, super::spi::Spi1, super::sig::SigSpiPcs0, 7);
pin!(PTD5, Ptd5, ptd5, PORTD, Portd, PTD5_PIN, PortPin, PORTD_PERIPH, PTD5_OWNED, PTD5_REF_COUNT, 5);
   pin_source!(Ptd5, super::adc::Adc0Ch6, super::sig::SigAdcSeb, 0);
   pin_source!(Ptd5, super::gpio::Pd5, super::sig::SigGpio, 1);
   pin_source!(Ptd5, super::spi::Spi0, super::sig::SigSpiPcs2, 2);
   pin_source!(Ptd5, super::ftm::Ftm0Ch5, super::sig::SigFtm, 4);
   pin_source!(Ptd5, super::spi::Spi1, super::sig::SigSpiSck, 7);
pin!(PTD6, Ptd6, ptd6, PORTD, Portd, PTD6_PIN, PortPin, PORTD_PERIPH, PTD6_OWNED, PTD6_REF_COUNT, 6);
   pin_source!(Ptd6, super::adc::Adc0Ch7, super::sig::SigAdcSeb, 0);
   pin_source!(Ptd6, super::gpio::Pd6, super::sig::SigGpio, 1);
   pin_source!(Ptd6, super::spi::Spi0, super::sig::SigSpiPcs3, 2);
   pin_source!(Ptd6, super::uart::Uart0, super::sig::SigUartRx, 3);
   pin_source!(Ptd6, super::ftm::Ftm0Ch6, super::sig::SigFtm, 4);
   pin_source!(Ptd6, super::spi::Spi1, super::sig::SigSpiSout, 7);
pin!(PTD7, Ptd7, ptd7, PORTD, Portd, PTD7_PIN, PortPin, PORTD_PERIPH, PTD7_OWNED, PTD7_REF_COUNT, 7);
   pin_source!(Ptd7, super::gpio::Pd7, super::sig::SigGpio, 1);
   pin_source!(Ptd7, super::uart::Uart0, super::sig::SigUartTx, 3);
   pin_source!(Ptd7, super::ftm::Ftm0Ch7, super::sig::SigFtm, 4);
   pin_source!(Ptd7, super::spi::Spi1, super::sig::SigSpiSin, 7);
pin!(PTD8, Ptd8, ptd8, PORTD, Portd, PTD8_PIN, PortPin, PORTD_PERIPH, PTD8_OWNED, PTD8_REF_COUNT, 8);
   pin_source!(Ptd8, super::gpio::Pd8, super::sig::SigGpio, 1);
   pin_source!(Ptd8, super::i2c::I2c0, super::sig::SigI2cScl, 2);
   pin_source!(Ptd8, super::uart::Uart5, super::sig::SigUartRx, 3);
pin!(PTD9, Ptd9, ptd9, PORTD, Portd, PTD9_PIN, PortPin, PORTD_PERIPH, PTD9_OWNED, PTD9_REF_COUNT, 9);
   pin_source!(Ptd9, super::gpio::Pd9, super::sig::SigGpio, 1);
   pin_source!(Ptd9, super::i2c::I2c0, super::sig::SigI2cSda, 2);
   pin_source!(Ptd9, super::uart::Uart5, super::sig::SigUartTx, 3);
pin!(PTD10, Ptd10, ptd10, PORTD, Portd, PTD10_PIN, PortPin, PORTD_PERIPH, PTD10_OWNED, PTD10_REF_COUNT, 10);
   pin_source!(Ptd10, super::gpio::Pd10, super::sig::SigGpio, 1);
pin!(PTD11, Ptd11, ptd11, PORTD, Portd, PTD11_PIN, PortPin, PORTD_PERIPH, PTD11_OWNED, PTD11_REF_COUNT, 11);
   pin_source!(Ptd11, super::gpio::Pd11, super::sig::SigGpio, 1);
   pin_source!(Ptd11, super::spi::Spi2, super::sig::SigSpiPcs0, 2);
pin!(PTD12, Ptd12, ptd12, PORTD, Portd, PTD12_PIN, PortPin, PORTD_PERIPH, PTD12_OWNED, PTD12_REF_COUNT, 12);
   pin_source!(Ptd12, super::gpio::Pd12, super::sig::SigGpio, 1);
   pin_source!(Ptd12, super::spi::Spi2, super::sig::SigSpiSck, 2);
pin!(PTD13, Ptd13, ptd13, PORTD, Portd, PTD13_PIN, PortPin, PORTD_PERIPH, PTD13_OWNED, PTD13_REF_COUNT, 13);
   pin_source!(Ptd13, super::gpio::Pd13, super::sig::SigGpio, 1);
   pin_source!(Ptd13, super::spi::Spi2, super::sig::SigSpiSout, 2);
pin!(PTD14, Ptd14, ptd14, PORTD, Portd, PTD14_PIN, PortPin, PORTD_PERIPH, PTD14_OWNED, PTD14_REF_COUNT, 14);
   pin_source!(Ptd14, super::gpio::Pd14, super::sig::SigGpio, 1);
   pin_source!(Ptd14, super::spi::Spi2, super::sig::SigSpiSin, 2);
pin!(PTD15, Ptd15, ptd15, PORTD, Portd, PTD15_PIN, PortPin, PORTD_PERIPH, PTD15_OWNED, PTD15_REF_COUNT, 15);
   pin_source!(Ptd15, super::gpio::Pd15, super::sig::SigGpio, 1);
   pin_source!(Ptd15, super::spi::Spi2, super::sig::SigSpiPcs1, 2);
pin!(PTE0, Pte0, pte0, PORTE, Porte, PTE0_PIN, PortPin, PORTE_PERIPH, PTE0_OWNED, PTE0_REF_COUNT, 0);
   pin_source!(Pte0, super::adc::Adc1Ch4, super::sig::SigAdcSea, 0);
   pin_source!(Pte0, super::gpio::Pe0, super::sig::SigGpio, 1);
   pin_source!(Pte0, super::spi::Spi1, super::sig::SigSpiPcs1, 2);
   pin_source!(Pte0, super::uart::Uart1, super::sig::SigUartTx, 3);
   pin_source!(Pte0, super::i2c::I2c1, super::sig::SigI2cSda, 6);
pin!(PTE1, Pte1, pte1, PORTE, Porte, PTE1_PIN, PortPin, PORTE_PERIPH, PTE1_OWNED, PTE1_REF_COUNT, 1);
   pin_source!(Pte1, super::adc::Adc1Ch5, super::sig::SigAdcSea, 0);
   pin_source!(Pte1, super::gpio::Pe1, super::sig::SigGpio, 1);
   pin_source!(Pte1, super::spi::Spi1, super::sig::SigSpiSout, 2);
   pin_source!(Pte1, super::uart::Uart1, super::sig::SigUartRx, 3);
   pin_source!(Pte1, super::i2c::I2c1, super::sig::SigI2cScl, 6);
   pin_source!(Pte1, super::spi::Spi1, super::sig::SigSpiSin, 7);
pin!(PTE2, Pte2, pte2, PORTE, Porte, PTE2_PIN, PortPin, PORTE_PERIPH, PTE2_OWNED, PTE2_REF_COUNT, 2);
   pin_source!(Pte2, super::adc::Adc0Ch2, super::sig::SigAdcDp, 0);
   pin_source!(Pte2, super::adc::Adc1Ch6, super::sig::SigAdcSea, 0);
   pin_source!(Pte2, super::gpio::Pe2, super::sig::SigGpio, 1);
   pin_source!(Pte2, super::spi::Spi1, super::sig::SigSpiSck, 2);
pin!(PTE3, Pte3, pte3, PORTE, Porte, PTE3_PIN, PortPin, PORTE_PERIPH, PTE3_OWNED, PTE3_REF_COUNT, 3);
   pin_source!(Pte3, super::adc::Adc0Ch2, super::sig::SigAdcDm, 0);
   pin_source!(Pte3, super::adc::Adc1Ch7, super::sig::SigAdcSea, 0);
   pin_source!(Pte3, super::gpio::Pe3, super::sig::SigGpio, 1);
   pin_source!(Pte3, super::spi::Spi1, super::sig::SigSpiSin, 2);
   pin_source!(Pte3, super::spi::Spi1, super::sig::SigSpiSout, 7);
pin!(PTE4, Pte4, pte4, PORTE, Porte, PTE4_PIN, PortPin, PORTE_PERIPH, PTE4_OWNED, PTE4_REF_COUNT, 4);
   pin_source!(Pte4, super::gpio::Pe4, super::sig::SigGpio, 1);
   pin_source!(Pte4, super::spi::Spi1, super::sig::SigSpiPcs0, 2);
   pin_source!(Pte4, super::uart::Uart3, super::sig::SigUartTx, 3);
pin!(PTE5, Pte5, pte5, PORTE, Porte, PTE5_PIN, PortPin, PORTE_PERIPH, PTE5_OWNED, PTE5_REF_COUNT, 5);
   pin_source!(Pte5, super::gpio::Pe5, super::sig::SigGpio, 1);
   pin_source!(Pte5, super::spi::Spi1, super::sig::SigSpiPcs2, 2);
   pin_source!(Pte5, super::uart::Uart3, super::sig::SigUartRx, 3);
pin!(PTE6, Pte6, pte6, PORTE, Porte, PTE6_PIN, PortPin, PORTE_PERIPH, PTE6_OWNED, PTE6_REF_COUNT, 6);
   pin_source!(Pte6, super::gpio::Pe6, super::sig::SigGpio, 1);
   pin_source!(Pte6, super::spi::Spi1, super::sig::SigSpiPcs3, 2);
pin!(PTE7, Pte7, pte7, PORTE, Porte, PTE7_PIN, PortPin, PORTE_PERIPH, PTE7_OWNED, PTE7_REF_COUNT, 7);
   pin_source!(Pte7, super::gpio::Pe7, super::sig::SigGpio, 1);
pin!(PTE8, Pte8, pte8, PORTE, Porte, PTE8_PIN, PortPin, PORTE_PERIPH, PTE8_OWNED, PTE8_REF_COUNT, 8);
   pin_source!(Pte8, super::gpio::Pe8, super::sig::SigGpio, 1);
   pin_source!(Pte8, super::uart::Uart5, super::sig::SigUartTx, 3);
pin!(PTE9, Pte9, pte9, PORTE, Porte, PTE9_PIN, PortPin, PORTE_PERIPH, PTE9_OWNED, PTE9_REF_COUNT, 9);
   pin_source!(Pte9, super::gpio::Pe9, super::sig::SigGpio, 1);
   pin_source!(Pte9, super::uart::Uart5, super::sig::SigUartRx, 3);
pin!(PTE10, Pte10, pte10, PORTE, Porte, PTE10_PIN, PortPin, PORTE_PERIPH, PTE10_OWNED, PTE10_REF_COUNT, 10);
   pin_source!(Pte10, super::gpio::Pe10, super::sig::SigGpio, 1);
pin!(PTE11, Pte11, pte11, PORTE, Porte, PTE11_PIN, PortPin, PORTE_PERIPH, PTE11_OWNED, PTE11_REF_COUNT, 11);
   pin_source!(Pte11, super::gpio::Pe11, super::sig::SigGpio, 1);
pin!(PTE12, Pte12, pte12, PORTE, Porte, PTE12_PIN, PortPin, PORTE_PERIPH, PTE12_OWNED, PTE12_REF_COUNT, 12);
   pin_source!(Pte12, super::gpio::Pe12, super::sig::SigGpio, 1);
pin!(PTE24, Pte24, pte24, PORTE, Porte, PTE24_PIN, PortPin, PORTE_PERIPH, PTE24_OWNED, PTE24_REF_COUNT, 24);
   pin_source!(Pte24, super::adc::Adc0Ch17, super::sig::SigAdc, 0);
   pin_source!(Pte24, super::gpio::Pe24, super::sig::SigGpio, 1);
   pin_source!(Pte24, super::uart::Uart4, super::sig::SigUartTx, 3);
   pin_source!(Pte24, super::i2c::I2c0, super::sig::SigI2cScl, 5);
pin!(PTE25, Pte25, pte25, PORTE, Porte, PTE25_PIN, PortPin, PORTE_PERIPH, PTE25_OWNED, PTE25_REF_COUNT, 25);
   pin_source!(Pte25, super::adc::Adc0Ch18, super::sig::SigAdc, 0);
   pin_source!(Pte25, super::gpio::Pe25, super::sig::SigGpio, 1);
   pin_source!(Pte25, super::uart::Uart4, super::sig::SigUartRx, 3);
   pin_source!(Pte25, super::i2c::I2c0, super::sig::SigI2cSda, 5);
pin!(PTE26, Pte26, pte26, PORTE, Porte, PTE26_PIN, PortPin, PORTE_PERIPH, PTE26_OWNED, PTE26_REF_COUNT, 26);
   pin_source!(Pte26, super::gpio::Pe26, super::sig::SigGpio, 1);
pin!(PTE27, Pte27, pte27, PORTE, Porte, PTE27_PIN, PortPin, PORTE_PERIPH, PTE27_OWNED, PTE27_REF_COUNT, 27);
   pin_source!(Pte27, super::gpio::Pe27, super::sig::SigGpio, 1);
pin!(PTE28, Pte28, pte28, PORTE, Porte, PTE28_PIN, PortPin, PORTE_PERIPH, PTE28_OWNED, PTE28_REF_COUNT, 28);
   pin_source!(Pte28, super::gpio::Pe28, super::sig::SigGpio, 1);
