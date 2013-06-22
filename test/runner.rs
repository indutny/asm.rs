use masm::*;
use masm::x64::*;
use common::*;

#[path="../src/masm.rs"]
mod masm;
mod common;

fn run_test(arg: uint, expected: uint, test: &fn(m: &mut Masm)) {
  let mut m = ~Masm::new();
  test(m);

  assert!(m.execute(arg) == expected);
}

#[test]
fn in_and_out() {
  do run_test(0x10ff, 0x1104) |m| {
    m.pushq(R(rbp));
    m.movq(R(rbp), R(rsp));

    m.movq(R(rax), R(rsi));
    m.addq(R(rax), Byte(5));

    m.movq(R(rsp), R(rbp));
    m.popq(R(rbp));
    m.ret(Empty);
  };
}
