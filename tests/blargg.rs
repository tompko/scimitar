extern crate gameboy;

mod common;

use self::gameboy::config::model::Model;

#[test]
fn cpu_instrs() {
    common::run_test_with_hash(
        "tests/blargg/cpu_instrs.gb",
        Model::Dmg,
        0xd9fc572a,
    );
}

#[test]
fn instr_timing() {
    common::run_test_with_hash(
        "tests/blargg/instr_timing.gb",
        Model::Dmg,
        0xb376297f,
    );
}

#[test]
fn mem_timing() {
    common::run_test_with_hash(
        "tests/blargg/mem_timing_2.gb",
        Model::Dmg,
        0xd373b16f,
    );
}
