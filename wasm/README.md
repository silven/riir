# README

## Purpose

Demonstate how the result of the wasm target can be loaded and ran by Nodejs. We use cargo-web and stdweb to handle generating lots of glue code and conversion methods for us.

## Running

```
rustup target add wasm32-unknown-unknown

cargo install -f cargo-web

cargo-web build --target=wasm32-unknown-unknown --release

nvm run 9 index.js
```