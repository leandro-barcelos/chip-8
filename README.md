# CHIP-8 Emulator

A CHIP-8 interpreter/emulator written in Rust, following the comprehensive guide by [Tobias V. Langhoff](https://tobiasvl.github.io/blog/write-a-chip-8-emulator).

## Features

- ✅ Complete CHIP-8 instruction set implementation
- ✅ Built-in hexadecimal font support
- ✅ Sound timer with beep functionality
- ✅ Configurable execution speed
- ✅ Cross-platform graphics and input handling
- 🔄 Configurable quirks for compatibility (in consts file at the moment)

## TODO

- Add SUPER-CHIP and XO-CHIP support
- Make it so drawing sprites to the display waits for the vertical blank interrupt (60 Hz)
- Make it easier to configure quirks

## Installation

### Prerequisites

- Rust 1.87+
- SDL2 development libraries (for graphics and audio)

### Building from Source

```bash
git clone https://github.com/leandro-barcelos/chip8-emulator
cd chip8-emulator
cargo build --release
```

## Usage

### Basic Usage

```
chip8-emulator <ROM_FILE> [<CYCLE_COUNT>]

Arguments:
  <ROM_FILE>     Path to the CHIP-8 ROM file
  <CYCLE_COUNT>  Maximum number of cycles to execute [optional]
```

### Keyboard Controls

The CHIP-8 keypad is mapped to your keyboard as follows:

```
CHIP-8 Keypad    Keyboard
1 2 3 C          1 2 3 4
4 5 6 D    -->   Q W E R
7 8 9 E          A S D F
A 0 B F          Z X C V
```

### Controls

- `Esc`: Exit emulator

## Testing

This emulator uses the comprehensive [CHIP-8 Test Suite by Timendus](https://github.com/Timendus/chip8-test-suite) to ensure accuracy and compatibility.

## Quirks

The emulator supports configurable quirks for different CHIP-8 variants:

- Memory operations increment behavior (`FX55`/`FX65`)
- Shift operations source register (`8XY6`/`8XYE`)
- Jump with offset instruction (`BNNN`)
- Flag register reset behavior (`8XY1-8XY3`)

## Resources

- **Development Guide**: [Write a CHIP-8 Emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) by Tobias V. Langhoff
- **Test Suite**: [CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite) by Timendus
- **Community**: [EmuDev Discord](https://discord.gg/dkmJAes) #chip-8 channel

---

*This emulator was built as a learning project to understand emulation concepts and low-level programming. It serves as an excellent introduction to emulator development and computer architecture.*
