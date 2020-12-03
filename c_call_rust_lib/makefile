run:
	cargo build
	gcc -o ../target/c_call_rust_lib main.c ../target/debug/libc_call_rust_lib.a
	../target/c_call_rust_lib
