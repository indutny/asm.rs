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

enum REXKind {
  REX,
  REXW
}

pub trait AsmX64Helper {
  fn emit_modrm(&mut self, r: Operand, rm: Operand);
  fn emit_rex(&mut self, kind: REXKind, r: Operand, rm: Operand);
  fn emit_opt_rex(&mut self, kind: REXKind, r: Operand, rm: Operand);
}

pub trait AsmX64 {
  // Debug
  fn nop(&mut self);
  fn int3(&mut self);

  // Basics
  fn pushq(&mut self, op: Operand);
  fn popq(&mut self, op: Operand);
  fn ret(&mut self, r: Operand);
  fn movq(&mut self, dst: Operand, src: Operand);

  // Math
  fn incq(&mut self, dst: Operand);
  fn decq(&mut self, dst: Operand);
  fn addq(&mut self, dst: Operand, src: Operand);
  fn subq(&mut self, dst: Operand, src: Operand);
  fn divq(&mut self, src: Operand);
  fn mulq(&mut self, src: Operand);
  fn shlq(&mut self, dst: Operand, src: Operand);
  fn shrq(&mut self, dst: Operand, src: Operand);
  fn sarq(&mut self, dst: Operand, src: Operand);
  fn andq(&mut self, dst: Operand, src: Operand);
  fn orq(&mut self, dst: Operand, src: Operand);
  fn xorq(&mut self, dst: Operand, src: Operand);

  // Branching
  fn testq(&mut self, dst: Operand, src: Operand);
  fn cmpq(&mut self, dst: Operand, src: Operand);
  fn jmpl(&mut self, l: &mut Label);
  fn jccl(&mut self, c: JumpCondition, l: &mut Label);
  fn callq(&mut self, target: Operand);
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

  fn is_operation(&self) -> bool {
    match self { &_Operation(_) => true, _ => false }
  }

  fn high(&self) -> u8 {
    match self {
      &R(ref r) => r.high(),
      &D(ref d) => d.high(),
      &M(ref r, _) => r.high(),
      _ => 0
    }
  }

  fn low(&self) -> u8 {
    match self {
      &R(ref r) => r.low(),
      &D(ref d) => d.low(),
      &M(ref r, _) => r.low(),
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

  fn emit_opt_rex(&mut self, kind: REXKind, r: Operand, rm: Operand) {
    if (r.high() != 0 || rm.high() != 0) {
      self.emit_rex(kind, r, rm);
    }
  }
}

impl<M: AsmBuffer+AsmX64Helper> AsmX64 for M {
  fn nop(&mut self) { self.emitb(0x90); }
  fn int3(&mut self) { self.emitb(0xcc); }

  fn pushq(&mut self, op: Operand) {
    match op {
      R(_) => {
        self.emit_rex(REXW, Empty, op);
        self.emitb(0xff);
        self.emit_modrm(_Operation(6), op);
      },
      M(_, _) => {
        self.emit_rex(REXW, Empty, op);
        self.emitb(0xff);
        self.emit_modrm(_Operation(6), op);
      },
      Byte(b) => {
        self.emitb(0x6a);
        self.emitb(b);
      },
      Long(l) => {
        self.emitb(0x68);
        self.emitl(l);
      },
      _ => fail!()
    }
  }

  fn popq(&mut self, op: Operand) {
    match op {
      R(_) => {
        self.emit_rex(REXW, op, Empty);
        self.emitb(0x58 | op.low());
      },
      M(_, _) => {
        self.emitb(0x8f);
        self.emit_rex(REXW, Empty, op);
        self.emit_modrm(_Operation(0), op);
      },
      _ => fail!()
    }
  }

  fn movq(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (_, R(_)) if dst.is_rm() => {
        self.emit_rex(REXW, src, dst);
        self.emitb(0x89);
        self.emit_modrm(src, dst);
      },
      (R(_), _) if src.is_rm() => {
        self.emit_rex(REXW, dst, src);
        self.emitb(0x8b);
        self.emit_modrm(dst, src);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0xc7);
        self.emit_modrm(_Operation(0), dst);
        self.emitl(l);
      },
      (R(_), Quad(q)) => {
        self.emit_rex(REXW, dst, src);
        self.emitb(0xb8 | dst.low());
        self.emitq(q);
      },
      _ => fail!()
    }
  }

  fn incq(&mut self, dst: Operand) {
    self.emit_rex(REXW, Empty, dst);
    self.emitb(0xff);
    self.emit_modrm(_Operation(0), dst);
  }

  fn decq(&mut self, dst: Operand) {
    self.emit_rex(REXW, Empty, dst);
    self.emitb(0xff);
    self.emit_modrm(_Operation(1), dst);
  }

