use asm::*;
use asm::ia32::base::*;

pub trait AsmIA32Basic {
  fn movl(&mut self, dst: Operand, src: Operand);
  fn movlzxb(&mut self, dst: Operand, src: Operand);
  fn movlzxl(&mut self, dst: Operand, src: Operand);
  fn movl_proc(&mut self, dst: Operand, l: &mut Label);
  fn xchgl(&mut self, dst: Operand, src: Operand);
  fn pushl(&mut self, op: Operand);
  fn popl(&mut self, op: Operand);
  fn ret(&mut self, r: Operand);
}

impl<A: AsmBuffer+AsmIA32Helper> AsmIA32Basic for A {
  fn movl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(_), _) if src.is_rm() => {
        self.emitb(0x8b);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emitb(0x89);
        self.emit_modrm(src, dst);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emitb(0xc7);
        self.emit_modrm(_Operation(0), dst);
        self.emitl(l);
      },
      (R(_), Long(l)) => {
        self.emitb(0xb8 | dst.val());
        self.emitl(l);
      },
      _ => fail!()
    }
  }

  fn movlzxb(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(_), _) if src.is_rm() => {
        self.emitb(0x0f);
        self.emitb(0xb6);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn movlzxl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(_), _) if src.is_rm() => {
        self.emitb(0x0f);
        self.emitb(0xb7);
        self.emit_modrm(dst, src);
      },
      _ => fail!()
    }
  }

  fn movl_proc(&mut self, dst: Operand, l: &mut Label) {
    match dst {
      R(_) => {
        self.emitb(0xb8 | dst.val());
        self.emit_use(l, RelocAbsolute, RelocLong, 0);
      },
      _ => fail!()
    }
  }

  fn xchgl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(eax), R(_)) => {
        self.emitb(0x90 | src.val());
      },
      (R(_), R(eax)) => {
        self.emitb(0x90 | dst.val());
      },
      (R(_), _) if src.is_rm() => {
        self.emitb(0x87);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if src.is_rm() => {
        self.emitb(0x87);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn pushl(&mut self, op: Operand) {
    match op {
      R(_) => {
        self.emitb(0xff);
        self.emit_modrm(_Operation(6), op);
      },
      M(_, _) => {
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

  fn popl(&mut self, op: Operand) {
    match op {
      R(_) => {
        self.emitb(0x58 | op.val());
      },
      M(_, _) => {
        self.emitb(0x8f);
        self.emit_modrm(_Operation(0), op);
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
