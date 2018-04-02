#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::sercom::*;

periph!( SERCOM0, Sercom0, SERCOM0_PERIPH, SercomPeriph, SERCOM0_OWNED, SERCOM0_REF_COUNT, 0x42000800, 0x00, 0x11);
periph!( SERCOM1, Sercom1, SERCOM1_PERIPH, SercomPeriph, SERCOM1_OWNED, SERCOM1_REF_COUNT, 0x42000c00, 0x01, 0x12);
periph!( SERCOM2, Sercom2, SERCOM2_PERIPH, SercomPeriph, SERCOM2_OWNED, SERCOM2_REF_COUNT, 0x42001000, 0x02, 0x13);
periph!( SERCOM3, Sercom3, SERCOM3_PERIPH, SercomPeriph, SERCOM3_OWNED, SERCOM3_REF_COUNT, 0x42001400, 0x03, 0x14);
periph!( SERCOM4, Sercom4, SERCOM4_PERIPH, SercomPeriph, SERCOM4_OWNED, SERCOM4_REF_COUNT, 0x42001800, 0x04, 0x15);
periph!( SERCOM5, Sercom5, SERCOM5_PERIPH, SercomPeriph, SERCOM5_OWNED, SERCOM5_REF_COUNT, 0x42001c00, 0x05, 0x16);

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("SERCOM0"), description: None }
impl ::bobbin_common::gate::GateEn for Sercom0 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().sercom0() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_sercom0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("SERCOM1"), description: None }
impl ::bobbin_common::gate::GateEn for Sercom1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().sercom1() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_sercom1(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("SERCOM2"), description: None }
impl ::bobbin_common::gate::GateEn for Sercom2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().sercom2() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_sercom2(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("SERCOM3"), description: None }
impl ::bobbin_common::gate::GateEn for Sercom3 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().sercom3() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_sercom3(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("SERCOM4"), description: None }
impl ::bobbin_common::gate::GateEn for Sercom4 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().sercom4() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_sercom4(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBCMASK"), field: Some("SERCOM5"), description: None }
impl ::bobbin_common::gate::GateEn for Sercom5 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbcmask().sercom5() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbcmask(|r| r.set_sercom5(value));
        self
    }
}

