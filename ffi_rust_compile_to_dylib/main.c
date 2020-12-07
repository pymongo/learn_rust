#include <stdio.h>

void hello(const char *str);
char *repeat_hi(int times);
void free_cstr(const char *str);
int sum_of_positive(int *nums, size_t len);

typedef struct {
    int x;
    int y;
} Point;
void print_point(Point point);

// C use opaque type(不透明数据类型) to bind Rust:Box<T>
typedef struct OpaqueType Map;
Map* map_new();
void map_insert(Map* map, int k, int v);
int map_get(Map* map, int k);
void map_free(Map* map);

/*
static link:
cargo build && gcc main.c ../target/debug/libc_call_rust_lib.a && ./a.out
dynamic link:
gcc main.c -Isrc -L ../target/debug/ -lc_call_rust_lib
*/
int main() {
    p();
    hello("rust");
    char* str = repeat_hi(3);
    printf("%s\n", str);
    free_cstr(str);
    int nums[3] = {1, -1, 3};
    int sum = sum_of_positive(nums, 3);
    printf("sum = %d\n", sum);
    Point point = {.x=1, .y=2};
    print_point(point);

    Map* map = map_new();
    map_insert(map, 1, 1);
    printf("map_get(map, 1) = %d\n", map_get(map, 1));
    printf("map_get(map, 2) = %d\n", map_get(map, 2));
    map_free(map);
    return 0;
}
