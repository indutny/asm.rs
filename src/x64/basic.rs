use asm::*;
use asm::x64::base::*;

pub trait AsmX64Basic {
  fn pushq(&mut self, op: Operand);
  fn popq(&mut self, op: Operand);
  fn ret(&mut self, r: Operand);
  fn movq(&mut self, dst: Operand, src: Operand);
}

impl<A: AsmBuffer+AsmX64Helper> AsmX64Basic for A {
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
}
