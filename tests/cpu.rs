extern crate gameboy;

use gameboy::cpu::Cpu;
use gameboy::vm::VM;
use gameboy::interconnect::{Interconnect, MockInterconnect};

fn run_cpu_test(instrs: &[u8], num_steps: usize) -> (Cpu, MockInterconnect, u16) {
    let mut cart_bytes = vec![0; 256 + instrs.len()];
    for (i, inst) in instrs.iter().enumerate() {
        cart_bytes[i + 256] = *inst;
    }
    let mut interconnect = MockInterconnect::new(&cart_bytes);

    for i in 0..10 {
        interconnect.set_mem(0xc000 + (i as u16), 0x50 + i as u8);
    }

    let mut vm = VM::new(interconnect, false);

    let mut cycles = 0;
    for _ in 0..num_steps {
        cycles += vm.step();
    }

    let (cpu, inter) = vm.get_children();
    (cpu, inter, cycles)
}

#[test]
fn cpu_ld_a_n() {
    // LD A, A
    let (cpu, _, cycles) = run_cpu_test(&[0x7f], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0x100 + 1);

    // LD A, B
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x42, 0x78], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x42, 0x79], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x42, 0x7a], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x42, 0x7b], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x42, 0x7c], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x42, 0x7d], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0xc0, 0x0e, 0x00, 0x0a], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0xc0, 0x1e, 0x00, 0x1a], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x00, 0x7e], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    let (cpu, _, cycles) = run_cpu_test(&[0xfa, 0x00, 0xc0], 1);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x50);
    assert_eq!(cpu.pc, 0x100 + 3);

    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);
}

#[test]
fn cpu_ld_b_n() {
    // LC B, n
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LD B, B
    let (cpu, _, cycles) = run_cpu_test(&[0x40], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0x100 + 1);

    // LD B, C
    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x42, 0x41], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD B, D
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x42, 0x42], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD B, E
    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x42, 0x43], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD B, H
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x42, 0x44], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD B, L
    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x42, 0x45], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD B, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x01, 0x46], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.b, 0x51);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD B, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x42, 0x47], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);
}

#[test]
fn cpu_ld_c_n() {
    // LC C, n
    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LD C, B
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x42, 0x48], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD C, C
    let (cpu, _, cycles) = run_cpu_test(&[0x49], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0x100 + 1);

    // LD C, D
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x42, 0x4a], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD C, E
    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x42, 0x4b], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD C, H
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x42, 0x4c], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD C, L
    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x42, 0x4d], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD C, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x02, 0x4e], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.c, 0x52);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD C, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x42, 0x4f], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);
}

#[test]
fn cpu_ld_d_n() {
    // LC D, n
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LD D, B
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x42, 0x50], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD D, C
    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x42, 0x51], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD D, D
    let (cpu, _, cycles) = run_cpu_test(&[0x52], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0x100 + 1);

    // LD D, E
    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x42, 0x53], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD D, H
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x42, 0x54], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD D, L
    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x42, 0x55], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD D, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x00, 0x56], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.d, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD D, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x42, 0x57], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);
}

#[test]
fn cpu_ld_e_n() {
    // LC E, n
    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.e, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LD E, B
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x42, 0x58], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.e, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD E, C
    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x42, 0x59], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.e, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD E, D
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x42, 0x5a], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.e, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD E, E
    let (cpu, _, cycles) = run_cpu_test(&[0x5b], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0x100 + 1);

    // LD E, H
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x42, 0x5c], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.e, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD E, L
    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x42, 0x5d], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.e, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD E, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x00, 0x5e], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.e, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD E, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x42, 0x5f], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.e, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);
}

#[test]
fn cpu_ld_h_n() {
    // LC H, n
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LD H, B
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x42, 0x60], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD H, C
    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x42, 0x61], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD H, D
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x42, 0x62], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD H, E
    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x42, 0x63], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD H, H
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x42, 0x64], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD H, L
    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x42, 0x65], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD H, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x00, 0x66], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.h, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD H, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x42, 0x67], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);
}

