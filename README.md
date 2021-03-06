# NES Rust Emulator
NES emulator written in Rust completely from scratch based only on publicly available documentation.

## Features / Task List
- [x] MOS Technology Accurate-ish (cycle accurate) 6502 CPU
  - [x] Register
  - [x] Interrupt
  - [x] Official opcode
  - [x] Unofficial opcode
- [x] Cassette(Mapper)
  - [x] NROM(0) 
  - [ ] UxROM (2) 
  - [ ] AxROM (7)
  - [ ] UNROM 5129 (30)
  - [ ] MMC1
  - [ ] MMC3
- [x] Custom Picture Processing Unit(PPU)
  - [x] OAM DMA
  - [x] BG
    - [x] Nametable Mirroring
    - [x] Scroll
      - [ ] Vertical Scroll Bug
  - [x] Sprite
    - [x] 8*8
    - [x] 8*16
    - [ ] PPU sprite overflow
- [x] PAD
  - [x] Microsoft controller
  - [x] Keyboard Input
- [ ] Custom Audio Processing Unit(APU)
  - [ ] Pulse Wave1
  - [ ] Pulse Wave2
  - [ ] Tri Wave
  - [ ] Noise
  - [ ] DMC
- [x] Emulation feature
    - [x] Snapshot
    - [x] Restore
    - [ ] Savestate support
    - [ ] ROM Selection Bootloader
    
## Dependencies
Install list for new users(todo: create makefile)
1. Assuming you have `rust` and `node`
2. You will need the following
```shell
brew install cmake
sudo apt install nodejs npm
sudo npm install -g n
sudo n 10.15.1
cargo install wasm-pack
```
