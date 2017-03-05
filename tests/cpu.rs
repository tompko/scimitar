extern crate gameboy;

use gameboy::cpu::Cpu;
use gameboy::vm::VM;
use gameboy::interconnect::MockInterconnect;

fn run_cpu_test(instrs: &[u8], num_steps: usize) -> (Cpu, u16) {
    let mut cart_bytes = vec![0; 256 + instrs.len()];
    for (i, inst) in instrs.iter().enumerate() {
        cart_bytes[i + 256] = *inst;
    }
    let mut interconnect = MockInterconnect::new(&cart_bytes);

    for i in 0..10 {
        interconnect.set_mem(0xc000 + (i as u16), 0x50 + i as u8);
    }

    let mut vm = VM::new(interconnect);

    let mut cycles = 0;
    for _ in 0..num_steps {
        cycles += vm.step();
    }

    (vm.get_cpu(), cycles)
}

#[test]
fn cpu_ld_nn_n() {
    // LC B, n
    let (cpu, cycles) = run_cpu_test(&[0x06, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LC C, n
    let (cpu, cycles) = run_cpu_test(&[0x0E, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LC D, n
    let (cpu, cycles) = run_cpu_test(&[0x16, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LC E, n
    let (cpu, cycles) = run_cpu_test(&[0x1E, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.e, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LC H, n
    let (cpu, cycles) = run_cpu_test(&[0x26, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LC L, n
    let (cpu, cycles) = run_cpu_test(&[0x2E, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.l, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);
}

#[test]
#[ignore]
fn cpu_ld_r1_r2() {
    // TODO -requires LD A,n
}

#[test]
fn cpu_ld_a_n() {
    // LD A, A
    let (cpu, cycles) = run_cpu_test(&[0x7f], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0x100 + 1);

    // LD A, B
    let (cpu, cycles) = run_cpu_test(&[0x06, 0x42, 0x78], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, cycles) = run_cpu_test(&[0x0e, 0x42, 0x79], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, cycles) = run_cpu_test(&[0x16, 0x42, 0x7a], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, cycles) = run_cpu_test(&[0x1e, 0x42, 0x7b], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, cycles) = run_cpu_test(&[0x26, 0x42, 0x7c], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, cycles) = run_cpu_test(&[0x2e, 0x42, 0x7d], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, cycles) = run_cpu_test(&[0x06, 0xc0, 0x0e, 0x00, 0x0a], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    let (cpu, cycles) = run_cpu_test(&[0x16, 0xc0, 0x1e, 0x00, 0x1a], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    let (cpu, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x00, 0x7e], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    let (cpu, cycles) = run_cpu_test(&[0xfa, 0x00, 0xc0], 1);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, cycles) = run_cpu_test(&[0x3e, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

}
