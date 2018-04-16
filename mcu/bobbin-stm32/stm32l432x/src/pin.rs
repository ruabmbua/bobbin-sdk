pub use super::gpio::*;

::bobbin_mcu::pin!(PA0, Pa0, pa0, GPIOA, Gpioa, PA0_PIN, GpioPin, GPIOA_PERIPH, PA0_OWNED, PA0_REF_COUNT, 0);
   ::bobbin_mcu::pin_source!(Pa0, super::adc::Adc1Ch5, super::sig::SigAdc, 0);
   ::bobbin_mcu::pin_source!(Pa0, super::tim_gen::Tim2Ch1, super::sig::SigTim, 1);
   ::bobbin_mcu::pin_source!(Pa0, super::usart::Usart2, super::sig::SigCts, 7);
   ::bobbin_mcu::pin_source!(Pa0, super::tim_gen::Tim2, super::sig::SigEtr, 14);
::bobbin_mcu::pin!(PA1, Pa1, pa1, GPIOA, Gpioa, PA1_PIN, GpioPin, GPIOA_PERIPH, PA1_OWNED, PA1_REF_COUNT, 1);
   ::bobbin_mcu::pin_source!(Pa1, super::adc::Adc1Ch6, super::sig::SigAdc, 0);
   ::bobbin_mcu::pin_source!(Pa1, super::tim_gen::Tim2Ch2, super::sig::SigTim, 1);
   ::bobbin_mcu::pin_source!(Pa1, super::i2c::I2c1, super::sig::SigI2cSmba, 4);
   ::bobbin_mcu::pin_source!(Pa1, super::spi::Spi1, super::sig::SigSpiSck, 5);
::bobbin_mcu::pin!(PA2, Pa2, pa2, GPIOA, Gpioa, PA2_PIN, GpioPin, GPIOA_PERIPH, PA2_OWNED, PA2_REF_COUNT, 2);
   ::bobbin_mcu::pin_source!(Pa2, super::adc::Adc1Ch7, super::sig::SigAdc, 0);
   ::bobbin_mcu::pin_source!(Pa2, super::tim_gen::Tim2Ch3, super::sig::SigTim, 1);
   ::bobbin_mcu::pin_source!(Pa2, super::usart::Usart2, super::sig::SigTx, 7);
   ::bobbin_mcu::pin_source!(Pa2, super::lpuart::Lpuart1, super::sig::SigTx, 8);
   ::bobbin_mcu::pin_source!(Pa2, super::tim_gen::Tim15Ch1, super::sig::SigTim, 14);
::bobbin_mcu::pin!(PA3, Pa3, pa3, GPIOA, Gpioa, PA3_PIN, GpioPin, GPIOA_PERIPH, PA3_OWNED, PA3_REF_COUNT, 3);
   ::bobbin_mcu::pin_source!(Pa3, super::adc::Adc1Ch8, super::sig::SigAdc, 0);
   ::bobbin_mcu::pin_source!(Pa3, super::tim_gen::Tim2Ch4, super::sig::SigTim, 1);
   ::bobbin_mcu::pin_source!(Pa3, super::usart::Usart2, super::sig::SigRx, 7);
   ::bobbin_mcu::pin_source!(Pa3, super::lpuart::Lpuart1, super::sig::SigRx, 8);
   ::bobbin_mcu::pin_source!(Pa3, super::tim_gen::Tim15Ch2, super::sig::SigTim, 14);
::bobbin_mcu::pin!(PA4, Pa4, pa4, GPIOA, Gpioa, PA4_PIN, GpioPin, GPIOA_PERIPH, PA4_OWNED, PA4_REF_COUNT, 4);
   ::bobbin_mcu::pin_source!(Pa4, super::adc::Adc1Ch9, super::sig::SigAdc, 0);
   ::bobbin_mcu::pin_source!(Pa4, super::dac::Dac1Ch1, super::sig::SigDac, 0);
   ::bobbin_mcu::pin_source!(Pa4, super::spi::Spi1, super::sig::SigSpiNss, 5);
   ::bobbin_mcu::pin_source!(Pa4, super::usart::Usart2, super::sig::SigCk, 7);
::bobbin_mcu::pin!(PA5, Pa5, pa5, GPIOA, Gpioa, PA5_PIN, GpioPin, GPIOA_PERIPH, PA5_OWNED, PA5_REF_COUNT, 5);
   ::bobbin_mcu::pin_source!(Pa5, super::adc::Adc1Ch10, super::sig::SigAdc, 0);
   ::bobbin_mcu::pin_source!(Pa5, super::dac::Dac1Ch2, super::sig::SigDac, 0);
   ::bobbin_mcu::pin_source!(Pa5, super::tim_gen::Tim2Ch1, super::sig::SigTim, 1);
   ::bobbin_mcu::pin_source!(Pa5, super::tim_gen::Tim2, super::sig::SigEtr, 2);
   ::bobbin_mcu::pin_source!(Pa5, super::spi::Spi1, super::sig::SigSpiSck, 5);
