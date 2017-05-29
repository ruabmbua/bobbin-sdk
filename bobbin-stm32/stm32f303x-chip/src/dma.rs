pub const DMA1: Dma = Dma(0x40020000);
pub const DMA2: Dma = Dma(0x40020400);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dma(pub u32);

impl Dma {
  pub unsafe fn isr(&self) -> Isr { 
     Isr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
  }

  pub unsafe fn set_ifcr(&mut self, value: Ifcr) {
     ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
  }

  pub unsafe fn ccr(&self, index: usize) -> Ccr { 
     assert!(index < 7);
     Ccr(::core::ptr::read_volatile(((self.0 as usize) + 0x8 + (index * 20)) as *const u32))
  }
  pub unsafe fn set_ccr(&mut self, index: usize, value: Ccr) {
     assert!(index < 7);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x8 + (index * 20)) as *mut u32, value.0);
  }
  pub unsafe fn with_ccr<F: FnOnce(Ccr) -> Ccr>(&mut self, index: usize, f: F) {
     let tmp = self.ccr(index);
     self.set_ccr(index, f(tmp))
  }

  pub unsafe fn cndtr(&self, index: usize) -> Cndtr { 
     assert!(index < 7);
     Cndtr(::core::ptr::read_volatile(((self.0 as usize) + 0xc + (index * 20)) as *const u32))
  }
  pub unsafe fn set_cndtr(&mut self, index: usize, value: Cndtr) {
     assert!(index < 7);
     ::core::ptr::write_volatile(((self.0 as usize) + 0xc + (index * 20)) as *mut u32, value.0);
  }
  pub unsafe fn with_cndtr<F: FnOnce(Cndtr) -> Cndtr>(&mut self, index: usize, f: F) {
     let tmp = self.cndtr(index);
     self.set_cndtr(index, f(tmp))
  }

  pub unsafe fn cpar(&self, index: usize) -> Cpar { 
     assert!(index < 7);
     Cpar(::core::ptr::read_volatile(((self.0 as usize) + 0x10 + (index * 20)) as *const u32))
  }
  pub unsafe fn set_cpar(&mut self, index: usize, value: Cpar) {
     assert!(index < 7);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x10 + (index * 20)) as *mut u32, value.0);
  }
  pub unsafe fn with_cpar<F: FnOnce(Cpar) -> Cpar>(&mut self, index: usize, f: F) {
     let tmp = self.cpar(index);
     self.set_cpar(index, f(tmp))
  }

  pub unsafe fn cmar(&self, index: usize) -> Cmar { 
     assert!(index < 7);
     Cmar(::core::ptr::read_volatile(((self.0 as usize) + 0x14 + (index * 20)) as *const u32))
  }
  pub unsafe fn set_cmar(&mut self, index: usize, value: Cmar) {
     assert!(index < 7);
     ::core::ptr::write_volatile(((self.0 as usize) + 0x14 + (index * 20)) as *mut u32, value.0);
  }
  pub unsafe fn with_cmar<F: FnOnce(Cmar) -> Cmar>(&mut self, index: usize, f: F) {
     let tmp = self.cmar(index);
     self.set_cmar(index, f(tmp))
  }

}

#[derive(PartialEq, Eq)]
pub struct Isr(pub u32);

