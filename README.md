# Scimitar

A gameboy emulator in Rust.

[![Build Status](https://travis-ci.org/tompko/scimitar.svg?branch=master)](https://travis-ci.org/tompko/scimitar)

## Blargg's tests

| Test         | Result|
|--------------|-------|
| cpu instr    | :+1:  |
| dmg sound 2  | :x:   |
| instr timing | :+1:  |
| mem timing 2 | :+1:  |
| oam bug 2    | :x:   |

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
| call cc timing          | :+1:   |
| call cc timing2         | :+1:   |
| call timing             | :+1:   |
| call timing2            | :+1:   |
| di timing GS            | :+1:   |
| div timing              | :+1:   |
| ei timing               | :+1:   |
| halt ime0 ei            | :+1:   |
| halt ime0 nointr timing | :+1:   |
| halt ime1 timing2 GS    | :+1:   |
| halt ime1 timing        | :+1:   |
| if ie registers         | :+1:   |
| intr timing             | :+1:   |
| jp cc timing            | :+1:   |
| jp timing               | :+1:   |
| ld hl sp e timing       | :+1:   |
| oam dma restart         | :x:    |
| oam dma start           | :x:    |
| oam dma timing          | :+1:   |
| pop timing              | :+1:   |
| push timing             | :+1:   |
| rapid di ei             | :+1:   |
| ret cc timing           | :+1:   |
| reti intr timing        | :+1:   |
| reti timing             | :+1:   |
| ret timing              | :+1:   |
| rst timing              | :+1:   |

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
