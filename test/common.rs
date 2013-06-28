use asm::*;
use std::os;
use std::ptr;
use std::cast;
use std::vec;

pub struct Asm {
  buffer: ~[u8],
  infos: ~[RelocationInfo]
}

impl AsmBuffer for Asm {
  fn emitb(&mut self, b: u8) {
    self.buffer.push(b);
  }

  fn emitw(&mut self, w: u16) {
    self.emitb((w & 0xff) as u8);
    self.emitb(((w >> 8) & 0xff) as u8);
  }

  fn emitl(&mut self, l: u32) {
    self.emitw((l & 0xffff) as u16);
    self.emitw(((l >> 16) & 0xffff) as u16);
  }

  fn emitq(&mut self, q: u64) {
    self.emitl((q & 0xffff_ffff) as u32);
    self.emitl(((q >> 32) & 0xffff_ffff) as u32);
  }

  fn offset(&self) -> AsmOffset {
    AsmOffset(self.buffer.len())
  }

  fn relocate(&mut self, info: &RelocationInfo) {
    self.infos.push(info.clone());
  }
}

impl Asm {
  pub fn new() -> Asm {
    Asm { buffer: ~[], infos: ~[] }
  }

  pub fn execute(&self, arg: uint) -> uint {
    let addr = match os::MmapChunk::new(self.buffer.len(), ~[
      os::MmapReadable,
      os::MmapWritable,
      os::MmapExecutable
    ]) {
      Ok(r) => r,
      Err(_) => fail!()
    };

    unsafe {
      let data: *mut u8 = cast::transmute(addr.data);
      ptr::copy_memory(data,
                       vec::raw::to_ptr(self.buffer),
                       self.buffer.len());

      for self.infos.iter().advance |info| {
        let AsmOffset(from) = info.from;
        let AsmOffset(to) = info.to;

        match info.kind {
          RelocAbsolute => {
            assert!(info.size == RelocQuad);
            let to_abs = (data as u64) + to as u64;
            let p: *mut u64 = cast::transmute(data.offset(from));
            *p = to_abs;
          },
          RelocRelative => {
            let delta = (to as int) - (from as int) + info.nudge;
            match info.size {
              RelocByte => {
                assert!(-127 <= delta && delta <= 128);
                let p: *mut u8 = cast::transmute(data.offset(from));
                *p = delta as u8;
              },
              RelocWord => {
                assert!(-32767 <= delta && delta <= 32768);
                let p: *mut u16 = cast::transmute(data.offset(from));
                *p = delta as u16;
              },
              RelocLong => {
                assert!(-8388607 <= delta && delta <= 8388608);
                let p: *mut u32 = cast::transmute(data.offset(from));
                *p = delta as u32;
              },
              RelocQuad => {
                let p: *mut u64 = cast::transmute(data.offset(from));
                *p = delta as u64;
              }
            }
          }
        }
      };
      let f: extern "Rust" fn(uint) -> uint = cast::transmute(data);
      f(arg)
    }
  }
}

pub fn run_test(arg: uint, expected: uint, test: &fn(m: &mut Asm)) {
  let mut m = ~Asm::new();
  test(m);

  assert!(m.execute(arg) == expected);
}