impl Isr {
  pub fn gif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_gif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn tcif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 1 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  pub fn set_tcif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn htif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 2 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  pub fn set_htif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn teif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 3 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [3]
  }
  pub fn set_teif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 3 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.gif(0) != 0 { try!(write!(f, " gif[0]"))}
      if self.gif(1) != 0 { try!(write!(f, " gif[1]"))}
      if self.gif(2) != 0 { try!(write!(f, " gif[2]"))}
      if self.gif(3) != 0 { try!(write!(f, " gif[3]"))}
      if self.gif(4) != 0 { try!(write!(f, " gif[4]"))}
      if self.gif(5) != 0 { try!(write!(f, " gif[5]"))}
      if self.gif(6) != 0 { try!(write!(f, " gif[6]"))}
      if self.tcif(0) != 0 { try!(write!(f, " tcif[0]"))}
      if self.tcif(1) != 0 { try!(write!(f, " tcif[1]"))}
      if self.tcif(2) != 0 { try!(write!(f, " tcif[2]"))}
      if self.tcif(3) != 0 { try!(write!(f, " tcif[3]"))}
      if self.tcif(4) != 0 { try!(write!(f, " tcif[4]"))}
      if self.tcif(5) != 0 { try!(write!(f, " tcif[5]"))}
      if self.tcif(6) != 0 { try!(write!(f, " tcif[6]"))}
      if self.htif(0) != 0 { try!(write!(f, " htif[0]"))}
      if self.htif(1) != 0 { try!(write!(f, " htif[1]"))}
      if self.htif(2) != 0 { try!(write!(f, " htif[2]"))}
      if self.htif(3) != 0 { try!(write!(f, " htif[3]"))}
      if self.htif(4) != 0 { try!(write!(f, " htif[4]"))}
      if self.htif(5) != 0 { try!(write!(f, " htif[5]"))}
      if self.htif(6) != 0 { try!(write!(f, " htif[6]"))}
      if self.teif(0) != 0 { try!(write!(f, " teif[0]"))}
      if self.teif(1) != 0 { try!(write!(f, " teif[1]"))}
      if self.teif(2) != 0 { try!(write!(f, " teif[2]"))}
      if self.teif(3) != 0 { try!(write!(f, " teif[3]"))}
      if self.teif(4) != 0 { try!(write!(f, " teif[4]"))}
      if self.teif(5) != 0 { try!(write!(f, " teif[5]"))}
      if self.teif(6) != 0 { try!(write!(f, " teif[6]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ifcr(pub u32);

impl Ifcr {
  pub fn cgif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 0 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [0]
  }
  pub fn set_cgif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn ctcif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 1 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [1]
  }
  pub fn set_ctcif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 1 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn chtif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 2 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [2]
  }
  pub fn set_chtif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 2 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

  pub fn cteif(&self, index: usize) -> u32 {
     assert!(index < 7);
     let shift: usize = 3 + (index << 2);
     ((self.0 as u32) >> shift) & 0x1 // [3]
  }
  pub fn set_cteif(mut self, index: usize, value: u32) -> Self {
     assert!(index < 7);
     assert!((value & !0x1) == 0);
     let shift: usize = 3 + (index << 2);
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}

impl ::core::fmt::Display for Ifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ifcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cgif(0) != 0 { try!(write!(f, " cgif[0]"))}
      if self.cgif(1) != 0 { try!(write!(f, " cgif[1]"))}
      if self.cgif(2) != 0 { try!(write!(f, " cgif[2]"))}
      if self.cgif(3) != 0 { try!(write!(f, " cgif[3]"))}
      if self.cgif(4) != 0 { try!(write!(f, " cgif[4]"))}
      if self.cgif(5) != 0 { try!(write!(f, " cgif[5]"))}
      if self.cgif(6) != 0 { try!(write!(f, " cgif[6]"))}
      if self.ctcif(0) != 0 { try!(write!(f, " ctcif[0]"))}
      if self.ctcif(1) != 0 { try!(write!(f, " ctcif[1]"))}
      if self.ctcif(2) != 0 { try!(write!(f, " ctcif[2]"))}
      if self.ctcif(3) != 0 { try!(write!(f, " ctcif[3]"))}
      if self.ctcif(4) != 0 { try!(write!(f, " ctcif[4]"))}
      if self.ctcif(5) != 0 { try!(write!(f, " ctcif[5]"))}
      if self.ctcif(6) != 0 { try!(write!(f, " ctcif[6]"))}
      if self.chtif(0) != 0 { try!(write!(f, " chtif[0]"))}
      if self.chtif(1) != 0 { try!(write!(f, " chtif[1]"))}
      if self.chtif(2) != 0 { try!(write!(f, " chtif[2]"))}
      if self.chtif(3) != 0 { try!(write!(f, " chtif[3]"))}
      if self.chtif(4) != 0 { try!(write!(f, " chtif[4]"))}
      if self.chtif(5) != 0 { try!(write!(f, " chtif[5]"))}
      if self.chtif(6) != 0 { try!(write!(f, " chtif[6]"))}
      if self.cteif(0) != 0 { try!(write!(f, " cteif[0]"))}
      if self.cteif(1) != 0 { try!(write!(f, " cteif[1]"))}
      if self.cteif(2) != 0 { try!(write!(f, " cteif[2]"))}
      if self.cteif(3) != 0 { try!(write!(f, " cteif[3]"))}
      if self.cteif(4) != 0 { try!(write!(f, " cteif[4]"))}
      if self.cteif(5) != 0 { try!(write!(f, " cteif[5]"))}
      if self.cteif(6) != 0 { try!(write!(f, " cteif[6]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Ccr(pub u32);

impl Ccr {
  pub fn en(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
  pub fn set_en(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

  pub fn tcie(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
  pub fn set_tcie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  pub fn htie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
  pub fn set_htie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  pub fn teie(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
  pub fn set_teie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

  pub fn dir(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
  pub fn set_dir(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

  pub fn circ(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
  pub fn set_circ(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

  pub fn pinc(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
  pub fn set_pinc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

  pub fn minc(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
  pub fn set_minc(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

  pub fn psize(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x3 // [9:8]
  }
  pub fn set_psize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 8);
     self.0 |= value << 8;
     self
  }

  pub fn msize(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x3 // [11:10]
  }
  pub fn set_msize(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 10);
     self.0 |= value << 10;
     self
  }

  pub fn pl(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x3 // [13:12]
  }
  pub fn set_pl(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 12);
     self.0 |= value << 12;
     self
  }

  pub fn mem2mem(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
  pub fn set_mem2mem(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

}

impl ::core::fmt::Display for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Ccr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.en() != 0 { try!(write!(f, " en"))}
      if self.tcie() != 0 { try!(write!(f, " tcie"))}
      if self.htie() != 0 { try!(write!(f, " htie"))}
      if self.teie() != 0 { try!(write!(f, " teie"))}
      if self.dir() != 0 { try!(write!(f, " dir"))}
      if self.circ() != 0 { try!(write!(f, " circ"))}
      if self.pinc() != 0 { try!(write!(f, " pinc"))}
      if self.minc() != 0 { try!(write!(f, " minc"))}
      if self.psize() != 0 { try!(write!(f, " psize=0x{:x}", self.psize()))}
      if self.msize() != 0 { try!(write!(f, " msize=0x{:x}", self.msize()))}
      if self.pl() != 0 { try!(write!(f, " pl=0x{:x}", self.pl()))}
      if self.mem2mem() != 0 { try!(write!(f, " mem2mem"))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cndtr(pub u32);

impl Cndtr {
  pub fn ndt(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
  pub fn set_ndt(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Cndtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cndtr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ndt() != 0 { try!(write!(f, " ndt=0x{:x}", self.ndt()))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cpar(pub u32);

impl Cpar {
  pub fn pa(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_pa(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Cpar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cpar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

#[derive(PartialEq, Eq)]
pub struct Cmar(pub u32);

impl Cmar {
  pub fn ma(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
  pub fn set_ma(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}

impl ::core::fmt::Display for Cmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}

impl ::core::fmt::Debug for Cmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

