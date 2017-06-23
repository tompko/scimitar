extern crate gameboy;

mod common;

use self::gameboy::config::model::Model;

#[test]
#[ignore]
fn boot_sclk_align_dmgabcx_mgb() {
    common::run_test_till_ed(
        "tests/mooneye/acceptance/serial/boot_sclk_align-dmgABCXmgb.gb",
        Model::Dmg,
    );
    common::run_test_till_ed(
        "tests/mooneye/acceptance/serial/boot_sclk_align-dmgABCXmgb.gb",
        Model::Mgb,
    );
}

