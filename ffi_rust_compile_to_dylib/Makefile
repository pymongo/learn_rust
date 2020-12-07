build:
    rustc -o ../librust.so lib.rs --crate-type=dylib
run: build
    gcc -o ../target/executable main.rs ../target/librust.so
    ldd ../executable
    ../executable
