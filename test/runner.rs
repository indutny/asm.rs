use asm::*;
use asm::x64::*;
use common::*;

#[path="../src/asm.rs"]
mod asm;
mod common;

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

#[test]
fn branching() {
  do run_test(100, 400) |m| {
    m.pushq(R(rbp));
    m.movq(R(rbp), R(rsp));

    // Initialize output
    m.movq(R(rax), Long(0));
    m.movq(R(rcx), R(rsi));

    let mut loop_start = Label::new();
    let mut done = Label::new();

    // Loop start
    m.bind(&mut loop_start);
    m.cmpq(R(rcx), Long(0));
    m.jccl(IfEqual, &mut done);

    // Loop body
    m.addq(R(rax), Byte(4));
    m.decq(R(rcx));

    // Loop end
    m.jmpl(&mut loop_start);
    m.bind(&mut done);

    m.movq(R(rsp), R(rbp));
    m.popq(R(rbp));
    m.ret(Empty);
  };
}

#[test]
fn proc() {
  do run_test(0, 123) |m| {
    let mut proc = Label::new();

    m.pushq(R(rbp));
    m.movq(R(rbp), R(rsp));

    m.movq_proc(R(rax), &mut proc);
    m.callq(R(rax));

    m.movq(R(rsp), R(rbp));
    m.popq(R(rbp));
    m.ret(Empty);

    m.int3();

    // Subproc
    m.bind(&mut proc);
    m.pushq(R(rbp));
    m.movq(R(rbp), R(rsp));

    m.movq(R(rax), Long(123));

    m.movq(R(rsp), R(rbp));
    m.popq(R(rbp));
    m.ret(Empty);
  };
}

#[test]
fn rex() {
  do run_test(0x1234, 0x1234) |m| {
    m.pushq(R(rbp));
    m.movq(R(rbp), R(rsp));

    // Save registers
    m.pushq(R(r15));
    m.pushq(R(r14));
    m.pushq(R(r13));
    m.pushq(R(rsi));

    // r15 = arg
    m.movq(R(r15), R(rsi));

    // r14 = 0x00ff
    m.movq(R(r14), Long(0x00ff));
    // r13 = 0xff00
    m.movq(R(r13), Long(0xff00));
    // r15 &= r14
    m.andq(R(r15), R(r14));
    // rsi &= r13
    m.andq(R(rsi), R(r13));
    // r15 += rsi
    m.addq(R(r15), R(rsi));

    // r15 <-> rax
    m.xchgq(R(rax), R(r15));

    // Restore registers
    m.popq(R(rsi));
    m.popq(R(r13));
    m.popq(R(r14));
    m.popq(R(r15));

    m.movq(R(rsp), R(rbp));
    m.popq(R(rbp));
    m.ret(Empty);
  }
}

#[test]
fn fp() {
  do run_test(13589, 5959) |m| {
    m.pushq(R(rbp));
    m.movq(R(rbp), R(rsp));

    // x = arg
    m.movq(R(rax), R(rsi));
    m.cvtsi2sd(D(xmm1), R(rax));

    // x /= 23
    m.movq(R(rax), Long(23));
    m.cvtsi2sd(D(xmm2), R(rax));
    m.divsd(D(xmm1), D(xmm2));

    // x += 5
    m.movq(R(rax), Long(5));
    m.cvtsi2sd(D(xmm2), R(rax));
    m.addsd(D(xmm1), D(xmm2));

    // x *= 10
    m.movq(R(rax), Long(10));
    m.cvtsi2sd(D(xmm2), R(rax));
    m.mulsd(D(xmm1), D(xmm2));

    // x = ceil(x)
    m.roundsd(D(xmm1), D(xmm1), RoundUp);
    m.cvtsd2si(R(rax), D(xmm1));

    m.movq(R(rsp), R(rbp));
    m.popq(R(rbp));
    m.ret(Empty);
  }
}