::bobbin_mcu::pin!(PA6, Pa6, pa6, GPIOA, Gpioa, PA6_PIN, GpioPin, GPIOA_PERIPH, PA6_OWNED, PA6_REF_COUNT, 6);
   ::bobbin_mcu::pin_source!(Pa6, super::adc::Adc1Ch11, super::sig::SigAdc, 0);
   ::bobbin_mcu::pin_source!(Pa6, super::spi::Spi1, super::sig::SigSpiMiso, 5);
   ::bobbin_mcu::pin_source!(Pa6, super::usart::Usart3, super::sig::SigCts, 7);
   ::bobbin_mcu::pin_source!(Pa6, super::lpuart::Lpuart1, super::sig::SigCts, 8);
   ::bobbin_mcu::pin_source!(Pa6, super::tim_gen::Tim16Ch1, super::sig::SigTim, 14);
::bobbin_mcu::pin!(PA7, Pa7, pa7, GPIOA, Gpioa, PA7_PIN, GpioPin, GPIOA_PERIPH, PA7_OWNED, PA7_REF_COUNT, 7);
   ::bobbin_mcu::pin_source!(Pa7, super::adc::Adc1Ch12, super::sig::SigAdc, 0);
   ::bobbin_mcu::pin_source!(Pa7, super::spi::Spi1, super::sig::SigSpiMosi, 5);
::bobbin_mcu::pin!(PA8, Pa8, pa8, GPIOA, Gpioa, PA8_PIN, GpioPin, GPIOA_PERIPH, PA8_OWNED, PA8_REF_COUNT, 8);
   ::bobbin_mcu::pin_source!(Pa8, super::usart::Usart1, super::sig::SigCk, 7);
::bobbin_mcu::pin!(PA9, Pa9, pa9, GPIOA, Gpioa, PA9_PIN, GpioPin, GPIOA_PERIPH, PA9_OWNED, PA9_REF_COUNT, 9);
   ::bobbin_mcu::pin_source!(Pa9, super::i2c::I2c1, super::sig::SigI2cScl, 4);
   ::bobbin_mcu::pin_source!(Pa9, super::usart::Usart1, super::sig::SigTx, 7);
::bobbin_mcu::pin!(PA10, Pa10, pa10, GPIOA, Gpioa, PA10_PIN, GpioPin, GPIOA_PERIPH, PA10_OWNED, PA10_REF_COUNT, 10);
   ::bobbin_mcu::pin_source!(Pa10, super::i2c::I2c1, super::sig::SigI2cSda, 4);
   ::bobbin_mcu::pin_source!(Pa10, super::usart::Usart1, super::sig::SigRx, 7);
::bobbin_mcu::pin!(PA11, Pa11, pa11, GPIOA, Gpioa, PA11_PIN, GpioPin, GPIOA_PERIPH, PA11_OWNED, PA11_REF_COUNT, 11);
   ::bobbin_mcu::pin_source!(Pa11, super::spi::Spi1, super::sig::SigSpiMiso, 5);
   ::bobbin_mcu::pin_source!(Pa11, super::usart::Usart1, super::sig::SigCts, 7);
::bobbin_mcu::pin!(PA12, Pa12, pa12, GPIOA, Gpioa, PA12_PIN, GpioPin, GPIOA_PERIPH, PA12_OWNED, PA12_REF_COUNT, 12);
   ::bobbin_mcu::pin_source!(Pa12, super::spi::Spi1, super::sig::SigSpiMosi, 5);
::bobbin_mcu::pin!(PA13, Pa13, pa13, GPIOA, Gpioa, PA13_PIN, GpioPin, GPIOA_PERIPH, PA13_OWNED, PA13_REF_COUNT, 13);
::bobbin_mcu::pin!(PA14, Pa14, pa14, GPIOA, Gpioa, PA14_PIN, GpioPin, GPIOA_PERIPH, PA14_OWNED, PA14_REF_COUNT, 14);
   ::bobbin_mcu::pin_source!(Pa14, super::i2c::I2c1, super::sig::SigI2cSmba, 4);
::bobbin_mcu::pin!(PA15, Pa15, pa15, GPIOA, Gpioa, PA15_PIN, GpioPin, GPIOA_PERIPH, PA15_OWNED, PA15_REF_COUNT, 15);
   ::bobbin_mcu::pin_source!(Pa15, super::tim_gen::Tim2Ch1, super::sig::SigTim, 1);
   ::bobbin_mcu::pin_source!(Pa15, super::tim_gen::Tim2, super::sig::SigEtr, 2);
   ::bobbin_mcu::pin_source!(Pa15, super::usart::Usart2, super::sig::SigRx, 3);
   ::bobbin_mcu::pin_source!(Pa15, super::spi::Spi1, super::sig::SigSpiNss, 5);