#[test]
fn cpu_ld_l_n() {
    // LC L, n
    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x42], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.l, 0x42);
    assert_eq!(cpu.pc, 0x100 + 2);

    // LD L, B
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x42, 0x68], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.l, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD L, C
    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x42, 0x69], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.l, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD L, D
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x42, 0x6a], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.l, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD L, E
    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x42, 0x6b], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.l, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD L, H
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x42, 0x6c], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.l, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD L, L
    let (cpu, _, cycles) = run_cpu_test(&[0x6d], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.pc, 0x100 + 1);

    // LD L, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x00, 0x6e], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.l, 0x50);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD L, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x42, 0x6f], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.l, 0x42);
    assert_eq!(cpu.pc, 0x100 + 3);
}

#[test]
fn cpu_ld_nn_a() {
    // LD (BC), A
    let (cpu, inter, cycles) = run_cpu_test(&[0x06, 0xd0, 0x0e, 0x01, 0x3e, 0x42, 0x02], 4);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xd001), 0x42);
    assert_eq!(cpu.pc, 0x100 + 7);

    // LD (DE), A
    let (cpu, inter, cycles) = run_cpu_test(&[0x16, 0xd0, 0x1e, 0x01, 0x3e, 0x42, 0x12], 4);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xd001), 0x42);
    assert_eq!(cpu.pc, 0x100 + 7);

    // LD (HL), A
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xd0, 0x2e, 0x01, 0x3e, 0x42, 0x77], 4);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xd001), 0x42);
    assert_eq!(cpu.pc, 0x100 + 7);

    // LD (nn), A
    let (cpu, inter, cycles) = run_cpu_test(&[0x3e, 0x42, 0xea, 0x01, 0xd0], 2);

    assert_eq!(cycles, 24);
    assert_eq!(inter.read_byte(0xd001), 0x42);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD A, (C)
    let (cpu, inter, cycles) = run_cpu_test(&[0x3e, 0x42, 0x0e, 0x01, 0xe2], 3);

    assert_eq!(cycles, 24);
    assert_eq!(inter.read_byte(0xff01), 0x42);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD (C), A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x42, 0x0e, 0x01, 0xe2, 0x3e, 0x00, 0xf2], 5);

    assert_eq!(cycles, 40);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 8);

    // LDD A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x02, 0x3a], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x52);
    assert_eq!(cpu.pc, 0x100 + 5);
    assert_eq!(cpu.hl(), 0xc001);

    // LDD (HL), A
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x02, 0x3e, 0x42, 0x32], 4);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xc002), 0x42);
    assert_eq!(cpu.hl(), 0xc001);
    assert_eq!(cpu.pc, 0x100 + 7);

    // LDI A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x02, 0x2a], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x52);
    assert_eq!(cpu.pc, 0x100 + 5);
    assert_eq!(cpu.hl(), 0xc003);

    // LDI (HL), A
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x02, 0x3e, 0x42, 0x22], 4);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xc002), 0x42);
    assert_eq!(cpu.hl(), 0xc003);
    assert_eq!(cpu.pc, 0x100 + 7);

    // LD A, (n)
    let (cpu, inter, cycles) = run_cpu_test(&[0x3e, 0x42, 0xe0, 0x01], 2);

    assert_eq!(cycles, 20);
    assert_eq!(inter.read_byte(0xff01), 0x42);
    assert_eq!(cpu.pc, 0x100 + 4);

    // LD (n), A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x42, 0xe0, 0x01, 0x3e, 0x00, 0xf0, 0x01], 4);

    assert_eq!(cycles, 40);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.pc, 0x100 + 8);
}

#[test]
fn cpu_ld_hl_n() {
    // LD (HL), B
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xd0, 0x2e, 0x00, 0x06, 0x42, 0x70], 4);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xd000), 0x42);
    assert_eq!(cpu.pc, 0x100 + 7);

    // LD (HL), C
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xd0, 0x2e, 0x00, 0x0e, 0x42, 0x71], 4);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xd000), 0x42);
    assert_eq!(cpu.pc, 0x100 + 7);

    // LD (HL), D
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xd0, 0x2e, 0x00, 0x16, 0x42, 0x72], 4);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xd000), 0x42);
    assert_eq!(cpu.pc, 0x100 + 7);

    // LD (HL), E
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xd0, 0x2e, 0x00, 0x1e, 0x42, 0x73], 4);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xd000), 0x42);
    assert_eq!(cpu.pc, 0x100 + 7);

    // LD (HL), H
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xd0, 0x2e, 0x00, 0x74], 3);

    assert_eq!(cycles, 24);
    assert_eq!(inter.read_byte(0xd000), 0xd0);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD (HL), L
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xd0, 0x2e, 0x01, 0x75], 3);

    assert_eq!(cycles, 24);
    assert_eq!(inter.read_byte(0xd001), 0x01);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD (HL), n
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xd0, 0x2e, 0x00, 0x36, 0x42], 3);

    assert_eq!(cycles, 28);
    assert_eq!(inter.read_byte(0xd000), 0x42);
    assert_eq!(cpu.pc, 0x100 + 6);
}

