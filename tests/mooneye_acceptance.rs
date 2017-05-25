extern crate gameboy;

mod common;

use self::gameboy::config::model::Model;

#[test]
#[ignore]
fn add_sp_e_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/add_sp_e_timing.gb",
    );
}

#[test]
fn boot_regs_dmg0() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/boot_regs-dmg0.gb",
        Model::Dmg0
    );
}

#[test]
#[ignore]
fn boot_hwio_dmg0() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/boot_hwio-dmg0.gb",
        Model::Dmg0,
    );
}

#[test]
fn boot_regs_dmg_abcx() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/boot_regs-dmgABCX.gb",
        Model::Dmg,
    );
}

#[test]
fn boot_regs_mgb() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/boot_regs-mgb.gb",
        Model::Mgb,
    );
}

#[test]
#[ignore]
fn boot_hwio_dmg_abcxmgb() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/boot_hwio-dmgABCXmgb.gb",
        Model::Dmg,
    );
}

#[test]
fn boot_regs_sgb() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/boot_regs-sgb.gb",
        Model::Sgb,
    );
}

#[test]
fn boot_regs_sgb2() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/boot_regs-sgb2.gb",
        Model::Sgb2,
    );
}

#[test]
#[ignore]
fn boot_hwio_s() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/boot_hwio-S.gb",
        Model::Sgb,
    );
    common::run_test_till_ed(
        "tests/mooneye/acceptance/boot_hwio-S.gb",
        Model::Sgb2,
    );
}

#[test]
#[ignore]
fn call_cc_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/call_cc_timing.gb",
    );
}

#[test]
#[ignore]
fn call_cc_timing2() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/call_cc_timing2.gb",
    );
}

#[test]
#[ignore]
fn call_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/call_timing.gb",
    );
}

#[test]
#[ignore]
fn call_timing2() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/call_timing2.gb",
    );
}

#[test]
#[ignore]
fn di_timing_gs() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/di_timing-GS.gb",
        Model::Dmg
    );
}

#[test]
#[ignore]
fn div_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/div_timing.gb",
    );
}

#[test]
#[ignore]
fn ei_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/ei_timing.gb",
    );
}

#[test]
#[ignore]
fn halt_ime0_ei() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/halt_ime0_ei.gb",
    );
}

#[test]
#[ignore]
fn halt_ime0_nointr_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/halt_ime0_nointr_timing.gb",
    );
}

#[test]
#[ignore]
fn halt_ime1_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/halt_ime1_timing.gb",
    );
}

#[test]
#[ignore]
fn halt_ime1_timing2_gs() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/halt_ime1_timing2-GS.gb",
        Model::Dmg
    );
}

#[test]
#[ignore]
fn if_ie_registers() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/if_ie_registers.gb",
    );
}

#[test]
#[ignore]
fn intr_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/intr_timing.gb",
    );
}

#[test]
#[ignore]
fn jp_cc_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/jp_cc_timing.gb",
    );
}

#[test]
#[ignore]
fn jp_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/jp_timing.gb",
    );
}

#[test]
#[ignore]
fn ld_hl_sp_e_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/ld_hl_sp_e_timing.gb",
    );
}

#[test]
#[ignore]
fn oam_dma_restart() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/oam_dma_restart.gb",
    );
}

#[test]
#[ignore]
fn oam_dma_start() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/oam_dma_start.gb",
    );
}

#[test]
#[ignore]
fn oam_dma_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/oam_dma_timing.gb",
    );
}

#[test]
#[ignore]
fn pop_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/pop_timing.gb",
    );
}

#[test]
#[ignore]
fn push_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/push_timing.gb",
    );
}

#[test]
#[ignore]
fn rapid_di_ei() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/rapid_di_ei.gb",
    );
}

#[test]
#[ignore]
fn ret_cc_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/ret_cc_timing.gb",
    );
}

#[test]
#[ignore]
fn reti_intr_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/reti_intr_timing.gb",
    );
}

#[test]
#[ignore]
fn reti_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/reti_timing.gb",
    );
}

#[test]
#[ignore]
fn ret_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/ret_timing.gb",
    );
}

#[test]
#[ignore]
fn rst_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/rst_timing.gb",
    );
}
