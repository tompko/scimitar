mod common;

#[test]
fn mem_oam() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/bits/mem_oam.gb",
    );
}

#[test]
fn reg_f() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/bits/reg_f.gb",
    );
}

#[test]
fn unused_hwio_gs() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/bits/unused_hwio-GS.gb",
    );
}
