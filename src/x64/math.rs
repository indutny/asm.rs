use asm::*;
use asm::x64::base::*;

pub trait AsmX64Math {
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
}

impl<A: AsmBuffer+AsmX64Helper> AsmX64Math for A {
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
}
