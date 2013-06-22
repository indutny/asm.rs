pub mod x64;

#[deriving(Clone)]
pub struct MasmOffset(uint);

#[deriving(Clone)]
pub enum RelocationKind {
  Absolute,
  Relative
}

#[deriving(Clone)]
pub enum RelocationValue {
  RelocByte(u8),
  RelocWord(u16),
  RelocLong(u32),
  RelocQuad(u64)
}

#[deriving(Clone)]
pub struct RelocationInfo {
  kind: RelocationKind,
  offset: MasmOffset,
  value: RelocationValue
}

pub struct Label {
  offset: Option<MasmOffset>,
  infos: ~[RelocationInfo]
}

pub trait MasmBuffer {
  fn emitb(&mut self, b: u8);
  fn emitw(&mut self, w: u16);
  fn emitl(&mut self, l: u32);
  fn emitq(&mut self, q: u64);
  fn offset(&self) -> MasmOffset;
  fn relocate(&mut self, info: &RelocationInfo);
}
