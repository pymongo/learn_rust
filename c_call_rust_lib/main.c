#include <stdio.h>
void p();
void hello(const char *str);
char *repeat_hi(int times);
void free_cstr(const char *str);

/*
static  link:
gcc main.c ../target/debug/libc_call_rust_lib.a
dynamic link:
gcc main.c -Isrc -L ../target/debug/ -lc_call_rust_lib
*/
int main() {
    p();
    hello("rust");
    char* str = repeat_hi(3);
    printf("%s", str);
    free_cstr(str);
    return 0;
}