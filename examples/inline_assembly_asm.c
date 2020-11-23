#include <stdio.h>

int main() {
    const u_int32_t input = 1;
    u_int32_t output;

    // 别名是__asm__()函数
    asm ("rol $1, %0" // 根据gcc官网的例子，最后一个汇编指令的输出寄存器%1要写成$1
    : "=r" (output) // 看来Rust的asm!宏的参数顺序是先output后in也是参考了gcc inline asm的顺序，或者说Rust的asm!就是gcc asm函数的wrapper/binding?
    : "r" (input));

    printf("1u32.rotate_left(1) = %d\n", output);
    return 0;
}
