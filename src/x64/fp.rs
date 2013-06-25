use asm::*;
use asm::x64::base::*;

pub enum RoundMode {
  RoundNearest = 0,
  RoundDown = 1,
  RoundUp = 2,
  RoundToward = 3
}

pub trait AsmX64FP {
  // Regular
  fn movsd(&mut self, dst: Operand, src: Operand);
  fn movqd(&mut self, dst: Operand, src: Operand);
  fn addsd(&mut self, dst: Operand, src: Operand);
  fn subsd(&mut self, dst: Operand, src: Operand);
  fn mulsd(&mut self, dst: Operand, src: Operand);
  fn divsd(&mut self, dst: Operand, src: Operand);

  // Binary
  fn andpd(&mut self, dst: Operand, src: Operand);
  fn orpd(&mut self, dst: Operand, src: Operand);
  fn xorpd(&mut self, dst: Operand, src: Operand);

  // Conversion
  fn cvtsi2sd(&mut self, dst: Operand, src: Operand);
  fn cvtsd2si(&mut self, dst: Operand, src: Operand);
  fn cvttsd2si(&mut self, dst: Operand, src: Operand);
  fn roundsd(&mut self, dst: Operand, src: Operand, mode: RoundMode);

  // Branching
  fn ucomisd(&mut self, dst: Operand, src: Operand);
}

impl<A: AsmBuffer+AsmX64Helper> AsmX64FP for A {
  fn movsd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0xf2);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x10);
        self.emit_modrm(dst, src);
      },
      (_, D(_)) if dst.is_dm() => {
        self.emitb(0xf2);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x11);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn movqd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0xf3);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x7e);
        self.emit_modrm(dst, src);
      },
      (_, D(_)) if dst.is_dm() => {
        self.emitb(0x66);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0xd6);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn addsd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0xf2);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x58);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn subsd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0xf2);
        self.emit_opt_rex(dst, src);
        self.emitb(0x05);
        self.emitb(0xfc);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn mulsd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0xf2);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x59);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn divsd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0xf2);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x5e);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn andpd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0x66);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x54);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn orpd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0x66);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x56);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn xorpd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0x66);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x57);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn cvtsi2sd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_rm() => {
        self.emitb(0xf2);
        self.emit_rex(REXW, dst, src);
        self.emitb(0x0f);
        self.emitb(0x2a);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn cvtsd2si(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(_), _) if src.is_dm() => {
        self.emitb(0xf2);
        self.emit_rex(REXW, dst, src);
        self.emitb(0x0f);
        self.emitb(0x2d);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn cvttsd2si(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(_), _) if src.is_dm() => {
        self.emitb(0xf2);
        self.emit_rex(REXW, dst, src);
        self.emitb(0x0f);
        self.emitb(0x2c);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn roundsd(&mut self, dst: Operand, src: Operand, mode: RoundMode) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0x66);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x3a);
        self.emitb(0x0b);
        self.emit_modrm(dst, src);

        self.emitb(0b1000 | (mode as u8));
      },
      _ => fail!()
    }
  }

  fn ucomisd(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (D(_), _) if src.is_dm() => {
        self.emitb(0x66);
        self.emit_opt_rex(dst, src);
        self.emitb(0x0f);
        self.emitb(0x2e);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }
}
