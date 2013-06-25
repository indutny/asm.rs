use asm::*;

pub enum Operand {
  Empty,
  _Operation(u8),
  R(Register),
  D(DoubleRegister),
  M(Register, u32),
  Byte(u8),
  Word(u16),
  Long(u32),
  Quad(u64),
}

pub enum Register {
  rax = 0,
  rcx = 1,
  rdx = 2,
  rbx = 3,
  rsp = 4,
  rbp = 5,
  rsi = 6,
  rdi = 7,

  r8 = 8,
  r9 = 9,
  r10 = 10,
  r11 = 11,
  r12 = 12,
  r13 = 13,
  r14 = 14,
  r15 = 15
}

pub enum DoubleRegister {
  xmm0 = 0,
  xmm1 = 1,
  xmm2 = 2,
  xmm3 = 3,
  xmm4 = 4,
  xmm5 = 5,
  xmm6 = 6,
  xmm7 = 7,
  xmm8 = 8,
  xmm9 = 9,
  xmm10 = 10,
  xmm11 = 11,
  xmm12 = 12,
  xmm13 = 13,
  xmm14 = 14,
  xmm15 = 15
}

pub enum JumpCondition {
  IfZero,
  IfNotZero,
  IfOverlow,
  IfNoOverlow,
  IfEqual,
  IfNotEqual,
  IfGreater,
  IfLess,
  IfGreaterOrEqual,
  IfLessOrEqual
}

pub enum REXKind {
  REX,
  REXW
}

pub trait AsmX64Helper {
  fn emit_modrm(&mut self, r: Operand, rm: Operand);
  fn emit_rex(&mut self, kind: REXKind, r: Operand, rm: Operand);
  fn emit_opt_rex(&mut self, r: Operand, rm: Operand);
}

pub trait AsmX64 {
  fn nop(&mut self);
  fn int3(&mut self);
}

impl Register {
  fn high(&self) -> u8 { ((*self as u8) >> 3) & 1 }
  fn low(&self) -> u8 { (*self as u8) & 0x7 }
}

impl DoubleRegister {
  fn high(&self) -> u8 { ((*self as u8) >> 3) & 1 }
  fn low(&self) -> u8 { (*self as u8) & 0x7 }
}

impl Operand {
  fn is_reg(&self) -> bool { match self { &R(_) => true, _ => false } }
  fn is_dreg(&self) -> bool { match self { &D(_) => true, _ => false } }
  fn is_mem(&self) -> bool { match self { &M(_, _) => true, _ => false } }
  fn is_rm(&self) -> bool { self.is_reg() || self.is_mem() }
  fn is_dm(&self) -> bool { self.is_dreg() || self.is_mem() }

  fn is_operation(&self) -> bool {
    match self { &_Operation(_) => true, _ => false }
  }

  fn high(&self) -> u8 {
    match self {
      &R(ref r) | &M(ref r, _) => r.high(),
      &D(ref d) => d.high(),
      _ => 0
    }
  }

  fn low(&self) -> u8 {
    match self {
      &R(ref r) | &M(ref r, _) => r.low(),
      &D(ref d) => d.low(),
      &_Operation(op) => op & 7,
      _ => 0
    }
  }
}

impl<M: AsmBuffer> AsmX64Helper for M {
  fn emit_modrm(&mut self, r: Operand, rm: Operand) {
    assert!(r.is_reg() || r.is_dreg() || r.is_operation());
    let rbit = r.low() << 3;

    match rm {
      M(ref rm, 0) => self.emitb(rbit | rm.low()),
      M(ref rm, b) if b <= 0xff => {
        self.emitb(0b0100_0000 | rbit | rm.low());
        self.emitb(b as u8)
      },
      M(ref rm, l) => {
        self.emitb(0b1000_0000 | rbit | rm.low());
        self.emitl(l)
      },
      R(ref rm) => self.emitb(0b1100_0000 | rbit | rm.low()),
      D(ref rm) => self.emitb(0b1100_0000 | rbit | rm.low()),
      Byte(_) => self.emitb(rbit),
      Word(_) => self.emitb(rbit),
      Long(_) => self.emitb(rbit),
      Quad(_) => self.emitb(rbit),
      Empty => self.emitb(rbit),
      _ => fail!()
    }
  }

  fn emit_rex(&mut self, kind: REXKind, r: Operand, rm: Operand) {
    self.emitb(match kind {
      REX => 0b0100_0000,
      REXW => 0b0100_1000
    } | (r.high() << 2) | rm.high());
  }

  fn emit_opt_rex(&mut self, r: Operand, rm: Operand) {
    if (r.high() != 0 || rm.high() != 0) {
      self.emit_rex(REX, r, rm);
    }
  }
}

impl<M: AsmBuffer+AsmX64Helper> AsmX64 for M {
  fn nop(&mut self) { self.emitb(0x90); }
  fn int3(&mut self) { self.emitb(0xcc); }
}