#[test]
fn cpu_ld_n_nn() {
    // LD BC, nn
    let (cpu, _, cycles) = run_cpu_test(&[0x01, 0x31, 0x45], 1);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.b, 0x45);
    assert_eq!(cpu.c, 0x31);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD DE, nn
    let (cpu, _, cycles) = run_cpu_test(&[0x11, 0x31, 0x45], 1);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.d, 0x45);
    assert_eq!(cpu.e, 0x31);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD HL, nn
    let (cpu, _, cycles) = run_cpu_test(&[0x21, 0x31, 0x45], 1);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.h, 0x45);
    assert_eq!(cpu.l, 0x31);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD SP, nn
    let (cpu, _, cycles) = run_cpu_test(&[0x31, 0x31, 0x45], 1);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.sp, 0x4531);
    assert_eq!(cpu.pc, 0x100 + 3);

    // LD SP, HL
    let (cpu, _, cycles) = run_cpu_test(&[0x21, 0x31, 0x45, 0xf9], 2);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.sp, 0x4531);
    assert_eq!(cpu.pc, 0x100 + 4);

    // LD HL, SP+n
    let (cpu, _, cycles) = run_cpu_test(&[0x31, 0x00, 0xc0, 0xf8, 0x05], 2);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.hl(), 0xc005);
    assert_eq!(cpu.pc, 0x100 + 5);

    // LD (nn), SP
    let (cpu, inter, cycles) = run_cpu_test(&[0x31, 0x34, 0x12, 0x08, 0x00, 0xd0], 2);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_halfword(0xd000), 0x1234);
    assert_eq!(cpu.pc, 0x100 + 6);
}

#[test]
fn cpu_push_pop() {
    // PUSH AF
    let (cpu, inter, cycles) = run_cpu_test(&[0x3e, 0x42, 0xf5], 2);

    assert_eq!(cycles, 24);
    assert_eq!(inter.read_halfword(0xfffe - 2), 0x4200);
    assert_eq!(cpu.pc, 0x100 + 3);

    // PUSH BC
    let (cpu, inter, cycles) = run_cpu_test(&[0x01, 0x34, 0x12, 0xc5], 2);

    assert_eq!(cycles, 28);
    assert_eq!(inter.read_halfword(0xfffe - 2), 0x1234);
    assert_eq!(cpu.pc, 0x100 + 4);

    // PUSH DE
    let (cpu, inter, cycles) = run_cpu_test(&[0x11, 0x34, 0x12, 0xd5], 2);

    assert_eq!(cycles, 28);
    assert_eq!(inter.read_halfword(0xfffe - 2), 0x1234);
    assert_eq!(cpu.pc, 0x100 + 4);

    // PUSH HL
    let (cpu, inter, cycles) = run_cpu_test(&[0x21, 0x34, 0x12, 0xe5], 2);

    assert_eq!(cycles, 28);
    assert_eq!(inter.read_halfword(0xfffe - 2), 0x1234);
    assert_eq!(cpu.pc, 0x100 + 4);

    // POP AF
    let (cpu, _, cycles) = run_cpu_test(&[0x01, 0xff, 0x12, 0xc5, 0xf1], 3);

    assert_eq!(cycles, 40);
    assert_eq!(cpu.a, 0x12);
    let val: u8 = cpu.f.into();
    assert_eq!(val, 0xf0);
    assert_eq!(cpu.pc, 0x100 + 5);

    // POP BC
    let (cpu, _, cycles) = run_cpu_test(&[0x11, 0x34, 0x12, 0xd5, 0xc1], 3);

    assert_eq!(cycles, 40);
    assert_eq!(cpu.bc(), 0x1234);
    assert_eq!(cpu.pc, 0x100 + 5);

    // POP DE
    let (cpu, _, cycles) = run_cpu_test(&[0x01, 0x34, 0x12, 0xc5, 0xd1], 3);

    assert_eq!(cycles, 40);
    assert_eq!(cpu.de(), 0x1234);
    assert_eq!(cpu.pc, 0x100 + 5);

    // POP HL
    let (cpu, _, cycles) = run_cpu_test(&[0x01, 0x34, 0x12, 0xc5, 0xe1], 3);

    assert_eq!(cycles, 40);
    assert_eq!(cpu.hl(), 0x1234);
    assert_eq!(cpu.pc, 0x100 + 5);
}

