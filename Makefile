ALL=basic-c bindgen-c haskell haskell2 jni python ruby wasm
SHELL=bash

.PHONY: all ${ALL}
all: ${ALL} 

basic-c:
	cd $@ && cargo run

bindgen-c:
	cd $@ && make && ./cbuild/main.elf

haskell:
	cd $@ && cargo build && cabal run

haskell2:
	cd $@ && LD_LIBRARY_PATH="/usr/lib/ghc/rts:dist:${LD_LIBRARY_PATH}" cargo run

jni:
	cd $@ && cargo build && CLASSPATH=java java -Djava.library.path="`pwd`/target/debug" main.HelloWorld

python:
	cd $@ && cargo build && python3 test.py

ruby:
	cd $@ && cargo build --release && ruby fib.rb

wasm:
	cd $@ && cargo-web build --target=wasm32-unknown-unknown --release && . ~/.nvm/nvm.sh && nvm run 9 index.js