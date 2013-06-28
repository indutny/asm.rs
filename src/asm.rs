pub mod ia32 {
  pub use asm::ia32::base::*;
  pub use asm::ia32::basic::*;
  pub use asm::ia32::math::*;
  pub use asm::ia32::branching::*;
  pub use asm::ia32::fp::*;

  pub mod base;
  pub mod basic;
  pub mod math;
  pub mod branching;
  pub mod fp;
}

pub mod x64 {
  pub use asm::x64::base::*;
  pub use asm::x64::basic::*;
  pub use asm::x64::math::*;
  pub use asm::x64::branching::*;
  pub use asm::x64::fp::*;

  pub mod base;
  pub mod basic;
  pub mod math;
  pub mod branching;
  pub mod fp;
}

#[deriving(Clone)]
pub struct AsmOffset(uint);

#[deriving(Clone, Eq)]
pub enum RelocationSize {
  RelocByte,
  RelocWord,
  RelocLong,
  RelocQuad
}

#[deriving(Clone)]
pub enum RelocationKind {
  RelocAbsolute,
  RelocRelative
}

#[deriving(Clone)]
pub struct RelocationInfo {
  kind: RelocationKind,
  size: RelocationSize,
  nudge: int,
  from: AsmOffset,
  to: AsmOffset
}

pub struct LabelRef {
  kind: RelocationKind,
  size: RelocationSize,
  nudge: int,
  from: AsmOffset
}

pub struct Label {
  offset: Option<AsmOffset>,
  refs: ~[LabelRef]
}

pub trait AsmBuffer {
  fn emitb(&mut self, b: u8);
  fn emitw(&mut self, w: u16);
  fn emitl(&mut self, l: u32);
  fn emitq(&mut self, q: u64);
  fn offset(&self) -> AsmOffset;
  fn relocate(&mut self, info: &RelocationInfo);
}

pub trait AsmHelper {
  fn bind(&mut self, l: &mut Label);
  fn emit_use(&mut self,
              l: &mut Label,
              kind: RelocationKind,
              size: RelocationSize,
              nudge: int);
}

impl Label {
  pub fn new() -> Label { Label { offset: None, refs: ~[] } }
}

impl<A: AsmBuffer> AsmHelper for A {
  fn bind(&mut self, l: &mut Label) {
    l.offset = Some(self.offset());
    for l.refs.iter().advance |r| {
      let info = RelocationInfo {
        kind: r.kind,
        size: r.size,
        nudge: r.nudge,
        from: r.from,
        to: l.offset.unwrap()
      };
      self.relocate(&info);
    }
  }

  fn emit_use(&mut self,
              l: &mut Label,
              kind: RelocationKind,
              size: RelocationSize,
              nudge: int) {
    if l.offset.is_none() {
      l.refs.push(LabelRef {
        kind: kind,
        size: size,
        nudge: nudge,
        from: self.offset()
      });
    } else {
      let info = RelocationInfo {
        kind: kind,
        size: size,
        nudge: nudge,
        from: self.offset(),
        to: l.offset.unwrap()
      };
      self.relocate(&info);
    }
    match size {
      RelocByte => self.emitb(0),
      RelocWord => self.emitw(0),
      RelocLong => self.emitl(0),
      RelocQuad => self.emitq(0)
    }
  }
}