#[test]
fn cpu_add() {
    // ADD A, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x02, 0x87], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // ADD A, B
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x02, 0x80], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // ADD A, B - Overflows and Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0xff, 0x06, 0x01, 0x80], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 5);

    // ADD A, B - C Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0xf0, 0x06, 0xf0, 0x80], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0xe0);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 5);

    // ADD A, B - H Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x0f, 0x06, 0x0f, 0x80], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x1e);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // ADD A, B - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x00, 0x06, 0x00, 0x80], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // ADD A, C
    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x02, 0x81], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // ADD A, D
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x02, 0x82], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // ADD A, E
    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x02, 0x83], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // ADD A, H
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x02, 0x84], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // ADD A, L
    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x02, 0x85], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // ADD A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x05, 0x86], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x55);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // ADD A, n
    let (cpu, _, cycles) = run_cpu_test(&[0xc6, 0x02], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 2);
}

#[test]
fn cpu_addc_a() {
    // ADDC A, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x02, 0x37, 0x8f], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x05);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // ADDC A, B
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x02, 0x37, 0x88], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x03);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // ADDC A, B - Overflows and Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0xfe, 0x06, 0x01, 0x37, 0x88], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 6);

    // ADDC A, B - C Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0xf0, 0x06, 0xf0, 0x37, 0x88], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0xe1);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 6);

    // ADDC A, B - H Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x0f, 0x06, 0x0e, 0x37, 0x88], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x1e);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // ADDC A, B - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x00, 0x06, 0xff, 0x37, 0x88], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 6);

    // ADDC A, C
    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x02, 0x37, 0x89], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x03);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // ADDC A, D
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x02, 0x37, 0x8a], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x03);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // ADDC A, E
    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x02, 0x37, 0x8b], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x03);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // ADDC A, H
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x02, 0x37, 0x8c], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x03);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // ADDC A, L
    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x02, 0x37, 0x8d], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x03);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // ADDC A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x05, 0x37, 0x8e], 4);

    assert_eq!(cycles, 28);
    assert_eq!(cpu.a, 0x56);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // ADDC A, n
    let (cpu, _, cycles) = run_cpu_test(&[0x37, 0xce, 0x02], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x03);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);
}

#[test]
fn cpu_sub() {
    // SUB A, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x02, 0x97], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // SUB A, B
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x06, 0x02, 0x90], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // SUB A, B - C Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x60, 0x06, 0x70, 0x90], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0xf0);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 5);

    // SUB A, B - H Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0xf6, 0x06, 0xe7, 0x90], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x0f);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // SUB A, B - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x00, 0x06, 0x00, 0x90], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // SUB A, C
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x0e, 0x02, 0x91], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // SUB A, D
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x16, 0x02, 0x92], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // SUB A, E
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x1e, 0x02, 0x93], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // SUB A, H
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x26, 0x02, 0x94], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // SUB A, L
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x2e, 0x02, 0x95], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // SUB A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x57, 0x26, 0xc0, 0x2e, 0x05, 0x96], 4);

    assert_eq!(cycles, 32);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 7);

    // SUB A, n
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0xd6, 0x02], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);
}

#[test]
fn cpu_subc_a() {
    // SUBC A, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x02, 0x37, 0x9f], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0xff);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 4);

    // SUBC A, B
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x06, 0x02, 0x37, 0x98], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // SUBC A, B - C Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0xe1, 0x06, 0xf0, 0x37, 0x98], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0xf0);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 6);

    // SUBC A, B - H Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x7e, 0x06, 0x0e, 0x37, 0x98], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x6f);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // SUBC A, B - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x02, 0x06, 0x01, 0x37, 0x98], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // SUBC A, C
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x0e, 0x02, 0x37, 0x99], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // SUBC A, D
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x16, 0x02, 0x37, 0x9a], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // SUBC A, E
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x1e, 0x02, 0x37, 0x9b], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // SUBC A, H
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x26, 0x02, 0x37, 0x9c], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // SUBC A, L
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x2e, 0x02, 0x37, 0x9d], 4);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);

    // SUBC A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x57, 0x26, 0xc0, 0x2e, 0x05, 0x37, 0x9e], 5);

    assert_eq!(cycles, 36);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 8);

    // SUBC A, n
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x37, 0xde, 0x02], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);
}

