# README

## Purpose
 
This calls Haskell from Rust

## Build Script

This build script calls Cabal to build our haskell library, it then uses the generated header to generate rust bindings.

## Usage

```
cabal install curryrs
cabal configure --enable-shared

LD_LIBRARY_PATH="/usr/lib/ghc/rts:dist:${LD_LIBRARY_PATH}" cargo run
```