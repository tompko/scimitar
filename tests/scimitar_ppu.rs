extern crate gameboy;

mod common;

use self::gameboy::config::model::Model;

#[test]
fn lcdoff_stat_mode() {
    common::run_test_till_ed(
        "hwtests/build/ppu/lcdoff_stat_mode.gb",
        Model::Mgb,
    );
}

