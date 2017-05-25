mod common;

#[test]
fn div_write() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/div_write.gb",
    );
}

#[test]
#[ignore]
fn rapid_toggle() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/rapid_toggle.gb",
    );
}

#[test]
fn tim00_div_trigger() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tim00_div_trigger.gb",
    );
}

#[test]
fn tim00() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tim00.gb",
    );
}

#[test]
fn tim01_div_trigger() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tim01_div_trigger.gb",
    );
}

#[test]
fn tim01() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tim01.gb",
    );
}

#[test]
fn tim10_div_trigger() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tim10_div_trigger.gb",
    );
}

#[test]
fn tim10() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tim10.gb",
    );
}

#[test]
fn tim11_div_trigger() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tim11_div_trigger.gb",
    );
}

#[test]
fn tim11() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tim11.gb",
    );
}

#[test]
fn tima_reload() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tima_reload.gb",
    );
}

#[test]
#[ignore]
fn tima_write_reloading() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tima_write_reloading.gb",
    );
}

#[test]
#[ignore]
fn tma_write_reloading() {
    common::run_all_models_till_ed(
        "tests/mooneye/acceptance/timer/tma_write_reloading.gb",
    );
}
