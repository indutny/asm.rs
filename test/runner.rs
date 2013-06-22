use asm::*;
use asm::x64::*;
use common::*;

#[path="../src/asm.rs"]
mod asm;
mod common;

fn run_test(arg: uint, expected: uint, test: &fn(m: &mut Asm)) {
  let mut m = ~Asm::new();
  test(m);

  assert!(m.execute(arg) == expected);
}

#[test]
fn in_and_out() {
  do run_test(13589, 13589) |m| {
    m.pushq(R(rbp));
    m.movq(R(rbp), R(rsp));

    m.movq(R(rax), R(rsi));

    m.movq(R(rsp), R(rbp));
    m.popq(R(rbp));
    m.ret(Empty);
  };
}

#[test]
fn math() {
  do run_test(13589, 40789) |m| {
    m.pushq(R(rbp));
    m.movq(R(rbp), R(rsp));

    // Reserve some space on stack
    m.subq(R(rsp), Byte(8));

    // And work with it
    m.movq(M(rbp, -8), R(rsi));
    m.addq(M(rbp, -8), Long(5));
    m.movq(R(rbx), M(rbp, -8));
    m.addq(R(rbx), M(rbp, -8));
    m.addq(R(rbx), Byte(7));
    m.movq(R(rax), R(rbx));
    m.addq(R(rax), M(rbp, -8));

    m.movq(R(rsp), R(rbp));
    m.popq(R(rbp));
    m.ret(Empty);
  };
}
