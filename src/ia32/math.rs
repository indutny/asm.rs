use asm::*;
use asm::ia32::base::*;

pub trait AsmIA32Math {
  fn incl(&mut self, dst: Operand);
  fn decl(&mut self, dst: Operand);
  fn addl(&mut self, dst: Operand, src: Operand);
  fn subl(&mut self, dst: Operand, src: Operand);
  fn divl(&mut self, src: Operand);
  fn mull(&mut self, src: Operand);
  fn idivl(&mut self, src: Operand);
  fn imull(&mut self, src: Operand);
  fn shll(&mut self, dst: Operand, src: Operand);
  fn shrl(&mut self, dst: Operand, src: Operand);
  fn sarl(&mut self, dst: Operand, src: Operand);
  fn andl(&mut self, dst: Operand, src: Operand);
  fn orl(&mut self, dst: Operand, src: Operand);
  fn xorl(&mut self, dst: Operand, src: Operand);
}

impl<A: AsmBuffer+AsmIA32Helper> AsmIA32Math for A {
  fn incl(&mut self, dst: Operand) {
    self.emitb(0xff);
    self.emit_modrm(_Operation(0), dst);
  }

  fn decl(&mut self, dst: Operand) {
    self.emitb(0xff);
    self.emit_modrm(_Operation(1), dst);
  }

  fn addl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(eax), Long(l)) => {
        self.emitb(0x05);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emitb(0x83);
        self.emit_modrm(_Operation(0), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emitb(0x81);
        self.emit_modrm(_Operation(0), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emitb(0x03);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emitb(0x01);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn subl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(eax), Long(l)) => {
        self.emitb(0x2d);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emitb(0x83);
        self.emit_modrm(_Operation(5), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emitb(0x81);
        self.emit_modrm(_Operation(5), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emitb(0x2b);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emitb(0x29);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn divl(&mut self, src: Operand) {
    assert!(src.is_rm());
    self.emitb(0xf7);
    self.emit_modrm(_Operation(6), src);
  }

  fn mull(&mut self, src: Operand) {
    assert!(src.is_rm());
    self.emitb(0xf7);
    self.emit_modrm(_Operation(4), src);
  }

  fn idivl(&mut self, src: Operand) {
    assert!(src.is_rm());
    self.emitb(0xf7);
    self.emit_modrm(_Operation(7), src);
  }

  fn imull(&mut self, src: Operand) {
    assert!(src.is_rm());
    self.emitb(0xf7);
    self.emit_modrm(_Operation(5), src);
  }

  fn shll(&mut self, dst: Operand, src: Operand) {
    assert!(dst.is_rm());
    match src {
      Byte(b) => {
        self.emitb(0xc1);
        self.emit_modrm(_Operation(4), dst);
        self.emitb(b);
      },
      _ => fail!()
    }
  }

  fn shrl(&mut self, dst: Operand, src: Operand) {
    assert!(dst.is_rm());
    match src {
      Byte(b) => {
        self.emitb(0xc1);
        self.emit_modrm(_Operation(5), dst);
        self.emitb(b);
      },
      _ => fail!()
    }
  }

  fn sarl(&mut self, dst: Operand, src: Operand) {
    assert!(dst.is_rm());
    match src {
      Byte(b) => {
        self.emitb(0xc1);
        self.emit_modrm(_Operation(7), dst);
        self.emitb(b);
      },
      _ => fail!()
    }
  }

  fn andl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(eax), Long(l)) => {
        self.emitb(0x25);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emitb(0x83);
        self.emit_modrm(_Operation(4), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emitb(0x81);
        self.emit_modrm(_Operation(4), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emitb(0x23);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emitb(0x21);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn orl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(eax), Long(l)) => {
        self.emitb(0x0d);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emitb(0x83);
        self.emit_modrm(_Operation(1), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emitb(0x81);
        self.emit_modrm(_Operation(1), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emitb(0x0b);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emitb(0x0a);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }

  fn xorl(&mut self, dst: Operand, src: Operand) {
    match (dst, src) {
      (R(eax), Long(l)) => {
        self.emitb(0x35);
        self.emitl(l);
      },
      (_, Byte(b)) if dst.is_rm() => {
        self.emitb(0x83);
        self.emit_modrm(_Operation(6), dst);
        self.emitb(b);
      },
      (_, Long(l)) if dst.is_rm() => {
        self.emitb(0x81);
        self.emit_modrm(_Operation(6), dst);
        self.emitl(l);
      },
      (R(_), _) if src.is_rm() => {
        self.emitb(0x33);
        self.emit_modrm(dst, src);
      },
      (_, R(_)) if dst.is_rm() => {
        self.emitb(0x31);
        self.emit_modrm(src, dst);
      },
      _ => fail!()
    }
  }
}