  fn addq(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(rax), Long(l)) => {
        self.emit_rex(REXW, Empty, Empty);
        self.emitb(0x05);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x83);
        self.emit_modrm(_Operation(0), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x81);
        self.emit_modrm(_Operation(0), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emit_rex(REXW, dst, src);
        self.emitb(0x03);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emit_rex(REXW, src, dst);
        self.emitb(0x01);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn subq(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(rax), Long(l)) => {
        self.emit_rex(REXW, Empty, Empty);
        self.emitb(0x2d);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x83);
        self.emit_modrm(_Operation(5), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x81);
        self.emit_modrm(_Operation(5), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emit_rex(REXW, dst, src);
        self.emitb(0x2b);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        fail!("You can only substract r32 from rm");
      },
      _ => fail!()
    }
  }

  fn divq(&mut self, src: Operand) {
    assert!(src.is_rm());
    self.emit_rex(REXW, Empty, src);
    self.emitb(0xf7);
    self.emit_modrm(_Operation(6), src);
  }

  fn mulq(&mut self, src: Operand) {
    assert!(src.is_rm());
    self.emit_rex(REXW, Empty, src);
    self.emitb(0xf7);
    self.emit_modrm(_Operation(4), src);
  }

  fn shlq(&mut self, dst: Operand, src: Operand) {
    assert!(dst.is_rm());
    match src {
      Byte(b) => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0xc1);
        self.emit_modrm(_Operation(4), dst);
        self.emitb(b);
      },
      _ => fail!()
    }
  }

  fn shrq(&mut self, dst: Operand, src: Operand) {
    assert!(dst.is_rm());
    match src {
      Byte(b) => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0xc1);
        self.emit_modrm(_Operation(5), dst);
        self.emitb(b);
      },
      _ => fail!()
    }
  }

  fn sarq(&mut self, dst: Operand, src: Operand) {
    assert!(dst.is_rm());
    match src {
      Byte(b) => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0xc1);
        self.emit_modrm(_Operation(7), dst);
        self.emitb(b);
      },
      _ => fail!()
    }
  }

  fn andq(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(rax), Long(l)) => {
        self.emit_rex(REXW, Empty, Empty);
        self.emitb(0x25);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x83);
        self.emit_modrm(_Operation(4), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x81);
        self.emit_modrm(_Operation(4), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emit_rex(REXW, dst, src);
        self.emitb(0x23);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emit_rex(REXW, src, dst);
        self.emitb(0x21);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn orq(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(rax), Long(l)) => {
        self.emit_rex(REXW, Empty, Empty);
        self.emitb(0x0d);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x83);
        self.emit_modrm(_Operation(1), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x81);
        self.emit_modrm(_Operation(1), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emit_rex(REXW, dst, src);
        self.emitb(0x0b);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emit_rex(REXW, src, dst);
        self.emitb(0x0a);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn xorq(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(rax), Long(l)) => {
        self.emit_rex(REXW, Empty, Empty);
        self.emitb(0x35);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x83);
        self.emit_modrm(_Operation(6), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x81);
        self.emit_modrm(_Operation(6), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emit_rex(REXW, dst, src);
        self.emitb(0x33);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emit_rex(REXW, src, dst);
        self.emitb(0x31);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn ret(&mut self, r: Operand) {
    match r {
      Empty => self.emitb(0xc3),
      Word(w) => {
        self.emitb(0xc2);
        self.emitw(w as u16);
      },
      _ => fail!()
    }
  }

  fn testq(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(rax), Long(l)) => {
        self.emit_rex(REXW, Empty, Empty);
        self.emitb(0xa9);
        self.emitl(l);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0xf7);
        self.emit_modrm(_Operation(0), dst);
        self.emitl(l);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emit_rex(REXW, src, dst);
        self.emitb(0x85);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn cmpq(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(rax), Long(l)) => {
        self.emit_rex(REXW, Empty, Empty);
        self.emitb(0x3d);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x83);
        self.emit_modrm(_Operation(7), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emit_rex(REXW, Empty, dst);
        self.emitb(0x81);
        self.emit_modrm(_Operation(7), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emit_rex(REXW, dst, src);
        self.emitb(0x3b);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emit_rex(REXW, src, dst);
        self.emitb(0x39);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn jmpl(&mut self, l: &mut Label) {
    self.emitb(0xe9);
    self.emit_use(l, RelocRelative, RelocLong, -4);
  }

  fn jccl(&mut self, c: JumpCondition, l: &mut Label) {
    self.emitb(0x0f);
    self.emitb(match c {
      IfZero => 0x84,
      IfNotZero => 0x85,
      IfOverlow => 0x80,
      IfNoOverlow => 0x81,
      IfEqual => 0x84,
      IfNotEqual => 0x85,
      IfGreater => 0x8f,
      IfLess => 0x8c,
      IfGreaterOrEqual => 0x8d,
      IfLessOrEqual => 0x8e
    });
    self.emit_use(l, RelocRelative, RelocLong, -4);
  }

  fn callq(&mut self, target: Operand) {
    self.emit_rex(REXW, Empty, target);
    self.emitb(0xff);
    self.emit_modrm(_Operation(2), target);
  }
}
