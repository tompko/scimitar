mod common;

#[test]
fn cpu_instrs() {
    common::run_test_with_hash("tests/blargg/cpu_instrs.gb", 0xd9fc572a);
}

#[test]
fn instr_timing() {
    common::run_test_with_hash("tests/blargg/instr_timing.gb", 0xb376297f);
}
