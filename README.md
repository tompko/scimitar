# Scimitar

A gameboy emulator in Rust.

[![Build Status](https://travis-ci.org/tompko/scimitar.svg?branch=master)](https://travis-ci.org/tompko/scimitar)

## Blargg's tests

| Test         | Result|
|--------------|-------|
| cpu_instr    | :+1:  |
| dmg_sound_2  | :x:   |
| instr_timing | :+1:  |
| mem_timing_2 | :x:   |
| oam_bug_2    | :x:   |

## Mooneye
Tested using master at f72ba415e1.

### Mooneye GB acceptance tests

| Test                    | Result |
| ----------------------- | -------|
| add sp e timing         | :+1:   |
| boot hwio dmg0          | :x:    |
| boot hwio dmgABCXmgb    | :x:    |
| boot hwio S             | :x:    |
| boot regs dmg0          | :+1:   |
| boot regs dmgABCX       | :+1:   |
| boot regs mgb           | :+1:   |
| boot regs sgb2          | :+1:   |
| boot regs sgb           | :+1:   |
| call cc timing2         | :x:    |
| call cc timing          | :x:    |
| call timing2            | :x:    |
| call timing             | :x:    |
| di timing GS            | :x:    |
| div timing              | :+1:   |
| ei timing               | :x:    |
| halt ime0 ei            | :x:    |
| halt ime0 nointr timing | :x:    |
| halt ime1 timing2 GS    | :x:    |
| halt ime1 timing        | :x:    |
| if ie registers         | :x:    |
| intr timing             | :x:    |
| jp cc timing            | :x:    |
| jp timing               | :x:    |
| ld hl sp e timing       | :x:    |
| oam dma restart         | :x:    |
| oam dma start           | :x:    |
| oam dma timing          | :+1:   |
| pop timing              | :x:    |
| push timing             | :x:    |
| rapid di ei             | :x:    |
| ret cc timing           | :x:    |
| reti intr timing        | :x:    |
| reti timing             | :x:    |
| ret timing              | :x:    |
| rst timing              | :x:    |

### Mooneye Bits (unusable bits in memory and registers)

| Test           | Result |
| -------------- | -------|
| mem oam        | :+1:   |
| reg f          | :+1:   |
| unused hwio GS | :+1:   |


### Mooneye GPU

| Test                        | Result |
| --------------------------- | -------|
| hblank ly scx timing GS     | :x:    |
| intr 1 2 timing GS          | :x:    |
| intr 2 0 timing             | :x:    |
| intr 2 mode0 timing         | :x:    |
| intr 2 mode0 timing sprites | :x:    |
| intr 2 mode3 timing         | :x:    |
| intr 2 oam ok timing        | :x:    |
| stat irq blocking           | :x:    |
| vblank stat intr GS         | :x:    |

### Mooneye Timer

| Test                 | Result |
| -------------------- | -------|
| div write            | :+1:   |
| rapid toggle         | :x:    |
| tim00 div trigger    | :+1:   |
| tim00                | :+1:   |
| tim01 div trigger    | :+1:   |
| tim01                | :+1:   |
| tim10 div trigger    | :+1:   |
| tim10                | :+1:   |
| tim11 div trigger    | :+1:   |
| tim11                | :+1:   |
| tima reload          | :+1:   |
| tima write reloading | :x:    |
| tma write reloading  | :x:    |

### Emulator Only

#### MBC1

| Test              | Result |
| ----------------- | -------|
| rom 512Kb         | :+1:   |
| rom 1Mb           | :+1:   |
| rom 2Mb           | :+1:   |
| rom 4Mb           | :+1:   |
| rom 8Mb           | :+1:   |
| rom 16Mb          | :+1:   |
| ram 64Kb          | :+1:   |
| ram 256Kb         | :+1:   |
| multicart rom 8Mb | :x:    |

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
