# 8080-Disassembler-in-Rust
Disassembler for the Intel 8080 architecture, written in Rust.

![compilation badge](https://github.com/julianbraha/8080-Disassembler-in-Rust/actions/workflows/build.yml/badge.svg)

How to compile:
`rustc disassemble.rs`

How to run:
`disassemble <filename>`, where `<filename>` is the output of `od -x --endian big <binary>`
