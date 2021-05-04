# NES Rust Emulator
NES emulator written in Rust completely from scratch based only on publicly available documentation.

## Features / Task List
- [ ] MOS Technology Accurate-ish (cycle accurate) 6502 CPU
- [ ] Custom Picture Processing Unit(PPU)
- [ ] Custom Audio Processing Unit(APU)
- [ ] Writable memory(RAM) or read-only memory(ROM)
- [ ] Controller
- [ ] Cartridges with custom circuitry
- [ ] ROM Test cases
- [ ] transistor-level simulation of a real PPU and PPU testsuite automatically
- [ ] Support NROM (0), MMC1 (1), UxROM (2), AxROM (7) and UNROM 512 (30) mappers
- [ ] compile into libretro core
- [ ] Run from the web (WebAssembly)

## Issues
- [ ] PPU sprite overflow
- [ ] Savestate support
- [ ] PAL support