::bobbin_mcu::pin!(PB0, Pb0, pb0, GPIOB, Gpiob, PB0_PIN, GpioPin, GPIOB_PERIPH, PB0_OWNED, PB0_REF_COUNT, 0);
   ::bobbin_mcu::pin_source!(Pb0, super::adc::Adc1Ch15, super::sig::SigAdc, 0);
   ::bobbin_mcu::pin_source!(Pb0, super::spi::Spi1, super::sig::SigSpiNss, 5);
   ::bobbin_mcu::pin_source!(Pb0, super::usart::Usart3, super::sig::SigCk, 7);
::bobbin_mcu::pin!(PB1, Pb1, pb1, GPIOB, Gpiob, PB1_PIN, GpioPin, GPIOB_PERIPH, PB1_OWNED, PB1_REF_COUNT, 1);
::bobbin_mcu::pin!(PB3, Pb3, pb3, GPIOB, Gpiob, PB3_PIN, GpioPin, GPIOB_PERIPH, PB3_OWNED, PB3_REF_COUNT, 3);
   ::bobbin_mcu::pin_source!(Pb3, super::tim_gen::Tim2Ch2, super::sig::SigTim, 1);
   ::bobbin_mcu::pin_source!(Pb3, super::spi::Spi1, super::sig::SigSpiSck, 5);
::bobbin_mcu::pin!(PB4, Pb4, pb4, GPIOB, Gpiob, PB4_PIN, GpioPin, GPIOB_PERIPH, PB4_OWNED, PB4_REF_COUNT, 4);
   ::bobbin_mcu::pin_source!(Pb4, super::spi::Spi1, super::sig::SigSpiMiso, 5);
   ::bobbin_mcu::pin_source!(Pb4, super::usart::Usart1, super::sig::SigCts, 7);
::bobbin_mcu::pin!(PB5, Pb5, pb5, GPIOB, Gpiob, PB5_PIN, GpioPin, GPIOB_PERIPH, PB5_OWNED, PB5_REF_COUNT, 5);
   ::bobbin_mcu::pin_source!(Pb5, super::i2c::I2c1, super::sig::SigI2cSmba, 4);
   ::bobbin_mcu::pin_source!(Pb5, super::spi::Spi1, super::sig::SigSpiMosi, 5);
   ::bobbin_mcu::pin_source!(Pb5, super::usart::Usart1, super::sig::SigCk, 7);
::bobbin_mcu::pin!(PB6, Pb6, pb6, GPIOB, Gpiob, PB6_PIN, GpioPin, GPIOB_PERIPH, PB6_OWNED, PB6_REF_COUNT, 6);
   ::bobbin_mcu::pin_source!(Pb6, super::i2c::I2c1, super::sig::SigI2cScl, 4);
   ::bobbin_mcu::pin_source!(Pb6, super::usart::Usart1, super::sig::SigTx, 7);
::bobbin_mcu::pin!(PB7, Pb7, pb7, GPIOB, Gpiob, PB7_PIN, GpioPin, GPIOB_PERIPH, PB7_OWNED, PB7_REF_COUNT, 7);
   ::bobbin_mcu::pin_source!(Pb7, super::i2c::I2c1, super::sig::SigI2cSda, 4);
   ::bobbin_mcu::pin_source!(Pb7, super::usart::Usart1, super::sig::SigRx, 7);
::bobbin_mcu::pin!(PC0, Pc0, pc0, GPIOC, Gpioc, PC0_PIN, GpioPin, GPIOC_PERIPH, PC0_OWNED, PC0_REF_COUNT, 0);
   ::bobbin_mcu::pin_source!(Pc0, super::lpuart::Lpuart1, super::sig::SigRx, 6);
::bobbin_mcu::pin!(PC14, Pc14, pc14, GPIOC, Gpioc, PC14_PIN, GpioPin, GPIOC_PERIPH, PC14_OWNED, PC14_REF_COUNT, 14);
::bobbin_mcu::pin!(PC15, Pc15, pc15, GPIOC, Gpioc, PC15_PIN, GpioPin, GPIOC_PERIPH, PC15_OWNED, PC15_REF_COUNT, 15);
::bobbin_mcu::pin!(PH3, Ph3, ph3, GPIOH, Gpioh, PH3_PIN, GpioPin, GPIOH_PERIPH, PH3_OWNED, PH3_REF_COUNT, 3);
