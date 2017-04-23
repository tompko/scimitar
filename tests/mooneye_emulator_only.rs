mod common;

#[test]
#[ignore]
fn multicart_rom_8_mb() {
    common::run_test_till_ed("tests/mooneye/emulator-only/mbc1/multicart_rom_8Mb.gb");
}

#[test]
fn ram_256_kb() {
    common::run_test_till_ed("tests/mooneye/emulator-only/mbc1/ram_256Kb.gb");
}

#[test]
fn ram_64_kb() {
    common::run_test_till_ed("tests/mooneye/emulator-only/mbc1/ram_64Kb.gb");
}

#[test]
fn ram_16_mb() {
    common::run_test_till_ed("tests/mooneye/emulator-only/mbc1/rom_16Mb.gb");
}

#[test]
fn ram_1_mb() {
    common::run_test_till_ed("tests/mooneye/emulator-only/mbc1/rom_1Mb.gb");
}

#[test]
fn ram_2_mb() {
    common::run_test_till_ed("tests/mooneye/emulator-only/mbc1/rom_2Mb.gb");
}

#[test]
fn ram_4_mb() {
    common::run_test_till_ed("tests/mooneye/emulator-only/mbc1/rom_4Mb.gb");
}

#[test]
fn ram_512_kb() {
    common::run_test_till_ed("tests/mooneye/emulator-only/mbc1/rom_512Kb.gb");
}

#[test]
fn ram_8_mb() {
    common::run_test_till_ed("tests/mooneye/emulator-only/mbc1/rom_8Mb.gb");
}
