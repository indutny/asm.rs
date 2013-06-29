use asm::*;
use asm::ia32::*;
use common::*;

#[test]
#[cfg(target_arch = "x86")]
fn in_and_out() {
  do run_test(13589, 13589) |m| {
    m.pushl(R(ebp));
    m.movl(R(ebp), R(esp));

    m.movl(R(eax), R(esi));

    m.movl(R(esp), R(ebp));
    m.popl(R(ebp));
    m.ret(Empty);
  };
}

#[test]
#[cfg(target_arch = "x86")]
fn math() {
  do run_test(13589, 40789) |m| {
    m.pushl(R(ebp));
    m.movl(R(ebp), R(esp));

    // Reserve some space on stack
    m.subl(R(esp), Byte(8));

    // And work with it
    m.movl(M(ebp, -8), R(esi));
    m.addl(M(ebp, -8), Long(5));
    m.movl(R(ebx), M(ebp, -8));
    m.addl(R(ebx), M(ebp, -8));
    m.addl(R(ebx), Byte(7));
    m.movl(R(eax), R(ebx));
    m.addl(R(eax), M(ebp, -8));

    m.movl(R(esp), R(ebp));
    m.popl(R(ebp));
    m.ret(Empty);
  };
}

#[test]
#[cfg(target_arch = "x86")]
fn branching() {
  do run_test(100, 400) |m| {
    m.pushl(R(ebp));
    m.movl(R(ebp), R(esp));

    // Initialize output
    m.movl(R(eax), Long(0));
    m.movl(R(ecx), R(esi));

    let mut loop_start = Label::new();
    let mut done = Label::new();

    // Loop start
    m.bind(&mut loop_start);
    m.cmpl(R(ecx), Long(0));
    m.jccl(IfEqual, &mut done);

    // Loop body
    m.addl(R(eax), Byte(4));
    m.decl(R(ecx));

    // Loop end
    m.jmpl(&mut loop_start);
    m.bind(&mut done);

    m.movl(R(esp), R(ebp));
    m.popl(R(ebp));
    m.ret(Empty);
  };
}

#[test]
#[cfg(target_arch = "x86")]
fn proc() {
  do run_test(0, 123) |m| {
    let mut proc = Label::new();

    m.pushl(R(ebp));
    m.movl(R(ebp), R(esp));

    m.movl_proc(R(eax), &mut proc);
    m.call(R(eax));

    m.movl(R(esp), R(ebp));
    m.popl(R(ebp));
    m.ret(Empty);

    m.int3();

    // Subproc
    m.bind(&mut proc);
    m.pushl(R(ebp));
    m.movl(R(ebp), R(esp));

    m.movl(R(eax), Long(123));

    m.movl(R(esp), R(ebp));
    m.popl(R(ebp));
    m.ret(Empty);
  };
}

#[test]
#[cfg(target_arch = "x86")]
fn fp() {
  do run_test(13589, 5959) |m| {
    m.pushl(R(ebp));
    m.movl(R(ebp), R(esp));

    // x = arg
    m.movl(R(eax), R(esi));
    m.cvtsi2sd(D(xmm1), R(eax));

    // x /= 23
    m.movl(R(eax), Long(23));
    m.cvtsi2sd(D(xmm2), R(eax));
    m.divsd(D(xmm1), D(xmm2));

    // x += 5
    m.movl(R(eax), Long(5));
    m.cvtsi2sd(D(xmm2), R(eax));
    m.addsd(D(xmm1), D(xmm2));

    // x *= 10
    m.movl(R(eax), Long(10));
    m.cvtsi2sd(D(xmm2), R(eax));
    m.mulsd(D(xmm1), D(xmm2));

    // x = ceil(x)
    m.roundsd(D(xmm1), D(xmm1), RoundUp);
    m.cvtsd2si(R(eax), D(xmm1));

    m.movl(R(esp), R(ebp));
    m.popl(R(ebp));
    m.ret(Empty);
  }
}
