use asm::*;
use std::libc::*;
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

fn round_up(x: c_long, f: c_long) -> c_long {
  let r = if x % f == 0 {
    x
  } else {
    x + f - (x % f)
  };
  if (r == 0) {
    f
  } else {
    r
  }
}

impl Asm {
  pub fn new() -> Asm {
    Asm { buffer: ~[], infos: ~[] }
  }

  pub fn execute(&self, arg: uint) -> uint {
    let f: extern "Rust" fn(uint) -> uint = unsafe {
      let len = round_up(self.buffer.len() as c_long,
                         sysconf(_SC_PAGESIZE));
      let addr = mmap(ptr::null(),
                      len as size_t,
                      PROT_READ | PROT_WRITE | PROT_EXEC,
                      MAP_ANON | MAP_PRIVATE,
                      -1,
                      0);
      assert!(addr != MAP_FAILED);
      ptr::copy_memory(cast::transmute(addr),
                       vec::raw::to_ptr(self.buffer),
                       self.buffer.len());

      cast::transmute(addr)
    };
    f(arg)
  }
}
