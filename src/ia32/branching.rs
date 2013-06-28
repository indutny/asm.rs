use asm::*;
use asm::ia32::base::*;

pub trait AsmIA32Branching {
  fn testl(&mut self, dst: Operand, src: Operand);
  fn cmpl(&mut self, dst: Operand, src: Operand);
  fn jmp(&mut self, target: Operand);
  fn jmpl(&mut self, l: &mut Label);
  fn jccl(&mut self, c: JumpCondition, l: &mut Label);
  fn call(&mut self, target: Operand);
}

impl<A: AsmBuffer+AsmIA32Helper> AsmIA32Branching for A {
  fn testl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(eax), Long(l)) => {
        self.emitb(0xa9);
        self.emitl(l);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emitb(0xf7);
        self.emit_modrm(_Operation(0), dst);
        self.emitl(l);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emitb(0x85);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn cmpl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(eax), Long(l)) => {
        self.emitb(0x3d);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emitb(0x83);
        self.emit_modrm(_Operation(7), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emitb(0x81);
        self.emit_modrm(_Operation(7), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emitb(0x3b);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emitb(0x39);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn jmp(&mut self, target: Operand) {
    self.emitb(0xff);
    self.emit_modrm(Empty, target);
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

  fn call(&mut self, target: Operand) {
    self.emitb(0xff);
    self.emit_modrm(_Operation(2), target);
  }
}
