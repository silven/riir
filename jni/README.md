# README

## Purpose

Java calls its FFI "JNI" and offers a C-based interface. JNI-RS offers Rust bindings around this.

## Build script

Our build script compiles our java code and generates a header using the ```javah``` tool. This is actually not needed, but nice to have.

## Running

```
CLASSPATH=java java -Djava.library.path="`pwd`/target/debug" main.HelloWorld
```
