void p();
void hello(const char *str);

/*
static  link: gcc main.c ../target/debug/libc_call_rust_lib.a
dynamic link: gcc main.c -Isrc -L ../target/debug/ -lc_call_rust_lib
*/
int main() {
    p();
    hello("rust");
    return 0;
}