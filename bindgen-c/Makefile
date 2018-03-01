cbuild/main.elf:  cbuild/main.o target/debug/libusing_bindgen.so
	gcc -o $@ $^ 

target/debug/libusing_bindgen.so: src/lib.rs Cargo.toml
	cargo build

-include target/debug/libusing_bindgen.d

cbuild/main.o: src/main.c
	gcc -c $^ -o $@