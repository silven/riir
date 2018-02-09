# README

## Purpose

This example demonstrates the most basic usage of compiling C code with the Rust build system, Cargo, and it's build script functionality. Cargo invokes the build script which, using the gcc crate, compiles our legacy code.

## Build script

Our legacy C code is compiled into liblegacy.so by the build script (build.rs) and rest of the build is handled by Cargo

## Cross compilation

Cargo also handles some basic cross compilation scenarios, this project can be cross compiled and run for aarch64 systems using qemu.

```
rustup target add aarch64-unknown-linux-gnu

cargo build --target=aarch64-unknown-linux-gnu

qemu-aarch64 -L /usr/aarch64-linux-gnu/ target/aarch64-unknown-linux-gnu/debug/basic-c
```