#[test]
fn cpu_and() {
    // AND A, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0xa7], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // AND A, B
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x06, 0x05, 0xa0], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // AND A, B - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x02, 0x06, 0x01, 0xa0], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // AND A, C
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x0e, 0x05, 0xa1], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // AND A, D
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x16, 0x05, 0xa2], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // AND A, E
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x1e, 0x05, 0xa3], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // AND A, H
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x26, 0x05, 0xa4], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // AND A, L
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x2e, 0x05, 0xa5], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // AND A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x26, 0xc0, 0x2e, 0x01, 0xa6], 4);

    assert_eq!(cycles, 32);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 7);

    // AND A, n
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0xe6, 0x05], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);
}

#[test]
fn cpu_or() {
    // OR A, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0xb7], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // OR A, B
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x06, 0x05, 0xb0], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x07);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // OR A, B - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x00, 0x06, 0x00, 0xb0], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // OR A, C
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x0e, 0x05, 0xb1], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x07);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // OR A, D
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x16, 0x05, 0xb2], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x07);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // OR A, E
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x1e, 0x05, 0xb3], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x07);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // OR A, H
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x26, 0x05, 0xb4], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x07);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // OR A, L
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x2e, 0x05, 0xb5], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x07);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // OR A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x26, 0xc0, 0x2e, 0x01, 0xb6], 4);

    assert_eq!(cycles, 32);
    assert_eq!(cpu.a, 0x53);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 7);

    // OR A, n
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0xf6, 0x05], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x07);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);
}

#[test]
fn cpu_xor() {
    // XOR A, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0xaf], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // XOR A, B
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x06, 0x05, 0xa8], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x06);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // XOR A, B - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x01, 0x06, 0x01, 0xa8], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // XOR A, C
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x0e, 0x05, 0xa9], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x06);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // XOR A, D
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x16, 0x05, 0xaa], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x06);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // XOR A, E
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x1e, 0x05, 0xab], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x06);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // XOR A, H
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x26, 0x05, 0xac], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x06);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // XOR A, L
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x2e, 0x05, 0xad], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x06);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // XOR A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0x26, 0xc0, 0x2e, 0x01, 0xae], 4);

    assert_eq!(cycles, 32);
    assert_eq!(cpu.a, 0x52);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 7);

    // XOR A, n
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x03, 0xee, 0x05], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x06);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);
}

#[test]
fn cpu_cmp() {
    // CMP A, A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x02, 0xbf], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // CMP A, B
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x06, 0x02, 0xb8], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // CMP A, B - C Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x60, 0x06, 0x70, 0xb8], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x60);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 5);

    // CMP A, B - H Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0xf6, 0x06, 0xe7, 0xb8], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0xf6);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // CMP A, B - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x00, 0x06, 0x00, 0xb8], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x0);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // CMP A, C
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x0e, 0x02, 0xb9], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // CMP A, D
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x16, 0x02, 0xba], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // CMP A, E
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x1e, 0x02, 0xbb], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // CMP A, H
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x26, 0x02, 0xbc], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // CMP A, L
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0x2e, 0x02, 0xbd], 3);

    assert_eq!(cycles, 20);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // CMP A, (HL)
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x57, 0x26, 0xc0, 0x2e, 0x05, 0xbe], 4);

    assert_eq!(cycles, 32);
    assert_eq!(cpu.a, 0x57);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 7);

    // CMP A, n
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x04, 0xfe, 0x02], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x04);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 4);
}

