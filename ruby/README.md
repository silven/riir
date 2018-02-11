# README

## Purpose

This example demonstates how to load rust code into ruby using the ruby ffi gem.

## Release mode

Because this example code relices in TCO to not blow up the stack, we must compile in release mode
```
cargo build --release
```

## Ruby FFI

The ruby FFI was installed via apt
```
sudo apt install ruby-ffi
```