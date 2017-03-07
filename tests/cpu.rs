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

    let mut vm = VM::new(interconnect);

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

    // ADD A, C
    let (cpu, _, cycles) = run_cpu_test(&[0xc6, 0x02], 1);

    assert_eq!(cycles, 8);
    assert_eq!(cpu.a, 0x02);
    assert_eq!(cpu.f.z, false);
    assert_eq!(cpu.f.n, false);
    assert_eq!(cpu.f.h, false);
    assert_eq!(cpu.f.c, false);
    assert_eq!(cpu.pc, 0x100 + 2);
}