#[test]
fn cpu_inc8() {
    // INC A
    let (cpu, _, cycles) = run_cpu_test(&[0x3c], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.a, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC A - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0xff, 0x3c], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // INC A - H Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x0f, 0x3c], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x10);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // INC B
    let (cpu, _, cycles) = run_cpu_test(&[0x04], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.b, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC C
    let (cpu, _, cycles) = run_cpu_test(&[0x0c], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.c, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC D
    let (cpu, _, cycles) = run_cpu_test(&[0x14], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.d, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC E
    let (cpu, _, cycles) = run_cpu_test(&[0x1c], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.e, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC H
    let (cpu, _, cycles) = run_cpu_test(&[0x24], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.h, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC L
    let (cpu, _, cycles) = run_cpu_test(&[0x2c], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.l, 0x01);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC (HL)
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x00, 0x34], 3);

    assert_eq!(cycles, 28);
    assert_eq!(inter.read_byte(0xc000), 0x51);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);
}

#[test]
fn cpu_dec8() {
    // DEC A
    let (cpu, _, cycles) = run_cpu_test(&[0x3d], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.a, 0xff);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC A - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x01, 0x3d], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // DEC A - H Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x10, 0x3d], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0x0f);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 3);

    // DEC B
    let (cpu, _, cycles) = run_cpu_test(&[0x05], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.b, 0xff);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC C
    let (cpu, _, cycles) = run_cpu_test(&[0x0d], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.c, 0xff);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC D
    let (cpu, _, cycles) = run_cpu_test(&[0x15], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.d, 0xff);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC E
    let (cpu, _, cycles) = run_cpu_test(&[0x1d], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.e, 0xff);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC H
    let (cpu, _, cycles) = run_cpu_test(&[0x25], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.h, 0xff);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC L
    let (cpu, _, cycles) = run_cpu_test(&[0x2d], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.l, 0xff);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC (HL)
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x00, 0x35], 3);

    assert_eq!(cycles, 28);
    assert_eq!(inter.read_byte(0xc000), 0x4f);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, true);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);
}

#[test]
fn cpu_add16() {
    // ADD HL, BC
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x12, 0x0e, 0x34, 0x09], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.hl(), 0x1234);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // ADD HL, BC - H Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x0f, 0x0e, 0xff, 0x2e, 0x01, 0x09], 4);

    assert_eq!(cycles, 32);
    assert_eq!(cpu.hl(), 0x1000);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 7);

    // ADD HL, BC - C Overflow
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0xff, 0x0e, 0xff, 0x2e, 0x01, 0x09], 4);

    assert_eq!(cycles, 32);
    assert_eq!(cpu.hl(), 0x0000);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 7);

    // ADD HL, DE
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x12, 0x1e, 0x34, 0x19], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.hl(), 0x1234);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // ADD HL, HL
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x01, 0x2e, 0x01, 0x29], 3);

    assert_eq!(cycles, 24);
    assert_eq!(cpu.hl(), 0x0202);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 5);

    // ADD HL, SP
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x01, 0x2e, 0x01, 0xf9, 0x39], 4);

    assert_eq!(cycles, 32);
    assert_eq!(cpu.hl(), 0x0202);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 6);
}

#[test]
fn cpu_add_sp() {
    // ADD SP, n
    let (cpu, _, cycles) = run_cpu_test(&[0xe8, 0x02], 1);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.sp, 0x0000);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, true);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 2);
}

#[test]
fn cpu_inc16() {
    // INC BC
    let (cpu, _, cycles) = run_cpu_test(&[0x03], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.bc(), 0x0001);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC DE
    let (cpu, _, cycles) = run_cpu_test(&[0x13], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.de(), 0x0001);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC HL
    let (cpu, _, cycles) = run_cpu_test(&[0x23], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.hl(), 0x0001);
    assert_eq!(cpu.pc, 0x100 + 1);

    // INC SP
    let (cpu, _, cycles) = run_cpu_test(&[0x33], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.sp, 0xffff);
    assert_eq!(cpu.pc, 0x100 + 1);
}

#[test]
fn cpu_dec16() {
    // DEC BC
    let (cpu, _, cycles) = run_cpu_test(&[0x0b], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.bc(), 0xffff);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC DE
    let (cpu, _, cycles) = run_cpu_test(&[0x1b], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.de(), 0xffff);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC HL
    let (cpu, _, cycles) = run_cpu_test(&[0x2b], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.hl(), 0xffff);
    assert_eq!(cpu.pc, 0x100 + 1);

    // DEC SP
    let (cpu, _, cycles) = run_cpu_test(&[0x3b], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.sp, 0xfffd);
    assert_eq!(cpu.pc, 0x100 + 1);
}

