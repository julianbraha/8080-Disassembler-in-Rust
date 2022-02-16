# 8080-Disassembler-in-Rust
Disassembler for the Intel 8080 architecture, written in Rust.

![compilation badge](https://github.com/julianbraha/8080-Disassembler-in-Rust/actions/workflows/build.yml/badge.svg)

## How to compile:
`rustc disassemble.rs`

## How to run:
### Option 1:
- `./disassemble <filename>`, where `<filename>` is a hexdump of the 8080 binary.

-or-

### Option 2:
- `bash hex.sh <binary>`, where `<binary>` is an 8080 binary.
- `./disassemble /intermediates/<binary>.x`, where `/intermediates/<binary>.x` is the output from the previous step.
