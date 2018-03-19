#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::uart::*;

periph!( UART0, Uart0, UART0_PERIPH, UartPeriph, 0x4006a000, 0x14);
periph!( UART1, Uart1, UART1_PERIPH, UartPeriph, 0x4006b000, 0x15);
periph!( UART2, Uart2, UART2_PERIPH, UartPeriph, 0x4006c000, 0x16);
periph!( UART3, Uart3, UART3_PERIPH, UartPeriph, 0x4006d000, 0x17);
periph!( UART4, Uart4, UART4_PERIPH, UartPeriph, 0x400ea000, 0x18);
periph!( UART5, Uart5, UART5_PERIPH, UartPeriph, 0x400eb000, 0x19);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("UART0"), description: None }
impl ::bobbin_common::gate::GateEn for Uart0 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc4().uart0() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_uart0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("UART1"), description: None }
impl ::bobbin_common::gate::GateEn for Uart1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc4().uart1() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_uart1(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("UART2"), description: None }
impl ::bobbin_common::gate::GateEn for Uart2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc4().uart2() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_uart2(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("UART3"), description: None }
impl ::bobbin_common::gate::GateEn for Uart3 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc4().uart3() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_uart3(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC1"), field: Some("UART4"), description: None }
impl ::bobbin_common::gate::GateEn for Uart4 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc1().uart4() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc1(|r| r.set_uart4(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC1"), field: Some("UART5"), description: None }
impl ::bobbin_common::gate::GateEn for Uart5 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc1().uart5() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc1(|r| r.set_uart5(value));
        self
    }
}

