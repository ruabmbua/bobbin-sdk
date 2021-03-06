pub use ::stm32_common::dma_f3::*;

::bobbin_mcu::periph!( DMA1, Dma1, DMA1_PERIPH, DmaPeriph, DMA1_OWNED, DMA1_REF_COUNT, 0x40020000, 0x00, 0x2e);
::bobbin_mcu::periph!( DMA2, Dma2, DMA2_PERIPH, DmaPeriph, DMA2_OWNED, DMA2_REF_COUNT, 0x40020400, 0x01, 0x2f);

::bobbin_mcu::channel!(DMA1_CH1, Dma1Ch1, dma1_ch1, DMA1, Dma1, DMA1_CH1_CH, DmaCh, DMA1_PERIPH, DMA1_CH1_OWNED, DMA1_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(DMA1_CH2, Dma1Ch2, dma1_ch2, DMA1, Dma1, DMA1_CH2_CH, DmaCh, DMA1_PERIPH, DMA1_CH2_OWNED, DMA1_CH2_REF_COUNT, 1);
::bobbin_mcu::channel!(DMA1_CH3, Dma1Ch3, dma1_ch3, DMA1, Dma1, DMA1_CH3_CH, DmaCh, DMA1_PERIPH, DMA1_CH3_OWNED, DMA1_CH3_REF_COUNT, 2);
::bobbin_mcu::channel!(DMA1_CH4, Dma1Ch4, dma1_ch4, DMA1, Dma1, DMA1_CH4_CH, DmaCh, DMA1_PERIPH, DMA1_CH4_OWNED, DMA1_CH4_REF_COUNT, 3);
::bobbin_mcu::channel!(DMA1_CH5, Dma1Ch5, dma1_ch5, DMA1, Dma1, DMA1_CH5_CH, DmaCh, DMA1_PERIPH, DMA1_CH5_OWNED, DMA1_CH5_REF_COUNT, 4);
::bobbin_mcu::channel!(DMA1_CH6, Dma1Ch6, dma1_ch6, DMA1, Dma1, DMA1_CH6_CH, DmaCh, DMA1_PERIPH, DMA1_CH6_OWNED, DMA1_CH6_REF_COUNT, 5);
::bobbin_mcu::channel!(DMA1_CH7, Dma1Ch7, dma1_ch7, DMA1, Dma1, DMA1_CH7_CH, DmaCh, DMA1_PERIPH, DMA1_CH7_OWNED, DMA1_CH7_REF_COUNT, 6);
::bobbin_mcu::channel!(DMA2_CH1, Dma2Ch1, dma2_ch1, DMA2, Dma2, DMA2_CH1_CH, DmaCh, DMA2_PERIPH, DMA2_CH1_OWNED, DMA2_CH1_REF_COUNT, 0);
::bobbin_mcu::channel!(DMA2_CH2, Dma2Ch2, dma2_ch2, DMA2, Dma2, DMA2_CH2_CH, DmaCh, DMA2_PERIPH, DMA2_CH2_OWNED, DMA2_CH2_REF_COUNT, 1);
::bobbin_mcu::channel!(DMA2_CH3, Dma2Ch3, dma2_ch3, DMA2, Dma2, DMA2_CH3_CH, DmaCh, DMA2_PERIPH, DMA2_CH3_OWNED, DMA2_CH3_REF_COUNT, 2);
::bobbin_mcu::channel!(DMA2_CH4, Dma2Ch4, dma2_ch4, DMA2, Dma2, DMA2_CH4_CH, DmaCh, DMA2_PERIPH, DMA2_CH4_OWNED, DMA2_CH4_REF_COUNT, 3);
::bobbin_mcu::channel!(DMA2_CH5, Dma2Ch5, dma2_ch5, DMA2, Dma2, DMA2_CH5_CH, DmaCh, DMA2_PERIPH, DMA2_CH5_OWNED, DMA2_CH5_REF_COUNT, 4);
// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("DMA1EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Dma1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().dma1en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_dma1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("DMA2EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Dma2 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().dma2en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_dma2en(value));
        self
    }
}

