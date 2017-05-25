extern crate gameboy;

mod common;

use self::gameboy::config::model::Model;

#[test]
#[ignore]
fn hblank_ly_scx_timing_gs() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/gpu/hblank_ly_scx_timing-GS.gb",
    );
}

#[test]
#[ignore]
fn intr_1_2_timing_gs() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/gpu/intr_1_2_timing-GS.gb",
        Model::Dmg,
    );
}

#[test]
#[ignore]
fn intr_2_0_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/gpu/intr_2_0_timing.gb",
    );
}

#[test]
#[ignore]
fn intr_2_mode0_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/gpu/intr_2_mode0_timing.gb",
    );
}

#[test]
#[ignore]
fn intr_2_mode0_timing_sprites() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/gpu/intr_2_mode0_timing_sprites.gb",
    );
}

#[test]
#[ignore]
fn intr_2_mode3_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/gpu/intr_2_mode3_timing.gb",
    );
}

#[test]
#[ignore]
fn intr_2_oam_ok_timing() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/gpu/intr_2_oam_ok_timing.gb",
    );
}

#[test]
#[ignore]
fn stat_irq_blocking() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/gpu/stat_irq_blocking.gb",
    );
}

#[test]
#[ignore]
fn vblank_stat_intr_gs() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/gpu/vblank_stat_intr-GS.gb",
        Model::Dmg,
    );
}