#[test]
fn cpu_swap() {
    // SWAP A
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x24, 0xcb, 0x37], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x42);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // SWAP A - Zero
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x00, 0xcb, 0x37], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.a, 0x00);
    assert_eq!(cpu.f.z, true);
    assert_eq!(cpu.pc, 0x100 + 4);

    // SWAP B
    let (cpu, _, cycles) = run_cpu_test(&[0x06, 0x24, 0xcb, 0x30], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.b, 0x42);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // SWAP C
    let (cpu, _, cycles) = run_cpu_test(&[0x0e, 0x24, 0xcb, 0x31], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.c, 0x42);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // SWAP D
    let (cpu, _, cycles) = run_cpu_test(&[0x16, 0x24, 0xcb, 0x32], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.d, 0x42);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // SWAP E
    let (cpu, _, cycles) = run_cpu_test(&[0x1e, 0x24, 0xcb, 0x33], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.e, 0x42);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // SWAP H
    let (cpu, _, cycles) = run_cpu_test(&[0x26, 0x24, 0xcb, 0x34], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.h, 0x42);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // SWAP L
    let (cpu, _, cycles) = run_cpu_test(&[0x2e, 0x24, 0xcb, 0x35], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.l, 0x42);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.pc, 0x100 + 4);

    // SWAP (HL)
    let (cpu, inter, cycles) = run_cpu_test(&[0x26, 0xc0, 0x2e, 0x09, 0xcb, 0x36], 3);

    assert_eq!(cycles, 32);
    assert_eq!(inter.read_byte(0xc009), 0x95);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.pc, 0x100 + 6);
}

#[test]
fn cpu_daa() {
    // TODO
}

#[test]
fn cpu_cpl() {
    let (cpu, _, cycles) = run_cpu_test(&[0x3e, 0x55, 0x2f], 2);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.a, 0xaa);
    assert_eq!(cpu.pc, 0x100 + 3);
}

#[test]
fn cpu_cf() {
    // SCF
    let (cpu, _, cycles) = run_cpu_test(&[0x37], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 1);

    // CCF
    let (cpu, _, cycles) = run_cpu_test(&[0x3f], 1);

    assert_eq!(cycles, 4);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, true);
    assert_eq!(cpu.pc, 0x100 + 1);

    let (cpu, _, cycles) = run_cpu_test(&[0x37, 0x3f], 2);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 2);
}

#[test]
fn cpu_jr() {
    // JR n - positive jump
    let (cpu, _, cycles) = run_cpu_test(&[0x18, 0x02], 1);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.pc, 0x100 + 4);

    // JR n - negative jump
    let (cpu, _, cycles) = run_cpu_test(&[0x18, 0xfe], 1);

    assert_eq!(cycles, 12);
    assert_eq!(cpu.pc, 0x100 + 0);
}

#[test]
fn cpu_jr_cc_n() {
    // JR NZ, n - take jump
    let (cpu, _, cycles) = run_cpu_test(&[0xc6, 0x02, 0x20, 0x02], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0x100 + 6);

    // JR NZ, n - no jump
    let (cpu, _, cycles) = run_cpu_test(&[0xc6, 0x00, 0x20, 0x02], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0x100 + 4);

    // JR Z, n - take jump
    let (cpu, _, cycles) = run_cpu_test(&[0xc6, 0x00, 0x28, 0x02], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0x100 + 6);

    // JR Z, n - no jump
    let (cpu, _, cycles) = run_cpu_test(&[0xc6, 0x02, 0x28, 0x02], 2);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0x100 + 4);

    // JR NC, n - take jump
    let (cpu, _, cycles) = run_cpu_test(&[0x37, 0x3F, 0x30, 0x02], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0x100 + 6);

    // JR NC, n - no jump
    let (cpu, _, cycles) = run_cpu_test(&[0x37, 0x00, 0x30, 0x02], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0x100 + 4);

    // JR C, n - take jump
    let (cpu, _, cycles) = run_cpu_test(&[0x37, 0x00, 0x38, 0x02], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0x100 + 6);

    // JR C, n - no jump
    let (cpu, _, cycles) = run_cpu_test(&[0x37, 0x3f, 0x38, 0x02], 3);

    assert_eq!(cycles, 16);
    assert_eq!(cpu.pc, 0x100 + 4);

}
