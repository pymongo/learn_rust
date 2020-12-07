build:
	rustc -o ../target/librust.so lib.rs --crate-type=dylib
#.ONESHELL:
run: build
	gcc -o ../target/executable main.c ../target/librust.so
	../target/executable
